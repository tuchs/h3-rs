use enum_primitive::FromPrimitive;

use crate::base_cells::{
    _baseCellIsCwOffset, _faceIjkToBaseCell, _faceIjkToBaseCellCCWrot60, _isBaseCellPentagon,
    MAX_FACE_COORD,
};
use crate::coord_ijk::{
    CoordIJK, Direction, _downAp7, _downAp7r, _ijkNormalize, _ijkSub, _rotate60ccw, _rotate60cw,
    _unitIjkToDigit, _upAp7, _upAp7r,
};
use crate::error::Error;
use crate::face_ijk::{FaceIJK, _geoToFaceIjk};
use crate::lat_lng::LatLng;
use crate::{constants::*, H3_NULL};

/** H3 index with mode 0, res 0, base cell 0, and 7 for all index digits. */
pub const H3_INIT: u64 = 35184372088831u64;

/** The number of bits in an H3 index. */
pub const H3_NUM_BITS: i32 = 64;

/** The bit offset of the max resolution digit in an H3 index. */
pub const H3_MAX_OFFSET: i32 = 63;

/** The bit offset of the mode in an H3 index. */
pub const H3_MODE_OFFSET: i32 = 59;

/** The bit offset of the base cell in an H3 index. */
pub const H3_BC_OFFSET: i32 = 45;

/** The bit offset of the resolution in an H3 index. */
pub const H3_RES_OFFSET: i32 = 52;

/** The bit offset of the reserved bits in an H3 index. */
pub const H3_RESERVED_OFFSET: i32 = 56;

/** The number of bits in a single H3 resolution digit. */
pub const H3_PER_DIGIT_OFFSET: i32 = 3;

/** 1 in the highest bit, 0's everywhere else. */
pub const H3_HIGH_BIT_MASK: u64 = 1u64 << H3_MAX_OFFSET;

/** 0 in the highest bit, 1's everywhere else. */
pub const H3_HIGH_BIT_MASK_NEGATIVE: u64 = !H3_HIGH_BIT_MASK;

/** 1's in the 4 mode bits, 0's everywhere else. */
pub const H3_MODE_MASK: u64 = 15u64 << H3_MODE_OFFSET;

/** 0's in the 4 mode bits, 1's everywhere else. */
pub const H3_MODE_MASK_NEGATIVE: u64 = !H3_MODE_MASK;

/** 1's in the 7 base cell bits, 0's everywhere else. */
pub const H3_BC_MASK: u64 = (127u64) << H3_BC_OFFSET;

/** 0's in the 7 base cell bits, 1's everywhere else. */
pub const H3_BC_MASK_NEGATIVE: u64 = !H3_BC_MASK;

/** 1's in the 3 reserved bits, 0's everywhere else. */
pub const H3_RESERVED_MASK: u64 = 7u64 << H3_RESERVED_OFFSET;

/** 0's in the 3 reserved bits, 1's everywhere else. */
pub const H3_RESERVED_MASK_NEGATIVE: u64 = !H3_RESERVED_MASK;

/** 1's in the 4 resolution bits, 0's everywhere else. */
pub const H3_RES_MASK: u64 = 15u64 << H3_RES_OFFSET;

/** 0's in the 4 resolution bits, 1's everywhere else. */
pub const H3_RES_MASK_NEGATIVE: u64 = !H3_RES_MASK;

/** 1's in the 3 bits of res 15 digit bits, 0's everywhere else. */
pub const H3_DIGIT_MASK: u64 = 7u64;

/** 0's in the 7 base cell bits, 1's everywhere else. */
pub const H3_DIGIT_MASK_NEGATIVE: u64 = !H3_DIGIT_MASK;

#[doc = " @brief the H3Index fits within a 64-bit unsigned integer"]
pub type H3Index = u64;

/**
 * Gets the highest bit of the H3 index.
 */
pub fn H3_GET_HIGH_BIT(h3: H3Index) -> i32 {
    return (((h3) & H3_HIGH_BIT_MASK) >> H3_MAX_OFFSET) as i32;
}

/**
 * Sets the highest bit of the h3 to v.
 */
pub fn H3_SET_HIGH_BIT(h3: &mut H3Index, v: i32) {
    *h3 = ((*h3) & H3_HIGH_BIT_MASK_NEGATIVE) | ((v as u64) << H3_MAX_OFFSET);
}

/**
 * Gets the integer mode of h3.
 */
