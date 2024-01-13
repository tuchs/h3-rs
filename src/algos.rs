use enum_primitive::FromPrimitive;

use crate::{
    base_cells::{
        _baseCellIsCwOffset, _isBaseCellPentagon, _isBaseCellPolarPentagon, baseCellData,
        baseCellNeighbor60CCWRots, baseCellNeighbors, INVALID_BASE_CELL,
    },
    constants::NUM_BASE_CELLS,
    coord_ijk::{Direction, _rotate60ccw},
    error::Error,
    h3_index::{
        H3Index, _h3LeadingNonZeroDigit, _h3Rotate60ccw, _h3Rotate60cw, _h3RotatePent60ccw,
        isPentagon, isResolutionClassIII, H3_GET_BASE_CELL, H3_GET_INDEX_DIGIT, H3_GET_RESOLUTION,
        H3_SET_BASE_CELL, H3_SET_INDEX_DIGIT,
    },
};

/**
 * Directions used for traversing a hexagonal ring counterclockwise around
 * {1, 0, 0}
 *
 * <pre>
 *      _
 *    _/ \\_
 *   / \\5/ \\
 *   \\0/ \\4/
 *   / \\_/ \\
 *   \\1/ \\3/
 *     \\2/
 * </pre>
 */
const DIRECTIONS: [Direction; 6] = [
    Direction::JAxesDigit,
    Direction::JKAxesDigit,
    Direction::KAxesDigit,
    Direction::IKAxesDigit,
    Direction::IAxesDigit,
    Direction::IJAxesDigit,
];

/**
 * Direction used for traversing to the next outward hexagonal ring.
 */
const NEXT_RING_DIRECTION: Direction = Direction::IAxesDigit;

/**
 * New digit when traversing along class II grids.
 *
 * Current digit -> direction -> new digit.
 */
const NEW_DIGIT_II: [[Direction; 7]; 7] = [
    [
        Direction::CenterDigit,
        Direction::KAxesDigit,
        Direction::JAxesDigit,
        Direction::JKAxesDigit,
        Direction::IAxesDigit,
        Direction::IKAxesDigit,
        Direction::IJAxesDigit,
    ],
    [
        Direction::KAxesDigit,
        Direction::IAxesDigit,
        Direction::JKAxesDigit,
        Direction::IJAxesDigit,
        Direction::IKAxesDigit,
        Direction::JAxesDigit,
        Direction::CenterDigit,
    ],
    [
        Direction::JAxesDigit,
        Direction::JKAxesDigit,
        Direction::KAxesDigit,
        Direction::IAxesDigit,
        Direction::IJAxesDigit,
        Direction::CenterDigit,
        Direction::IKAxesDigit,
    ],
    [
        Direction::JKAxesDigit,
        Direction::IJAxesDigit,
        Direction::IAxesDigit,
        Direction::IKAxesDigit,
        Direction::CenterDigit,
        Direction::KAxesDigit,
        Direction::JAxesDigit,
    ],
    [
        Direction::IAxesDigit,
        Direction::IKAxesDigit,
        Direction::IJAxesDigit,
        Direction::CenterDigit,
        Direction::JAxesDigit,
        Direction::JKAxesDigit,
        Direction::KAxesDigit,
    ],
    [
        Direction::IKAxesDigit,
        Direction::JAxesDigit,
        Direction::CenterDigit,
        Direction::KAxesDigit,
        Direction::JKAxesDigit,
        Direction::IJAxesDigit,
        Direction::IAxesDigit,
    ],
    [
        Direction::IJAxesDigit,
        Direction::CenterDigit,
        Direction::IKAxesDigit,
        Direction::JAxesDigit,
        Direction::KAxesDigit,
        Direction::IAxesDigit,
        Direction::JKAxesDigit,
    ],
];

/**
 * New traversal direction when traversing along class II grids.
 *
 * Current digit -> direction -> new ap7 move (at coarser level).
 */
