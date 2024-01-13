use enum_primitive::FromPrimitive;
use num::pow;

use crate::base_cells::{
    _baseCellIsCwOffset, _faceIjkToBaseCell, _faceIjkToBaseCellCCWrot60, _isBaseCellPentagon,
    baseCellData, MAX_FACE_COORD,
};
use crate::coord_ijk::{
    CoordIJK, Direction, _downAp7, _downAp7r, _ijkNormalize, _ijkSub, _neighbor, _rotate60ccw,
    _rotate60cw, _unitIjkToDigit, _upAp7, _upAp7r,
};
use crate::error::Error;
use crate::face_ijk::{FaceIJK, Overage, _adjustOverageClassII, _faceIjkToGeo, _geoToFaceIjk};
use crate::iterators::IterCellsChildren;
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
 * Returns the H3 resolution of an H3 index.
 * @param h The H3 index.
 * @return The resolution of the H3 index argument.
 */
pub fn getResolution(h: H3Index) -> i32 {
    return H3_GET_RESOLUTION(h);
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
 * Determines whether one resolution is a valid child resolution for a cell.
 * Each resolution is considered a valid child resolution of itself.
 *
 * @param h         h3Index  parent cell
 * @param childRes  int      resolution of the child
 *
 * @return The validity of the child resolution
 */
pub fn _hasChildAtRes(h: H3Index, childRes: i32) -> bool {
    let parentRes = H3_GET_RESOLUTION(h);
    if childRes < parentRes || childRes > MAX_H3_RES {
        return false;
    }
    return true;
}

/**
 * cellToChildrenSize returns the exact number of children for a cell at a
 * given child resolution.
 *
 * @param h         H3Index to find the number of children of
 * @param childRes  The child resolution you're interested in
 *
 * @return int      Exact number of children (handles hexagons and pentagons
 *                  correctly)
 */
pub fn cellToChildrenSize(h: H3Index, childRes: i32) -> Result<i64, Error> {
    if !_hasChildAtRes(h, childRes) {
        return Err(Error::ResDomain);
    }

    let n = (childRes - H3_GET_RESOLUTION(h)) as u32;

    if isPentagon(h) {
        return Ok((1 + 5 * ((7i32.pow(n) - 1) / 6)) as i64);
    } else {
        return Ok(7i32.pow(n) as i64);
    }
}

/**
 * cellToChildren takes the given hexagon id and generates all of the children
 * at the specified resolution storing them into the provided memory pointer.
 * It's assumed that cellToChildrenSize was used to determine the allocation.
 *
 * @param h H3Index to find the children of
 * @param childRes int the child level to produce
 * @param children H3Index* the memory to store the resulting addresses in
 */
pub fn cellToChildren(h: H3Index, childRes: i32) -> Result<Vec<H3Index>, Error> {
    let mut children = Vec::<H3Index>::new();
    for child in IterCellsChildren::from_parent(h, childRes) {
        // (IterCellsChildren iter = iterInitParent(h, childRes); iter.h;
        //iterStepChild(&iter)) {
        children.push(child);
    }
    return Ok(children);
}

/**
 * Zero out index digits from start to end, inclusive.
 * No-op if start > end.
 */
pub fn _zeroIndexDigits(h: H3Index, start: i32, end: i32) -> H3Index {
    if start > end {
        return h;
    }

    let mut m: H3Index = 0;

    m = !m;
    m <<= H3_PER_DIGIT_OFFSET * (end - start + 1);
    m = !m;
    m <<= H3_PER_DIGIT_OFFSET * (MAX_H3_RES - end);
    m = !m;

    return h & m;
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

/**
 * Convert an H3Index to the FaceIJK address on a specified icosahedral face.
 * @param h The H3Index.
 * @param fijk The FaceIJK address, initialized with the desired face
 *        and normalized base cell coordinates.
 * @return Returns 1 if the possibility of overage exists, otherwise 0.
 */
pub fn _h3ToFaceIjkWithInitializedFijk(h: H3Index, fijk: &mut FaceIJK) -> bool {
    //CoordIJK *ijk = &fijk->coord;
    let res = H3_GET_RESOLUTION(h);

    // center base cell hierarchy is entirely on this face
    let mut possibleOverage = true;
    if !_isBaseCellPentagon(H3_GET_BASE_CELL(h))
        && (res == 0 || (fijk.coord.i == 0 && fijk.coord.j == 0 && fijk.coord.k == 0))
    {
        possibleOverage = false;
    }

    for r in 1..(res + 1) {
        if isResolutionClassIII(r) {
            // Class III == rotate ccw
            _downAp7(&mut fijk.coord);
        } else {
            // Class II == rotate cw
            _downAp7r(&mut fijk.coord);
        }

        _neighbor(&mut fijk.coord, H3_GET_INDEX_DIGIT(h, r));
    }

    return possibleOverage;
}

/**
 * Convert an H3Index to a FaceIJK address.
 * @param h The H3Index.
 * @param fijk The corresponding FaceIJK address.
 */
pub fn _h3ToFaceIjk(mut h: H3Index) -> Result<FaceIJK, Error> {
    let baseCell = H3_GET_BASE_CELL(h);
    if baseCell < 0 || baseCell >= NUM_BASE_CELLS {
        // LCOV_EXCL_BR_LINE
        // Base cells less than zero can not be represented in an index
        return Err(Error::CellInvalid);
    }
    // adjust for the pentagonal missing sequence; all of sub-sequence 5 needs
    // to be adjusted (and some of sub-sequence 4 below)
    if _isBaseCellPentagon(baseCell) && _h3LeadingNonZeroDigit(h) as i32 == 5 {
        h = _h3Rotate60cw(h);
    }

    // start with the "home" face and ijk+ coordinates for the base cell of c
    let mut fijk = baseCellData[baseCell as usize].homeFijk;
    if !_h3ToFaceIjkWithInitializedFijk(h, &mut fijk) {
        return Ok(fijk); // no overage is possible; h lies on this face
    }

    // if we're here we have the potential for an "overage"; i.e., it is
    // possible that c lies on an adjacent face
    let origIJK = fijk.coord;

    // if we're in Class III, drop into the next finer Class II grid
    let mut res = H3_GET_RESOLUTION(h);
    if isResolutionClassIII(res) {
        // Class III
        _downAp7r(&mut fijk.coord);
        res += 1;
    }

    // adjust for overage if needed
    // a pentagon base cell with a leading 4 digit requires special handling
    let pentLeading4 = _isBaseCellPentagon(baseCell) && (_h3LeadingNonZeroDigit(h) as i32) == 4;
    if _adjustOverageClassII(&mut fijk, res, pentLeading4, false) != Overage::NoOverage {
        // if the base cell is a pentagon we have the potential for secondary
        // overages
        if _isBaseCellPentagon(baseCell) {
            while _adjustOverageClassII(&mut fijk, res, false, false) != Overage::NoOverage {
                continue;
            }
        }

        if res != H3_GET_RESOLUTION(h) {
            _upAp7r(&mut fijk.coord);
        }
    } else if res != H3_GET_RESOLUTION(h) {
        fijk.coord = origIJK;
    }
    return Ok(fijk);
}

/**
 * Determines the spherical coordinates of the center point of an H3 index.
 *
 * @param h3 The H3 index.
 * @param g The spherical coordinates of the H3 cell center.
 */
pub fn cellToLatLng(h3: H3Index) -> Result<LatLng, Error> {
    let mut fijk: FaceIJK = _h3ToFaceIjk(h3)?;
    let geo = _faceIjkToGeo(fijk, H3_GET_RESOLUTION(h3));
    return Ok(geo);
}

/**
 * Validate a child position in the context of a given parent, returning
 * an error if validation fails.
 */
pub fn validateChildPos(childPos: i64, parent: H3Index, childRes: i32) -> Result<(), Error> {
    let maxChildCount = cellToChildrenSize(parent, childRes)?;
    if childPos < 0 || childPos >= maxChildCount {
        return Err(Error::Domain);
    }
    Ok(())
}

/**
 * Returns the child cell at a given position within an ordered list of all
 * children at the specified resolution */
pub fn childPosToCell(childPos: i64, parent: H3Index, childRes: i32) -> Result<H3Index, Error> {
    // Validate resolution
    if childRes < 0 || childRes > MAX_H3_RES {
        return Err(Error::ResDomain);
    }
    // Validate parent resolution
    let parentRes = H3_GET_RESOLUTION(parent);
    if childRes < parentRes {
        return Err(Error::ResMismatch);
    }
    // Validate child pos
    let _childPosErr = validateChildPos(childPos, parent, childRes)?;

    let resOffset = childRes - parentRes;

    let mut child = parent;
    let mut idx = childPos;

    H3_SET_RESOLUTION(&mut child, childRes);

    if isPentagon(parent) {
        // Pentagon tile logic. Pentagon tiles skip the 1 digit, so the offsets
        // are different
        let mut inPent = true;
        for res in 1..(resOffset + 1) {
            let resWidth = pow(7, (resOffset - res) as usize);
            if inPent {
                // While we are inside a parent pentagon, we need to check if
                // this cell is a pentagon, and if not, we need to offset its
                // digit to account for the skipped direction
                let pentWidth = 1 + (5 * (resWidth - 1)) / 6;
                if idx < pentWidth {
                    H3_SET_INDEX_DIGIT(&mut child, parentRes + res, 0);
                } else {
                    idx -= pentWidth;
                    inPent = false;
                    H3_SET_INDEX_DIGIT(&mut child, parentRes + res, (idx / resWidth) as i32 + 2);
                    idx %= resWidth;
                }
            } else {
                // We're no longer inside a pentagon, continue as for hex
                H3_SET_INDEX_DIGIT(&mut child, parentRes + res, (idx / resWidth) as i32);
                idx %= resWidth;
            }
        }
    } else {
        // Hexagon tile logic. Offsets are simple powers of 7
        for res in 1..(resOffset + 1) {
            let resWidth = pow(7, (resOffset - res) as usize);
            H3_SET_INDEX_DIGIT(&mut child, parentRes + res, (idx / resWidth) as i32);
            idx %= resWidth;
        }
    }

    return Ok(child);
}

#[cfg(test)]
mod tests {
    use num::Float;

    use crate::lat_lng::{geoAlmostEqualThreshold, setGeoDegs};

    use super::*;

    fn assertNoDuplicates(cells: &Vec<H3Index>) {
        for i in 0..cells.len() {
            if cells[i] == H3_NULL {
                continue;
            }
            assert!(isValidCell(cells[i]), "must be valid H3 cell");
            for j in (i + 1)..cells.len() {
                assert!(cells[i] != cells[j], "can't have duplicate cells in set");
            }
        }
    }

    // assert that set1 is a subset of set2
    fn assertSubset(set1: &Vec<H3Index>, set2: &Vec<H3Index>) {
        assertNoDuplicates(set1);

        for i in set1 {
            if *i == H3_NULL {
                continue;
            }

            let mut present = false;
            for j in set2 {
                if *i == *j {
                    present = true;
                    break;
                };
            }
            assert!(present, "children must match");
        }
    }

    /*
    Assert that two arrays of h3 cells are equal sets:
        - No duplicate cells allowed.
        - Ignore zero elements (so array sizes may be different).
        - Ignore array order.
    */
    fn assertSetsEqual(set1: &Vec<H3Index>, set2: &Vec<H3Index>) {
        assertSubset(set1, set2);
        assertSubset(set2, set1);
    }

    fn checkChildren(
        h: H3Index,
        res: i32,
        expectedError: Result<i64, Error>,
        expected: Vec<H3Index>,
    ) {
        let numChildren: i64 = 0;
        let numChildrenError = cellToChildrenSize(h, res);

        assert_eq!(numChildrenError, expectedError, "Expected error code");
        if expectedError.is_err() {
            return;
        }
        let children = cellToChildren(h, res).unwrap();

        assertSetsEqual(&children, &expected);
    }

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

    #[test]
    fn cellToChildrenSize_hexagon() {
        let h: H3Index = 0x87283080dffffff; // res 7 *hexagon*

        let mut sz: i64 = 0;
        assert!(
            cellToChildrenSize(h, 3) == Err(Error::ResDomain),
            "got expected size for coarser res"
        );
        sz = cellToChildrenSize(h, 7).unwrap();
        assert_eq!(sz, 1, "got expected size for same res");
        sz = cellToChildrenSize(h, 8).unwrap();
        assert_eq!(sz, 7, "got expected size for child res");
        sz = cellToChildrenSize(h, 9).unwrap();
        assert_eq!(sz, 7 * 7, "got expected size for grandchild res");
    }

    #[test]
    fn cellToChildrenSize_pentagon() {
        let h: H3Index = 0x870800000ffffff; // res 7 *pentagon*

        let mut sz: i64 = 0;
        assert!(
            cellToChildrenSize(h, 3) == Err(Error::ResDomain),
            "got expected size for coarser res"
        );
        sz = cellToChildrenSize(h, 7).unwrap();
        assert_eq!(sz, 1, "got expected size for same res");
        sz = cellToChildrenSize(h, 8).unwrap();
        assert_eq!(sz, 6, "got expected size for child res");
        sz = cellToChildrenSize(h, 9).unwrap();
        assert_eq!(
            sz,
            (5 * 7) + (1 * 6),
            "got expected size for grandchild res"
        );
    }

    #[test]
    fn oneResStep() {
        let h: H3Index = 0x88283080ddfffff;
        let res = 9;

        let expected: Vec<H3Index> = vec![
            0x89283080dc3ffff,
            0x89283080dc7ffff,
            0x89283080dcbffff,
            0x89283080dcfffff,
            0x89283080dd3ffff,
            0x89283080dd7ffff,
            0x89283080ddbffff,
        ];

        checkChildren(h, res, Ok(expected.len() as i64), expected);
    }

    #[test]
    fn multipleResSteps() {
        let h = 0x88283080ddfffff;
        let res = 10;

        let expected = vec![
            0x8a283080dd27fff,
            0x8a283080dd37fff,
            0x8a283080dc47fff,
            0x8a283080dcdffff,
            0x8a283080dc5ffff,
            0x8a283080dc27fff,
            0x8a283080ddb7fff,
            0x8a283080dc07fff,
            0x8a283080dd8ffff,
            0x8a283080dd5ffff,
            0x8a283080dc4ffff,
            0x8a283080dd47fff,
            0x8a283080dce7fff,
            0x8a283080dd1ffff,
            0x8a283080dceffff,
            0x8a283080dc6ffff,
            0x8a283080dc87fff,
            0x8a283080dcaffff,
            0x8a283080dd2ffff,
            0x8a283080dcd7fff,
            0x8a283080dd9ffff,
            0x8a283080dd6ffff,
            0x8a283080dcc7fff,
            0x8a283080dca7fff,
            0x8a283080dccffff,
            0x8a283080dd77fff,
            0x8a283080dc97fff,
            0x8a283080dd4ffff,
            0x8a283080dd97fff,
            0x8a283080dc37fff,
            0x8a283080dc8ffff,
            0x8a283080dcb7fff,
            0x8a283080dcf7fff,
            0x8a283080dd87fff,
            0x8a283080dda7fff,
            0x8a283080dc9ffff,
            0x8a283080dc77fff,
            0x8a283080dc67fff,
            0x8a283080dc57fff,
            0x8a283080ddaffff,
            0x8a283080dd17fff,
            0x8a283080dc17fff,
            0x8a283080dd57fff,
            0x8a283080dc0ffff,
            0x8a283080dd07fff,
            0x8a283080dc1ffff,
            0x8a283080dd0ffff,
            0x8a283080dc2ffff,
            0x8a283080dd67fff,
        ];

        checkChildren(h, res, Ok(expected.len() as i64), expected);
    }

    fn assertCellToLatLngExpected(h1: H3Index, g1: LatLng) {
        let epsilon = 0.000001 * M_PI_180;
        // convert H3 to lat/lng and verify
        let g2 = cellToLatLng(h1).unwrap();
        //assert_eq!(g1.lat, g2.lat);
        //assert_eq!(g1.lng, g2.lng);

        assert!(
            geoAlmostEqualThreshold(&g2, &g1, epsilon),
            "got expected cellToLatLng output"
        );

        // Convert back to H3 to verify
        let res = getResolution(h1);
        let h2 = latLngToCell(&g2, res).unwrap();
        assert_eq!(h1, h2, "got expected latLngToCell output");
    }

    #[test]
    fn provisionalCellToLatLngTest() {
        assertCellToLatLngExpected(
            0x8001fffffffffff,
            LatLng {
                lat: 79.2423985098.to_radians(),
                lng: 38.0234070080.to_radians(),
            },
        );

        assertCellToLatLngExpected(
            0x8045fffffffffff,
            LatLng {
                lat: 25.4691389839.to_radians(),
                lng: -85.1593898623.to_radians(),
            },
        );

        assertCellToLatLngExpected(
            0x81ccbffffffffff,
            LatLng {
                lat: -35.9592925857.to_radians(),
                lng: 84.9085000539.to_radians(),
            },
        );

        assertCellToLatLngExpected(
            0x845a5ebffffffff,
            LatLng {
                lat: 16.7016667635.to_radians(),
                lng: 164.8158089958.to_radians(),
            },
        );
    }
}