pub fn H3_GET_MODE(h3: H3Index) -> i32 {
    return (((h3) & H3_MODE_MASK) >> H3_MODE_OFFSET) as i32;
}

/**
 * Sets the integer mode of h3 to v.
 */
pub fn H3_SET_MODE(h3: &mut H3Index, v: i32) {
    *h3 = ((*h3) & H3_MODE_MASK_NEGATIVE) | ((v as u64) << H3_MODE_OFFSET);
}

/**
 * Gets the integer base cell of h3.
 */
pub fn H3_GET_BASE_CELL(h3: H3Index) -> i32 {
    return (((h3) & H3_BC_MASK) >> H3_BC_OFFSET) as i32;
}

/**
 * Sets the integer base cell of h3 to bc.
 */
pub fn H3_SET_BASE_CELL(h3: &mut H3Index, bc: i32) {
    *h3 = ((*h3) & H3_BC_MASK_NEGATIVE) | ((bc as u64) << H3_BC_OFFSET)
}

/**
 * Gets the integer resolution of h3.
 */
pub fn H3_GET_RESOLUTION(h3: H3Index) -> i32 {
    return (((h3) & H3_RES_MASK) >> H3_RES_OFFSET) as i32;
}

/**
 * Sets the integer resolution of h3.
 */
pub fn H3_SET_RESOLUTION(h3: &mut H3Index, res: i32) {
    *h3 = ((*h3) & H3_RES_MASK_NEGATIVE) | ((res as u64) << H3_RES_OFFSET)
}

/**
 * Gets the resolution res integer digit (0-7) of h3.
 */
pub fn H3_GET_INDEX_DIGIT(h3: H3Index, res: i32) -> Direction {
    return Direction::from_u64(
        ((h3) >> ((MAX_H3_RES - (res)) * H3_PER_DIGIT_OFFSET)) & H3_DIGIT_MASK,
    )
    .unwrap_or(Direction::InvalidDigit);
}

/**
 * Sets the resolution res digit of h3 to the integer digit (0-7)
 */
pub fn H3_SET_INDEX_DIGIT(h3: &mut H3Index, res: i32, digit: i32) {
    *h3 = ((*h3) & !(H3_DIGIT_MASK << ((MAX_H3_RES - (res)) * H3_PER_DIGIT_OFFSET)))
        | ((digit as u64) << ((MAX_H3_RES - (res)) * H3_PER_DIGIT_OFFSET))
}

/**
 * Sets a value in the reserved space. Setting to non-zero may produce invalid
 * indexes.
 */
pub fn H3_SET_RESERVED_BITS(h3: &mut H3Index, v: i32) {
    *h3 = ((*h3) & H3_RESERVED_MASK_NEGATIVE) | ((v as u64) << H3_RESERVED_OFFSET);
}

/**
 * Gets a value in the reserved space. Should always be zero for valid indexes.
 */
pub fn H3_GET_RESERVED_BITS(h3: H3Index) -> i32 {
    return ((h3 & H3_RESERVED_MASK) >> H3_RESERVED_OFFSET) as i32;
}

/**
 * Returns whether or not an H3 index is a valid cell (hexagon or pentagon).
 * @param h The H3 index to validate.
 * @return 1 if the H3 index if valid, and 0 if it is not.
 */
pub fn isValidCell(h: H3Index) -> bool {
    if H3_GET_HIGH_BIT(h) != 0 {
        return false;
    }

    if H3_GET_MODE(h) != H3_CELL_MODE {
        return false;
    }

    if H3_GET_RESERVED_BITS(h) != 0 {
        return false;
    }

    let baseCell = H3_GET_BASE_CELL(h);
    if baseCell < 0 || baseCell >= NUM_BASE_CELLS {
        // LCOV_EXCL_BR_LINE
        // Base cells less than zero can not be represented in an index
        return false;
    }

    let res = H3_GET_RESOLUTION(h);
    if res < 0 || res > MAX_H3_RES {
        // LCOV_EXCL_BR_LINE
        // Resolutions less than zero can not be represented in an index
        return false;
    }

    let mut foundFirstNonZeroDigit = false;
    for r in 1..(res + 1) {
        let digit = H3_GET_INDEX_DIGIT(h, r);

        if !foundFirstNonZeroDigit && digit != Direction::CenterDigit {
            foundFirstNonZeroDigit = true;
            if _isBaseCellPentagon(baseCell) && digit == Direction::KAxesDigit {
                return false;
            }
        }

        if digit < Direction::CenterDigit || digit >= Direction::NUM_DIGITS {
            return false;
        }
    }

    for r in (res + 1)..(MAX_H3_RES + 1) {
        let digit = H3_GET_INDEX_DIGIT(h, r);
        if digit != Direction::InvalidDigit {
            return false;
        }
    }

    return true;
}

