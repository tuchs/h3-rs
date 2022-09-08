use enum_primitive::FromPrimitive;

use crate::{
    algos::{directionForNeighbor, h3NeighborRotations},
    constants::{H3_CELL_MODE, H3_DIRECTEDEDGE_MODE},
    coord_ijk::Direction,
    error::Error,
    h3_index::{
        isPentagon, isValidCell, H3Index, H3_GET_MODE, H3_GET_RESERVED_BITS, H3_SET_MODE,
        H3_SET_RESERVED_BITS,
    },
    H3_NULL,
};

/**
 * Returns a directed edge H3 index based on the provided origin and
 * destination
 * @param origin The origin H3 hexagon index
 * @param destination The destination H3 hexagon index
 * @return The directed edge H3Index, or H3_NULL on failure.
 */
pub fn cellsToDirectedEdge(origin: H3Index, destination: H3Index) -> Result<H3Index, Error> {
    // Determine the IJK direction from the origin to the destination
    let direction: Direction = directionForNeighbor(origin, destination);

    // The direction will be invalid if the cells are not neighbors
    if direction == Direction::InvalidDigit {
        return Err(Error::NotNeighbors);
    }

    // Create the edge index for the neighbor direction
    let mut output = origin;
    H3_SET_MODE(&mut output, H3_DIRECTEDEDGE_MODE);
    H3_SET_RESERVED_BITS(&mut output, direction as i32);

    return Ok(output);
}

/**
 * Returns the origin hexagon from the directed edge H3Index
 * @param edge The edge H3 index
 * @return The origin H3 hexagon index, or H3_NULL on failure
 */
pub fn getDirectedEdgeOrigin(edge: H3Index) -> Result<H3Index, Error> {
    if H3_GET_MODE(edge) != H3_DIRECTEDEDGE_MODE {
        return Err(Error::DirectedEdgeInvalid);
    }
    let mut origin: H3Index = edge;
    H3_SET_MODE(&mut origin, H3_CELL_MODE);
    H3_SET_RESERVED_BITS(&mut origin, 0);
    return Ok(origin);
}

/**
 * Returns the destination hexagon from the directed edge H3Index
 * @param edge The edge H3 index
 * @return The destination H3 hexagon index, or H3_NULL on failure
 */
pub fn getDirectedEdgeDestination(edge: H3Index) -> Result<H3Index, Error> {
    let direction: Direction = Direction::from_i32(H3_GET_RESERVED_BITS(edge)).unwrap();
    let mut rotations: i32 = 0;
    // Note: This call is also checking for H3_DIRECTEDEDGE_MODE
    let mut origin: H3Index = getDirectedEdgeOrigin(edge)?;
    return h3NeighborRotations(origin, direction, &mut rotations);
}

/**
 * Determines if the provided H3Index is a valid directed edge index
 * @param edge The directed edge H3Index
 * @return 1 if it is a directed edge H3Index, otherwise 0.
 */
pub fn isValidDirectedEdge(edge: H3Index) -> bool {
    let neighborDirection = H3_GET_RESERVED_BITS(edge);
    if neighborDirection <= Direction::CenterDigit as i32
        || neighborDirection >= Direction::NUM_DIGITS as i32
    {
        return false;
    }

    // Note: This call is also checking for H3_DIRECTEDEDGE_MODE
    let origin: H3Index = match getDirectedEdgeOrigin(edge) {
        Ok(result) => result,
        Err(err) => {
            return false;
        }
    };

    if isPentagon(origin) && neighborDirection == Direction::KAxesDigit as i32 {
        return false;
    }

    return isValidCell(origin);
}

/**
 * Returns the origin, destination pair of hexagon IDs for the given edge ID
 * @param edge The directed edge H3Index
 * @param originDestination Pointer to memory to store origin and destination
 * IDs
 */