const NEW_ADJUSTMENT_II: [[Direction; 7]; 7] = [
    [
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
    ],
    [
        Direction::CenterDigit,
        Direction::KAxesDigit,
        Direction::CenterDigit,
        Direction::KAxesDigit,
        Direction::CenterDigit,
        Direction::IKAxesDigit,
        Direction::CenterDigit,
    ],
    [
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::JAxesDigit,
        Direction::JKAxesDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::JAxesDigit,
    ],
    [
        Direction::CenterDigit,
        Direction::KAxesDigit,
        Direction::JKAxesDigit,
        Direction::JKAxesDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
    ],
    [
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::IAxesDigit,
        Direction::IAxesDigit,
        Direction::IJAxesDigit,
    ],
    [
        Direction::CenterDigit,
        Direction::IKAxesDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::IAxesDigit,
        Direction::IKAxesDigit,
        Direction::CenterDigit,
    ],
    [
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::JAxesDigit,
        Direction::CenterDigit,
        Direction::IJAxesDigit,
        Direction::CenterDigit,
        Direction::IJAxesDigit,
    ],
];

/**
 * New traversal direction when traversing along class III grids.
 *
 * Current digit -> direction -> new ap7 move (at coarser level).
 */
const NEW_DIGIT_III: [[Direction; 7]; 7] = [
    [
        Direction::CenterDigit,
        Direction::KAxesDigit,
        Direction::JAxesDigit,
        Direction::JKAxesDigit,
        Direction::IAxesDigit,
        Direction::IKAxesDigit,
        Direction::IJAxesDigit,
    ],
    [
        Direction::KAxesDigit,
        Direction::JAxesDigit,
        Direction::JKAxesDigit,
        Direction::IAxesDigit,
        Direction::IKAxesDigit,
        Direction::IJAxesDigit,
        Direction::CenterDigit,
    ],
    [
        Direction::JAxesDigit,
        Direction::JKAxesDigit,
        Direction::IAxesDigit,
        Direction::IKAxesDigit,
        Direction::IJAxesDigit,
        Direction::CenterDigit,
        Direction::KAxesDigit,
    ],
    [
        Direction::JKAxesDigit,
        Direction::IAxesDigit,
        Direction::IKAxesDigit,
        Direction::IJAxesDigit,
        Direction::CenterDigit,
        Direction::KAxesDigit,
        Direction::JAxesDigit,
    ],
    [
        Direction::IAxesDigit,
        Direction::IKAxesDigit,
        Direction::IJAxesDigit,
        Direction::CenterDigit,
        Direction::KAxesDigit,
        Direction::JAxesDigit,
        Direction::JKAxesDigit,
    ],
    [
        Direction::IKAxesDigit,
        Direction::IJAxesDigit,
        Direction::CenterDigit,
        Direction::KAxesDigit,
        Direction::JAxesDigit,
        Direction::JKAxesDigit,
        Direction::IAxesDigit,
    ],
    [
        Direction::IJAxesDigit,
        Direction::CenterDigit,
        Direction::KAxesDigit,
        Direction::JAxesDigit,
        Direction::JKAxesDigit,
        Direction::IAxesDigit,
        Direction::IKAxesDigit,
    ],
];

/**
 * New traversal direction when traversing along class III grids.
 *
 * Current digit -> direction -> new ap7 move (at coarser level).
 */
const NEW_ADJUSTMENT_III: [[Direction; 7]; 7] = [
    [
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
    ],
    [
        Direction::CenterDigit,
        Direction::KAxesDigit,
        Direction::CenterDigit,
        Direction::JKAxesDigit,
        Direction::CenterDigit,
        Direction::KAxesDigit,
        Direction::CenterDigit,
    ],
    [
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::JAxesDigit,
        Direction::JAxesDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::IJAxesDigit,
    ],
    [
        Direction::CenterDigit,
        Direction::JKAxesDigit,
        Direction::JAxesDigit,
        Direction::JKAxesDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
    ],
    [
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::IAxesDigit,
        Direction::IKAxesDigit,
        Direction::IAxesDigit,
    ],
    [
        Direction::CenterDigit,
        Direction::KAxesDigit,
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::IKAxesDigit,
        Direction::IKAxesDigit,
        Direction::CenterDigit,
    ],
    [
        Direction::CenterDigit,
        Direction::CenterDigit,
        Direction::IJAxesDigit,
        Direction::CenterDigit,
        Direction::IAxesDigit,
        Direction::CenterDigit,
        Direction::IJAxesDigit,
    ],
];