/**
 * Initializes an H3 index.
 * @param hp The H3 index to initialize.
 * @param res The H3 resolution to initialize the index to.
 * @param baseCell The H3 base cell to initialize the index to.
 * @param initDigit The H3 digit (0-7) to initialize all of the index digits to.
 */
pub fn setH3Index(hp: &mut H3Index, res: i32, base_cell: i32, init_digit: i32) {
    let mut h: H3Index = H3_INIT;
    H3_SET_MODE(&mut h, H3_CELL_MODE);
    H3_SET_RESOLUTION(&mut h, res);
    H3_SET_BASE_CELL(&mut h, base_cell);
    for r in 1..(res + 1) {
        H3_SET_INDEX_DIGIT(&mut h, r, init_digit);
    }
    *hp = h;
}

/**
 * Encodes a coordinate on the sphere to the H3 index of the containing cell at
 * the specified resolution.
 *
 * Returns 0 on invalid input.
 *
 * @param g The spherical coordinates to encode.
 * @param res The desired H3 resolution for the encoding.
 * @param out The encoded H3Index.
 * @returns E_SUCCESS (0) on success, another value otherwise
 */
pub fn latLngToCell(g: &LatLng, res: i32) -> Result<H3Index, Error> {
    if res < 0 || res > MAX_H3_RES {
        return Err(Error::ResDomain);
    }
    if !g.lat.is_finite() || !g.lng.is_finite() {
        return Err(Error::LatLngDomain);
    }

    let fijk: FaceIJK = _geoToFaceIjk(g, res);
    return Ok(_faceIjkToH3(&fijk, res));
}

/**
 * Returns whether or not a resolution is a Class III grid. Note that odd
 * resolutions are Class III and even resolutions are Class II.
 * @param res The H3 resolution.
 * @return 1 if the resolution is a Class III grid, and 0 if the resolution is
 *         a Class II grid.
 */
pub fn isResolutionClassIII(res: i32) -> bool {
    return (res % 2) != 0;
}

/**
 * h3IsPentagon takes an H3Index and determines if it is actually a
 * pentagon.
 * @param h The H3Index to check.
 * @return Returns 1 if it is a pentagon, otherwise 0.
 */
pub fn isPentagon(h: H3Index) -> bool {
    return _isBaseCellPentagon(H3_GET_BASE_CELL(h))
        && (_h3LeadingNonZeroDigit(h) == Direction::CenterDigit);
}

/**
 * Returns the highest resolution non-zero digit in an H3Index.
 * @param h The H3Index.
 * @return The highest resolution non-zero digit in the H3Index.
 */
pub fn _h3LeadingNonZeroDigit(h: H3Index) -> Direction {
    for r in 1..(H3_GET_RESOLUTION(h) + 1) {
        if H3_GET_INDEX_DIGIT(h, r) != Direction::CenterDigit {
            return H3_GET_INDEX_DIGIT(h, r);
        }
    }

    // if we're here it's all 0's
    return Direction::CenterDigit;
}

/**
 * Rotate an H3Index 60 degrees counter-clockwise about a pentagonal center.
 * @param h The H3Index.
 */
pub fn _h3RotatePent60ccw(mut h: H3Index) -> H3Index {
    // rotate in place; skips any leading 1 digits (k-axis)

    let mut foundFirstNonZeroDigit: i32 = 0;
    let res: i32 = H3_GET_RESOLUTION(h);
    for r in 1..(res + 1) {
        // rotate this digit
        let oldIndex = H3_GET_INDEX_DIGIT(h, r);
        H3_SET_INDEX_DIGIT(&mut h, r, _rotate60ccw(oldIndex) as i32);

        // look for the first non-zero digit so we
        // can adjust for deleted k-axes sequence
        // if necessary
        if foundFirstNonZeroDigit == 0 && (H3_GET_INDEX_DIGIT(h, r) as i32) != 0 {
            foundFirstNonZeroDigit = 1;

            // adjust for deleted k-axes sequence
            if _h3LeadingNonZeroDigit(h) == Direction::KAxesDigit {
                h = _h3Rotate60ccw(h);
            }
        }
    }
    return h;
}