pub fn directedEdgeToCells(edge: H3Index) -> Result<(H3Index, H3Index), Error> {
    let originResult = getDirectedEdgeOrigin(edge)?;
    let destinationResult = getDirectedEdgeDestination(edge)?;
    return Ok((originResult, destinationResult));
}

/**
 * Provides all of the directed edges from the current H3Index.
 * @param origin The origin hexagon H3Index to find edges for.
 * @param edges The memory to store all of the edges inside.
 */
pub fn originToDirectedEdges(origin: H3Index) -> [H3Index; 6] {
    let mut edges = [0; 6];
    // Determine if the origin is a pentagon and special treatment needed.
    let isPent = isPentagon(origin);

    // This is actually quite simple. Just modify the bits of the origin
    // slightly for each direction, except the 'k' direction in pentagons,
    // which is zeroed.
    for i in 0..6 {
        if isPent && i == 0 {
            edges[i] = H3_NULL;
        } else {
            edges[i] = origin;
            H3_SET_MODE(&mut edges[i], H3_DIRECTEDEDGE_MODE);
            H3_SET_RESERVED_BITS(&mut edges[i], (i as i32) + 1);
        }
    }
    return edges;
}

#[cfg(test)]
mod tests {
    use crate::{
        algos::gridRingUnsafe,
        h3_index::{latLngToCell, setH3Index},
        lat_lng::LatLng,
    };

    use super::*;

    static sfGeo: LatLng = LatLng {
        lat: 0.659966917655,
        lng: -2.1364398519396,
    };

    #[test]
    fn cellsToDirectedEdgeAndFriends() {
        let mut sf: H3Index = latLngToCell(&sfGeo, 9).unwrap();
        let ring = gridRingUnsafe(sf, 1).unwrap();
        let mut sf2 = ring[0];

        let mut edge = cellsToDirectedEdge(sf, sf2).unwrap();
        let mut edgeOrigin: H3Index = getDirectedEdgeOrigin(edge).unwrap();
        assert_eq!(sf, edgeOrigin, "can retrieve the origin from the edge");
        let mut edgeDestination: H3Index = getDirectedEdgeDestination(edge).unwrap();
        assert_eq!(
            sf2, edgeDestination,
            "can retrieve the destination from the edge"
        );

        let originDestination = directedEdgeToCells(edge).unwrap();

        assert_eq!(
            originDestination.0, sf,
            "got the origin first in the pair request"
        );
        assert_eq!(
            originDestination.1, sf2,
            "got the destination last in the pair request"
        );

        assert_eq!(
            directedEdgeToCells(0),
            Err(Error::DirectedEdgeInvalid),
            "directedEdgeToCells fails for invalid edges"
        );

        let mut invalidEdge = 0;
        setH3Index(&mut invalidEdge, 1, 4, 0);
        H3_SET_RESERVED_BITS(&mut invalidEdge, Direction::InvalidDigit as i32);
        H3_SET_MODE(&mut invalidEdge, H3_DIRECTEDEDGE_MODE);
        assert!(
            directedEdgeToCells(invalidEdge).is_err(),
            "directedEdgeToCells fails for invalid edges"
        );

        let largerRing = gridRingUnsafe(sf, 2).unwrap();
        let sf3 = largerRing[0];

        assert_eq!(
            cellsToDirectedEdge(sf, sf3),
            Err(Error::NotNeighbors),
            "Non-neighbors can't have edges"
        );
    }

    #[test]
    fn originToDirectedEdges() {
        let sf = latLngToCell(&sfGeo, 9).unwrap();
        let edges = super::originToDirectedEdges(sf);

        for i in 0..6 {
            assert!(isValidDirectedEdge(edges[i]), "edge is an edge");
            let origin = getDirectedEdgeOrigin(edges[i]).unwrap();
            assert!(sf == origin, "origin is correct");
            let destination = getDirectedEdgeDestination(edges[i]).unwrap();
            assert!(sf != destination, "destination is not origin");
        }
    }
}