/**
 * Maximum number of cells that result from the gridDisk algorithm with the
 * given k. Formula source and proof: https://oeis.org/A003215
 *
 * @param   k   k value, k >= 0.
 * @param out   size in indexes
 */
pub fn maxGridDiskSize(k: u32) -> usize {
    return (3 * k as i64 * (k as i64 + 1) + 1) as usize;
}

/**
 * Produce cells and their distances from the given origin cell, up to
 * distance k.
 *
 * k-ring 0 is defined as the origin cell, k-ring 1 is defined as k-ring 0 and
 * all neighboring cells, and so on.
 *
 * Output is placed in the provided array in no particular order. Elements of
 * the output array may be left zero, as can happen when crossing a pentagon.
 *
 * @param  origin      origin cell
 * @param  k           k >= 0
 * @param  out         zero-filled array which must be of size
 * maxGridDiskSize(k)
 * @param  distances   NULL or a zero-filled array which must be of size
 *                     maxGridDiskSize(k)
 */
pub fn gridDiskDistances(origin: H3Index, k: u32) -> Result<Vec<(H3Index, u32)>, Error> {
    // Optimistically try the faster gridDiskUnsafe algorithm first
    match gridDiskDistancesUnsafe(origin, k) {
        Ok(out) => return Ok(out),
        Err(e) => e,
    };

    let maxIdx = maxGridDiskSize(k);

    // Fast algo failed, fall back to slower, correct algo
    let mut out: Vec<(H3Index, u32)> = Vec::new();
    out.resize(maxIdx, (0, 0));

    _gridDiskDistancesInternal(origin, k, &mut out, maxIdx, 0)?;
    Ok(out
        .into_iter()
        .filter(|(h3index, _distance)| *h3index != 0)
        .collect())
}

/**
 * Internal algorithm for the safe but slow version of gridDiskDistances
 *
 * Adds the origin cell to the output set (treating it as a hash set)
 * and recurses to its neighbors, if needed.
 *
 * @param  origin      Origin cell
 * @param  k           Maximum distance to move from the origin
 * @param  out         Array treated as a hash set, elements being either
 *                     H3Index or 0.
 * @param  distances   Scratch area, with elements paralleling the out array.
 *                     Elements indicate ijk distance from the origin cell to
 *                     the output cell
 * @param  maxIdx      Size of out and scratch arrays (must be
 * maxGridDiskSize(k))
 * @param  curK        Current distance from the origin
 */
fn _gridDiskDistancesInternal(
    mut origin: H3Index,
    k: u32,
    out: &mut Vec<(H3Index, u32)>,
    maxIdx: usize,
    curK: u32,
) -> Result<(), Error> {
    // Put origin in the output array. out is used as a hash set.
    let mut off: usize = (origin % maxIdx as u64) as usize;
    while out[off].0 != 0 && out[off].0 != origin {
        off = ((off + 1) % maxIdx) as usize;
    }

    // We either got a free slot in the hash set or hit a duplicate
    // We might need to process the duplicate anyways because we got
    // here on a longer path before.
    if out[off].0 == origin && out[off].1 <= curK {
        return Ok(());
    }

    out[off].0 = origin;
    out[off].1 = curK;

    // Base case: reached an index k away from the origin.
    if curK >= k {
        return Ok(());
    }

    // Recurse to all neighbors in no particular order.
    for i in 0..6 {
        let mut rotations: i32 = 0;
        match h3NeighborRotations(origin, DIRECTIONS[i], &mut rotations) {
            Ok(result) => {
                _gridDiskDistancesInternal(result, k, out, maxIdx, curK + 1)?;
            }
            Err(e) => {
                if e != Error::Pentagon {
                    return Err(e);
                }
            }
        };
    }
    return Ok(());
}