/**
 * Rotate an H3Index 60 degrees clockwise about a pentagonal center.
 * @param h The H3Index.
 */
pub fn _h3RotatePent60cw(mut h: H3Index) -> H3Index {
    // rotate in place; skips any leading 1 digits (k-axis)

    let mut foundFirstNonZeroDigit: i32 = 0;
    let res: i32 = H3_GET_RESOLUTION(h);
    for r in 1..(res + 1) {
        // rotate this digit
        let oldIndex = H3_GET_INDEX_DIGIT(h, r);
        H3_SET_INDEX_DIGIT(&mut h, r, _rotate60cw(oldIndex) as i32);

        // look for the first non-zero digit so we
        // can adjust for deleted k-axes sequence
        // if necessary
        if foundFirstNonZeroDigit == 0 && (H3_GET_INDEX_DIGIT(h, r) as i32) != 0 {
            foundFirstNonZeroDigit = 1;

            // adjust for deleted k-axes sequence
            if _h3LeadingNonZeroDigit(h) == Direction::KAxesDigit {
                h = _h3Rotate60cw(h);
            }
        }
    }
    return h;
}

/**
 * Rotate an H3Index 60 degrees counter-clockwise.
 * @param h The H3Index.
 */
pub fn _h3Rotate60ccw(mut h: H3Index) -> H3Index {
    let res: i32 = H3_GET_RESOLUTION(h);
    for r in 1..(res + 1) {
        let oldDigit: Direction = H3_GET_INDEX_DIGIT(h, r);
        H3_SET_INDEX_DIGIT(&mut h, r, _rotate60ccw(oldDigit) as i32);
    }

    return h;
}

/**
 * Rotate an H3Index 60 degrees clockwise.
 * @param h The H3Index.
 */
pub fn _h3Rotate60cw(mut h: H3Index) -> H3Index {
    let res: i32 = H3_GET_RESOLUTION(h);
    for r in 1..(res + 1) {
        let oldIndex = H3_GET_INDEX_DIGIT(h, r);
        H3_SET_INDEX_DIGIT(&mut h, r, _rotate60cw(oldIndex) as i32);
    }

    return h;
}

/**
 * Convert an FaceIJK address to the corresponding H3Index.
 * @param fijk The FaceIJK address.
 * @param res The cell resolution.
 * @return The encoded H3Index (or 0 on failure).
 */
pub fn _faceIjkToH3(fijk: &FaceIJK, res: i32) -> H3Index {
    // initialize the index
    let mut h: H3Index = H3_INIT;
    H3_SET_MODE(&mut h, H3_CELL_MODE);
    H3_SET_RESOLUTION(&mut h, res);

    // check for res 0/base cell
    if res == 0 {
        if fijk.coord.i > MAX_FACE_COORD
            || fijk.coord.j > MAX_FACE_COORD
            || fijk.coord.k > MAX_FACE_COORD
        {
            // out of range input
            return H3_NULL;
        }

        H3_SET_BASE_CELL(&mut h, _faceIjkToBaseCell(fijk));
        return h;
    }

    // we need to find the correct base cell FaceIJK for this H3 index;
    // start with the passed in face and resolution res ijk coordinates
    // in that face's coordinate system
    let mut fijkBC: FaceIJK = *fijk;

    // build the H3Index from finest res up
    // adjust r for the fact that the res 0 base cell offsets the indexing
    // digits
    let mut ijk: CoordIJK = fijkBC.coord;
    for r in (0..res).rev() {
        let lastIJK: CoordIJK = ijk;
        let mut lastCenter: CoordIJK;
        if isResolutionClassIII(r + 1) {
            // rotate ccw
            _upAp7(&mut ijk);
            lastCenter = ijk;
            _downAp7(&mut lastCenter);
        } else {
            // rotate cw
            _upAp7r(&mut ijk);
            lastCenter = ijk;
            _downAp7r(&mut lastCenter);
        }

        let mut diff: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
        _ijkSub(lastIJK, lastCenter, &mut diff);
        _ijkNormalize(&mut diff);

        H3_SET_INDEX_DIGIT(&mut h, r + 1, _unitIjkToDigit(diff) as i32);
    }
    fijkBC.coord = ijk;

    // fijkBC should now hold the IJK of the base cell in the
    // coordinate system of the current face

    if fijkBC.coord.i > MAX_FACE_COORD
        || fijkBC.coord.j > MAX_FACE_COORD
        || fijkBC.coord.k > MAX_FACE_COORD
    {
        // out of range input
        return H3_NULL;
    }

    // lookup the correct base cell
    let baseCell: i32 = _faceIjkToBaseCell(&fijkBC);
    H3_SET_BASE_CELL(&mut h, baseCell);

    // rotate if necessary to get canonical base cell orientation
    // for this base cell
    let numRots: i32 = _faceIjkToBaseCellCCWrot60(&fijkBC);
    if _isBaseCellPentagon(baseCell) {
        // force rotation out of missing k-axes sub-sequence
        if _h3LeadingNonZeroDigit(h) == Direction::KAxesDigit {
            // check for a cw/ccw offset face; default is ccw
            if _baseCellIsCwOffset(baseCell, fijkBC.face) {
                h = _h3Rotate60cw(h);
            } else {
                h = _h3Rotate60ccw(h);
            }
        }

        for _i in 0..numRots {
            h = _h3RotatePent60ccw(h);
        }
    } else {
        for _i in 0..numRots {
            h = _h3Rotate60ccw(h);
        }
    }

    return h;
}