/**
 * Returns the hexagon index neighboring the origin, in the direction dir.
 *
 * Implementation note: The only reachable case where this returns 0 is if the
 * origin is a pentagon and the translation is in the k direction. Thus,
 * 0 can only be returned if origin is a pentagon.
 *
 * @param origin Origin index
 * @param dir Direction to move in
 * @param rotations Number of ccw rotations to perform to reorient the
 *                  translation vector. Will be modified to the new number of
 *                  rotations to perform (such as when crossing a face edge.)
 * @return H3Index of the specified neighbor or 0 if deleted k-subsequence
 *         distortion is encountered.
 */
pub fn h3NeighborRotations(
    origin: H3Index,
    mut dir: Direction,
    rotations: &mut i32,
) -> Result<H3Index, Error> {
    let mut current: H3Index = origin;

    if dir < Direction::CenterDigit || dir >= Direction::InvalidDigit {
        return Err(Error::Failed);
    }
    for _i in 0..*rotations {
        dir = _rotate60ccw(dir);
    }

    let mut newRotations: i32 = 0;
    let oldBaseCell: i32 = H3_GET_BASE_CELL(current);
    if oldBaseCell < 0 || oldBaseCell >= NUM_BASE_CELLS {
        // LCOV_EXCL_BR_LINE
        // Base cells less than zero can not be represented in an index
        return Err(Error::CellInvalid);
    }
    let oldLeadingDigit: Direction = _h3LeadingNonZeroDigit(current);

    // Adjust the indexing digits and, if needed, the base cell.
    let mut r: i32 = H3_GET_RESOLUTION(current) - 1;
    loop {
        if r == -1 {
            H3_SET_BASE_CELL(
                &mut current,
                baseCellNeighbors[oldBaseCell as usize][dir as usize],
            );
            newRotations = baseCellNeighbor60CCWRots[oldBaseCell as usize][dir as usize];

            if H3_GET_BASE_CELL(current) == INVALID_BASE_CELL {
                // Adjust for the deleted k vertex at the base cell level.
                // This edge actually borders a different neighbor.
                H3_SET_BASE_CELL(
                    &mut current,
                    baseCellNeighbors[oldBaseCell as usize][Direction::IKAxesDigit as usize],
                );
                newRotations = baseCellNeighbor60CCWRots[oldBaseCell as usize]
                    [Direction::IKAxesDigit as usize];

                // perform the adjustment for the k-subsequence we're skipping
                // over.
                current = _h3Rotate60ccw(current);
                *rotations = *rotations + 1;
            }
            break;
        } else {
            let oldDigit: Direction = H3_GET_INDEX_DIGIT(current, r + 1);
            let nextDir: Direction;
            if oldDigit == Direction::InvalidDigit {
                // Only possible on invalid input
                return Err(Error::CellInvalid);
            } else if isResolutionClassIII(r + 1) {
                H3_SET_INDEX_DIGIT(
                    &mut current,
                    r + 1,
                    NEW_DIGIT_II[oldDigit as usize][dir as usize] as i32,
                );
                nextDir = NEW_ADJUSTMENT_II[oldDigit as usize][dir as usize];
            } else {
                H3_SET_INDEX_DIGIT(
                    &mut current,
                    r + 1,
                    NEW_DIGIT_III[oldDigit as usize][dir as usize] as i32,
                );
                nextDir = NEW_ADJUSTMENT_III[oldDigit as usize][dir as usize];
            }

            if nextDir != Direction::CenterDigit {
                dir = nextDir;
                r -= 1;
            } else {
                // No more adjustment to perform
                break;
            }
        }
    }

    let newBaseCell: i32 = H3_GET_BASE_CELL(current);
    if _isBaseCellPentagon(newBaseCell) {
        let mut alreadyAdjustedKSubsequence: i32 = 0;

        // force rotation current of missing k-axes sub-sequence
        if _h3LeadingNonZeroDigit(current) == Direction::KAxesDigit {
            if oldBaseCell != newBaseCell {
                // in this case, we traversed into the deleted
                // k subsequence of a pentagon base cell.
                // We need to rotate current of that case depending
                // on how we got here.
                // check for a cw/ccw offset face; default is ccw

                if _baseCellIsCwOffset(
                    newBaseCell,
                    baseCellData[oldBaseCell as usize].homeFijk.face,
                ) {
                    current = _h3Rotate60cw(current);
                } else {
                    // See cwOffsetPent in testKRing.c for why this is
                    // unreachable.
                    current = _h3Rotate60ccw(current); // LCOV_EXCL_LINE
                }
                alreadyAdjustedKSubsequence = 1;
            } else {
                // In this case, we traversed into the deleted
                // k subsequence from within the same pentagon
                // base cell.
                if oldLeadingDigit == Direction::CenterDigit {
                    // Undefined: the k direction is deleted from here
                    return Err(Error::Pentagon);
                } else if oldLeadingDigit == Direction::JKAxesDigit {
                    // Rotate current of the deleted k subsequence
                    // We also need an additional change to the direction we're
                    // moving in
                    current = _h3Rotate60ccw(current);
                    *rotations = *rotations + 1;
                } else if oldLeadingDigit == Direction::IKAxesDigit {
                    // Rotate current of the deleted k subsequence
                    // We also need an additional change to the direction we're
                    // moving in
                    current = _h3Rotate60cw(current);
                    *rotations = *rotations + 5;
                } else {
                    // Should never occur
                    return Err(Error::Failed); // LCOV_EXCL_LINE
                }
            }
        }

        for _i in 0..newRotations {
            current = _h3RotatePent60ccw(current);
        }

        // Account for differing orientation of the base cells (this edge
        // might not follow properties of some other edges.)
        if oldBaseCell != newBaseCell {
            if _isBaseCellPolarPentagon(newBaseCell) {
                // 'polar' base cells behave differently because they have all
                // i neighbors.
                if oldBaseCell != 118
                    && oldBaseCell != 8
                    && _h3LeadingNonZeroDigit(current) != Direction::JKAxesDigit
                {
                    *rotations = *rotations + 1;
                }
            } else if _h3LeadingNonZeroDigit(current) == Direction::IKAxesDigit
                && alreadyAdjustedKSubsequence == 0
            {
                // account for distortion introduced to the 5 neighbor by the
                // deleted k subsequence.
                *rotations = *rotations + 1;
            }
        }
    } else {
        for _i in 0..newRotations {
            current = _h3Rotate60ccw(current);
        }
    }

    *rotations = (*rotations + newRotations) % 6;

    return Ok(current);
}