#[cfg(test)]
mod tests {
    use num::Float;

    use crate::lat_lng::setGeoDegs;

    use super::*;

    #[test]
    fn latLngToCellExtremeCoordinates() {
        // Check that none of these cause crashes.
        let g = LatLng {
            lat: 0.0,
            lng: 1E45,
        };
        latLngToCell(&g, 14).unwrap();

        let g2 = LatLng {
            lat: 1E46,
            lng: 1E45,
        };
        latLngToCell(&g2, 15).unwrap();

        let mut g4 = LatLng { lat: 0.0, lng: 0.0 };
        setGeoDegs(&mut g4, 2.0, -3E39);
        latLngToCell(&g4, 0).unwrap();
    }

    #[test]
    fn faceIjkToH3ExtremeCoordinates() {
        let fijk0I = FaceIJK {
            face: 0,
            coord: CoordIJK { i: 3, j: 0, k: 0 },
        };
        assert!(_faceIjkToH3(&fijk0I, 0) == 0, "i out of bounds at res 0");
        let fijk0J = FaceIJK {
            face: 1,
            coord: CoordIJK { i: 0, j: 4, k: 0 },
        };
        assert!(_faceIjkToH3(&fijk0J, 0) == 0, "j out of bounds at res 0");
        let fijk0K = FaceIJK {
            face: 2,
            coord: CoordIJK { i: 2, j: 0, k: 5 },
        };
        assert!(_faceIjkToH3(&fijk0K, 0) == 0, "k out of bounds at res 0");

        let fijk1I = FaceIJK {
            face: 3,
            coord: CoordIJK { i: 6, j: 0, k: 0 },
        };
        assert!(_faceIjkToH3(&fijk1I, 1) == 0, "i out of bounds at res 1");
        let fijk1J = FaceIJK {
            face: 4,
            coord: CoordIJK { i: 0, j: 7, k: 1 },
        };
        assert!(_faceIjkToH3(&fijk1J, 1) == 0, "j out of bounds at res 1");
        let fijk1K = FaceIJK {
            face: 5,
            coord: CoordIJK { i: 2, j: 0, k: 8 },
        };
        assert!(_faceIjkToH3(&fijk1K, 1) == 0, "k out of bounds at res 1");

        let fijk2I = FaceIJK {
            face: 6,
            coord: CoordIJK { i: 18, j: 0, k: 0 },
        };
        assert!(_faceIjkToH3(&fijk2I, 2) == 0, "i out of bounds at res 2");
        let fijk2J = FaceIJK {
            face: 7,
            coord: CoordIJK { i: 0, j: 19, k: 1 },
        };
        assert!(_faceIjkToH3(&fijk2J, 2) == 0, "j out of bounds at res 2");
        let fijk2K = FaceIJK {
            face: 8,
            coord: CoordIJK { i: 2, j: 0, k: 20 },
        };
        assert!(_faceIjkToH3(&fijk2K, 2) == 0, "k out of bounds at res 2");
    }

    #[test]
    fn isValidCellAtResolution() {
        for i in 0..(MAX_H3_RES + 1) {
            let g: LatLng = LatLng { lat: 0.0, lng: 0.0 };
            let mut h3: H3Index = 0;
            h3 = latLngToCell(&g, i).unwrap();
            assert!(isValidCell(h3), "isValidCell failed on resolution {}", i);
        }
    }
}