/**
 * Get the direction from the origin to a given neighbor. This is effectively
 * the reverse operation for h3NeighborRotations. Returns INVALID_DIGIT if the
 * cells are not neighbors.
 *
 * TODO: This is currently a brute-force algorithm, but as it's O(6) that's
 * probably acceptable.
 */
pub fn directionForNeighbor(origin: H3Index, destination: H3Index) -> Direction {
    let isPent: bool = isPentagon(origin);
    // Checks each neighbor, in order, to determine which direction the
    // destination neighbor is located. Skips CENTER_DIGIT since that
    // would be the origin; skips deleted K direction for pentagons.
    for direction in (if isPent {
        Direction::JAxesDigit as usize
    } else {
        Direction::KAxesDigit as usize
    })..(Direction::InvalidDigit as usize)
    {
        let mut rotations: i32 = 0;
        match h3NeighborRotations(
            origin,
            Direction::from_usize(direction).unwrap(),
            &mut rotations,
        ) {
            Ok(neighbor) => {
                if neighbor == destination {
                    return Direction::from_usize(direction).unwrap();
                }
            }
            Err(err) => {}
        }
    }
    return Direction::InvalidDigit;
}

/**
 * gridDiskDistancesUnsafe produces indexes within k distance of the origin
 * index. Output behavior is undefined when one of the indexes returned by this
 * function is a pentagon or is in the pentagon distortion area.
 *
 * k-ring 0 is defined as the origin index, k-ring 1 is defined as k-ring 0 and
 * all neighboring indexes, and so on.
 *
 * Output is placed in the provided array in order of increasing distance from
 * the origin. The distances in hexagons is placed in the distances array at
 * the same offset.
 *
 * @param origin Origin location.
 * @param k k >= 0
 * @param out Array which must be of size maxGridDiskSize(k).
 * @param distances Null or array which must be of size maxGridDiskSize(k).
 * @return 0 if no pentagon or pentagonal distortion area was encountered.
 */
pub fn gridDiskDistancesUnsafe(
    mut origin: H3Index,
    k: u32,
    //out: &mut Vec<H3Index>,
    //distances: &mut Vec<i32>,
) -> Result<Vec<(H3Index, u32)>, Error> {
    // Return codes:
    // 1 Pentagon was encountered
    // 2 Pentagon distortion (deleted k subsequence) was encountered
    // Pentagon being encountered is not itself a problem; really the deleted
    // k-subsequence is the problem, but for compatibility reasons we fail on
    // the pentagon.
    if k < 0 {
        return Err(Error::Domain);
    }

    let mut out: Vec<(H3Index, u32)> = Vec::new();

    // k must be >= 0, so origin is always needed
    out.push((origin, 0));

    if isPentagon(origin) {
        // Pentagon was encountered; bail out as user doesn't want this.
        return Err(Error::Pentagon);
    }

    // 0 < ring <= k, current ring
    let mut ring: u32 = 1;
    // 0 <= direction < 6, current side of the ring
    let mut direction: i32 = 0;
    // 0 <= i < ring, current position on the side of the ring
    let mut i: u32 = 0;
    // Number of 60 degree ccw rotations to perform on the direction (based on
    // which faces have been crossed.)
    let mut rotations: i32 = 0;

    while ring <= k {
        if direction == 0 && i == 0 {
            // Not putting in the output set as it will be done later, at
            // the end of this ring.
            origin = h3NeighborRotations(origin, NEXT_RING_DIRECTION, &mut rotations)?;

            if isPentagon(origin) {
                // Pentagon was encountered; bail out as user doesn't want this.
                return Err(Error::Pentagon);
            }
        }

        origin = h3NeighborRotations(origin, DIRECTIONS[direction as usize], &mut rotations)?;
        out.push((origin, ring));
        //distances.push(ring);
        //idx += 1;

        i += 1;
        // Check if end of this side of the k-ring
        if i == ring {
            i = 0;
            direction += 1;
            // Check if end of this ring.
            if direction == 6 {
                direction = 0;
                ring += 1;
            }
        }

        if isPentagon(origin) {
            // Pentagon was encountered; bail out as user doesn't want this.
            return Err(Error::Pentagon);
        }
    }
    return Ok(out);
}

/**
 * Returns the "hollow" ring of hexagons at exactly grid distance k from
 * the origin hexagon. In particular, k=0 returns just the origin hexagon.
 *
 * A nonzero failure code may be returned in some cases, for example,
 * if a pentagon is encountered.
 * Failure cases may be fixed in future versions.
 *
 * @param origin Origin location.
 * @param k k >= 0
 * @param out Array which must be of size 6 * k (or 1 if k == 0)
 * @return 0 if successful; nonzero otherwise.
 */
pub fn gridRingUnsafe(mut origin: H3Index, k: u32) -> Result<Vec<H3Index>, Error> {
    let mut out = Vec::<H3Index>::new();
    out.reserve_exact(6 * k as usize);

    // Short-circuit on 'identity' ring
    if k == 0 {
        out.push(origin);
        return Ok(out);
    }

    // Number of 60 degree ccw rotations to perform on the direction (based on
    // which faces have been crossed.)
    let mut rotations: i32 = 0;
    // Scratch structure for checking for pentagons
    if isPentagon(origin) {
        // Pentagon was encountered; bail out as user doesn't want this.
        return Err(Error::Pentagon);
    }

    for ring in 0..k {
        origin = h3NeighborRotations(origin, NEXT_RING_DIRECTION, &mut rotations)?;

        if isPentagon(origin) {
            return Err(Error::Pentagon);
        }
    }

    let lastIndex = origin;

    out.push(origin);

    for direction in 0..6 {
        for pos in 0..k {
            origin = h3NeighborRotations(origin, DIRECTIONS[direction], &mut rotations)?;

            // Skip the very last index, it was already added. We do
            // however need to traverse to it because of the pentagonal
            // distortion check, below.
            if pos != k - 1 || direction != 5 {
                out.push(origin);

                if isPentagon(origin) {
                    return Err(Error::Pentagon);
                }
            }
        }
    }

    // Check that this matches the expected lastIndex, if it doesn't,
    // it indicates pentagonal distortion occurred and we should report
    // failure.
    if lastIndex != origin {
        return Err(Error::Pentagon);
    } else {
        return Ok(out);
    }
}

#[cfg(test)]
mod tests {
    use num::Float;

    use crate::{
        h3_index::{latLngToCell, setH3Index},
        lat_lng::LatLng,
    };

    use super::*;

    #[test]
    fn gridDisk0() {
        let sf = LatLng {
            lat: 0.659966917655,
            lng: 2.0 * 3.14159 - 2.1364398519396,
        };
        let sfHex0 = latLngToCell(&sf, 0).unwrap();

        let expectedK1: Vec<(H3Index, u32)> = vec![
            (0x8029fffffffffff, 0),
            (0x801dfffffffffff, 0),
            (0x8013fffffffffff, 0),
            (0x8027fffffffffff, 0),
            (0x8049fffffffffff, 0),
            (0x8051fffffffffff, 0),
            (0x8037fffffffffff, 0),
        ];

        let res = gridDiskDistances(sfHex0, 1).unwrap();
        for i in 0..7 {
            assert!(res[i].0 != 0, "index is populated");
            let mut in_list = 0;
            for j in 0..7 {
                if res[i].0 == expectedK1[j].0 {
                    assert!(
                        res[i].1 == (if res[i].0 == sfHex0 { 0 } else { 1 }),
                        "distance is as expected"
                    );
                    in_list += 1;
                }
            }
            assert!(in_list == 1, "index found in expected set");
        }
    }

    #[test]
    fn gridDisk0_PolarPentagon() {
        let mut polar: H3Index = 0;
        setH3Index(&mut polar, 0, 4, 0);

        let expectedK2: Vec<H3Index> = vec![
            0x8009fffffffffff,
            0x8007fffffffffff,
            0x8001fffffffffff,
            0x8011fffffffffff,
            0x801ffffffffffff,
            0x8019fffffffffff,
            0x0,
        ];

        let mut k2 = gridDiskDistances(polar, 1).unwrap();
        k2.resize(7, (0u64, 0));
        let mut k2present = 0;
        for i in 0..7 {
            if (k2[i].0 != 0) {
                k2present += 1;
                let mut in_list = 0;
                for j in 0..7 {
                    if k2[i].0 == expectedK2[j] {
                        assert!(
                            k2[i].1 == (if k2[i].0 == polar { 0 } else { 1 }),
                            "distance is as expected"
                        );
                        in_list += 1;
                    }
                }
                assert!(in_list == 1, "index found in expected set");
            }
        }
        assert!(k2present == 6, "pentagon has 5 neighbors");
    }
}
