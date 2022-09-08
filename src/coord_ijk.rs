use num::FromPrimitive;

use crate::constants::*;
use crate::vec2d::Vec2d;

#[derive(Copy, Clone)]
pub struct CoordIJK {
    pub i: i32,
    pub j: i32,
    pub k: i32,
}

/** @brief CoordIJK unit vectors corresponding to the 7 H3 digits.
 */
pub const UNIT_VECS: [CoordIJK; 7] = [
    CoordIJK { i: 0, j: 0, k: 0 }, // direction 0
    CoordIJK { i: 0, j: 0, k: 1 }, // direction 1
    CoordIJK { i: 0, j: 1, k: 0 }, // direction 2
    CoordIJK { i: 0, j: 1, k: 1 }, // direction 3
    CoordIJK { i: 1, j: 0, k: 0 }, // direction 4
    CoordIJK { i: 1, j: 0, k: 1 }, // direction 5
    CoordIJK { i: 1, j: 1, k: 0 }, // direction 6
];

/** @brief H3 digit representing ijk+ axes direction.
 * Values will be within the lowest 3 bits of an integer.
 */
enum_from_primitive! {
    #[derive(PartialEq, PartialOrd, Copy, Clone)]
    pub enum Direction {
        /** H3 digit in center */
        CenterDigit = 0,
        /** H3 digit in k-axes direction */
        KAxesDigit = 1,
        /** H3 digit in j-axes direction */
        JAxesDigit = 2,
        /** H3 digit in j == k direction */
        JKAxesDigit = 3,
        /** H3 digit in i-axes direction */
        IAxesDigit = 4,
        /** H3 digit in i == k direction */
        IKAxesDigit = 5,
        /** H3 digit in i == j direction */
        IJAxesDigit = 6,
        /** H3 digit in the invalid direction */
        InvalidDigit = 7,
    }
}

impl Direction {
    /** Valid digits will be less than this value. Same value as INVALID_DIGIT.
     */
    pub const NUM_DIGITS: Direction = Direction::InvalidDigit;
    /** Child digit which is skipped for pentagons */
    pub const PENTAGON_SKIPPED_DIGIT: Direction = Direction::KAxesDigit; /* 1 */
}

/**
 * Sets an IJK coordinate to the specified component values.
 *
 * @param ijk The IJK coordinate to set.
 * @param i The desired i component value.
 * @param j The desired j component value.
 * @param k The desired k component value.
 */
pub fn _setIJK(ijk: &mut CoordIJK, i: i32, j: i32, k: i32) {
    ijk.i = i;
    ijk.j = j;
    ijk.k = k;
}

/**
 * Determine the containing hex in ijk+ coordinates for a 2D cartesian
 * coordinate vector (from DGGRID).
 *
 * @param v The 2D cartesian coordinate vector.
 * @param h The ijk+ coordinates of the containing hex.
 */
pub fn _hex2dToCoordIJK(v: Vec2d, h: &mut CoordIJK) {
    let a1: f64;
    let a2: f64;
    let x1: f64;
    let x2: f64;
    let m1: i32;
    let m2: i32;
    let r1: f64;
    let r2: f64;

    // quantize into the ij system and then normalize
    h.k = 0;

    a1 = v.x.abs();
    a2 = v.y.abs();

    // first do a reverse conversion
    x2 = a2 / M_SIN60;
    x1 = a1 + x2 / 2.0f64;

    // check if we have the center of a hex
    m1 = x1 as i32;
    m2 = x2 as i32;

    // otherwise round correctly
    r1 = x1 - (m1 as f64);
    r2 = x2 - (m2 as f64);

    if r1 < 0.5f64 {
        if r1 < 1.0f64 / 3.0f64 {
            if r2 < (1.0f64 + r1) / 2.0f64 {
                h.i = m1;
                h.j = m2;
            } else {
                h.i = m1;
                h.j = m2 + 1;
            }
        } else {
            if r2 < (1.0f64 - r1) {
                h.j = m2;
            } else {
                h.j = m2 + 1;
            }

            if (1.0f64 - r1) <= r2 && r2 < (2.0 * r1) {
                h.i = m1 + 1;
            } else {
                h.i = m1;
            }
        }
    } else {
        if r1 < 2.0f64 / 3.0f64 {
            if r2 < (1.0f64 - r1) {
                h.j = m2;
            } else {
                h.j = m2 + 1;
            }

            if (2.0f64 * r1 - 1.0f64) < r2 && r2 < (1.0f64 - r1) {
                h.i = m1;
            } else {
                h.i = m1 + 1;
            }
        } else {
            if r2 < (r1 / 2.0f64) {
                h.i = m1 + 1;
                h.j = m2;
            } else {
                h.i = m1 + 1;
                h.j = m2 + 1;
            }
        }
    }

    // now fold across the axes if necessary

    if v.x < 0.0f64 {
        if (h.j % 2) == 0
        // even
        {
            let axisi: i32 = h.j / 2;
            let diff: i32 = h.i - axisi;
            h.i = h.i - 2 * diff;
        } else {
            let axisi: i32 = (h.j + 1) / 2;
            let diff: i32 = h.i - axisi;
            h.i = h.i - (2 * diff + 1);
        }
    }

    if v.y < 0.0f64 {
        h.i = h.i - (2 * h.j + 1) / 2;
        h.j = -1 * h.j;
    }

    _ijkNormalize(h);
}

/**
 * Find the center point in 2D cartesian coordinates of a hex.
 *
 * @param h The ijk coordinates of the hex.
 * @param v The 2D cartesian coordinates of the hex center point.
 */
pub fn _ijkToHex2d(h: &CoordIJK) -> Vec2d {
    let i = h.i - h.k;
    let j = h.j - h.k;

    let v: Vec2d = Vec2d {
        x: (i as f64) - 0.5f64 * (j as f64),
        y: (j as f64) * M_SQRT3_2,
    };
    return v;
}

/**
 * Returns whether or not two ijk coordinates contain exactly the same
 * component values.
 *
 * @param c1 The first set of ijk coordinates.
 * @param c2 The second set of ijk coordinates.
 * @return 1 if the two addresses match, 0 if they do not.
 */
pub fn _ijkMatches(c1: CoordIJK, c2: CoordIJK) -> bool {
    return c1.i == c2.i && c1.j == c2.j && c1.k == c2.k;
}

/**
 * Add two ijk coordinates.
 *
 * @param h1 The first set of ijk coordinates.
 * @param h2 The second set of ijk coordinates.
 * @param sum The sum of the two sets of ijk coordinates.
 */
pub fn _ijkAdd(h1: CoordIJK, h2: CoordIJK, sum: &mut CoordIJK) {
    sum.i = h1.i + h2.i;
    sum.j = h1.j + h2.j;
    sum.k = h1.k + h2.k;
}

/**
 * Subtract two ijk coordinates.
 *
 * @param h1 The first set of ijk coordinates.
 * @param h2 The second set of ijk coordinates.
 * @param diff The difference of the two sets of ijk coordinates (h1 - h2).
 */
pub fn _ijkSub(h1: CoordIJK, h2: CoordIJK, diff: &mut CoordIJK) {
    diff.i = h1.i - h2.i;
    diff.j = h1.j - h2.j;
    diff.k = h1.k - h2.k;
}

/**
 * Uniformly scale ijk coordinates by a scalar. Works in place.
 *
 * @param c The ijk coordinates to scale.
 * @param factor The scaling factor.
 */
pub fn _ijkScale(c: &mut CoordIJK, factor: i32) {
    c.i *= factor;
    c.j *= factor;
    c.k *= factor;
}

/**
 * Normalizes ijk coordinates by setting the components to the smallest possible
 * values. Works in place.
 *
 * @param c The ijk coordinates to normalize.
 */
pub fn _ijkNormalize(c: &mut CoordIJK) {
    // remove any negative values
    if c.i < 0 {
        c.j -= c.i;
        c.k -= c.i;
        c.i = 0;
    }

    if c.j < 0 {
        c.i -= c.j;
        c.k -= c.j;
        c.j = 0;
    }

    if c.k < 0 {
        c.i -= c.k;
        c.j -= c.k;
        c.k = 0;
    }

    // remove the min value if needed
    let mut min: i32 = c.i;
    if c.j < min {
        min = c.j;
    }
    if c.k < min {
        min = c.k;
    }
    if min > 0 {
        c.i -= min;
        c.j -= min;
        c.k -= min;
    }
}

/**
 * Determines the H3 digit corresponding to a unit vector in ijk coordinates.
 *
 * @param ijk The ijk coordinates; must be a unit vector.
 * @return The H3 digit (0-6) corresponding to the ijk unit vector, or
 * InvalidIndex on failure.
 */
pub fn _unitIjkToDigit(ijk: CoordIJK) -> Direction {
    let mut c: CoordIJK = ijk;
    _ijkNormalize(&mut c);

    let mut digit: Direction = Direction::InvalidDigit;
    for i in (Direction::CenterDigit as usize)..(Direction::InvalidDigit as usize) {
        // Direction::iterator() {
        if _ijkMatches(c, UNIT_VECS[i]) {
            digit = Direction::from_usize(i).unwrap_or(Direction::InvalidDigit);
            break;
        }
    }

    return digit;
}

/**
 * Find the normalized ijk coordinates of the indexing parent of a cell in a
 * counter-clockwise aperture 7 grid. Works in place.
 *
 * @param ijk The ijk coordinates.
 */
pub fn _upAp7(ijk: &mut CoordIJK) {
    // convert to CoordIJ
    let i: i32 = ijk.i - ijk.k;
    let j: i32 = ijk.j - ijk.k;

    ijk.i = ((3.0f64 * (i as f64) - (j as f64)) / 7.0f64).round() as i32;
    ijk.j = (((i as f64) + 2.0f64 * (j as f64)) / 7.0f64).round() as i32;
    ijk.k = 0;
    _ijkNormalize(ijk);
}

/**
 * Find the normalized ijk coordinates of the indexing parent of a cell in a
 * clockwise aperture 7 grid. Works in place.
 *
 * @param ijk The ijk coordinates.
 */
pub fn _upAp7r(ijk: &mut CoordIJK) {
    // convert to CoordIJ
    let i: i32 = ijk.i - ijk.k;
    let j: i32 = ijk.j - ijk.k;

    ijk.i = ((2.0f64 * (i as f64) + (j as f64)) / 7.0f64).round() as i32;
    ijk.j = ((3.0f64 * (j as f64) - (i as f64)) / 7.0f64).round() as i32;
    ijk.k = 0;
    _ijkNormalize(ijk);
}

/**
 * Find the normalized ijk coordinates of the hex centered on the indicated
 * hex at the next finer aperture 7 counter-clockwise resolution. Works in
 * place.
 *
 * @param ijk The ijk coordinates.
 */
pub fn _downAp7(ijk: &mut CoordIJK) {
    // res r unit vectors in res r+1
    let mut iVec: CoordIJK = CoordIJK { i: 3, j: 0, k: 1 };
    let mut jVec: CoordIJK = CoordIJK { i: 1, j: 3, k: 0 };
    let mut kVec: CoordIJK = CoordIJK { i: 0, j: 1, k: 3 };

    _ijkScale(&mut iVec, ijk.i);
    _ijkScale(&mut jVec, ijk.j);
    _ijkScale(&mut kVec, ijk.k);

    _ijkAdd(iVec, jVec, ijk);
    _ijkAdd(*ijk, kVec, ijk);

    _ijkNormalize(ijk);
}

/**
 * Find the normalized ijk coordinates of the hex centered on the indicated
 * hex at the next finer aperture 7 clockwise resolution. Works in place.
 *
 * @param ijk The ijk coordinates.
 */
pub fn _downAp7r(ijk: &mut CoordIJK) {
    // res r unit vectors in res r+1
    let mut iVec: CoordIJK = CoordIJK { i: 3, j: 1, k: 0 };
    let mut jVec: CoordIJK = CoordIJK { i: 0, j: 3, k: 1 };
    let mut kVec: CoordIJK = CoordIJK { i: 1, j: 0, k: 3 };

    _ijkScale(&mut iVec, ijk.i);
    _ijkScale(&mut jVec, ijk.j);
    _ijkScale(&mut kVec, ijk.k);

    _ijkAdd(iVec, jVec, ijk);
    _ijkAdd(*ijk, kVec, ijk);

    _ijkNormalize(ijk);
}

/**
 * Find the normalized ijk coordinates of the hex in the specified digit
 * direction from the specified ijk coordinates. Works in place.
 *
 * @param ijk The ijk coordinates.
 * @param digit The digit direction from the original ijk coordinates.
 */
pub fn _neighbor(ijk: &mut CoordIJK, digit: Direction) {
    if digit > Direction::CenterDigit && digit < Direction::NUM_DIGITS {
        _ijkAdd(*ijk, UNIT_VECS[digit as usize], ijk);
        _ijkNormalize(ijk);
    }
}

/**
 * Rotates ijk coordinates 60 degrees counter-clockwise. Works in place.
 *
 * @param ijk The ijk coordinates.
 */
pub fn _ijkRotate60ccw(ijk: &mut CoordIJK) {
    // unit vector rotations
    let mut iVec = CoordIJK { i: 1, j: 1, k: 0 };
    let mut jVec = CoordIJK { i: 0, j: 1, k: 1 };
    let mut kVec = CoordIJK { i: 1, j: 0, k: 1 };

    _ijkScale(&mut iVec, ijk.i);
    _ijkScale(&mut jVec, ijk.j);
    _ijkScale(&mut kVec, ijk.k);

    _ijkAdd(iVec, jVec, ijk);
    _ijkAdd(*ijk, kVec, ijk);

    _ijkNormalize(ijk);
}

/**
 * Rotates ijk coordinates 60 degrees clockwise. Works in place.
 *
 * @param ijk The ijk coordinates.
 */
pub fn _ijkRotate60cw(ijk: &mut CoordIJK) {
    // unit vector rotations
    let mut iVec = CoordIJK { i: 1, j: 0, k: 1 };
    let mut jVec = CoordIJK { i: 1, j: 1, k: 0 };
    let mut kVec = CoordIJK { i: 0, j: 1, k: 1 };

    _ijkScale(&mut iVec, ijk.i);
    _ijkScale(&mut jVec, ijk.j);
    _ijkScale(&mut kVec, ijk.k);

    _ijkAdd(iVec, jVec, ijk);
    _ijkAdd(*ijk, kVec, ijk);

    _ijkNormalize(ijk);
}

/**
 * Rotates indexing digit 60 degrees clockwise. Returns result.
 *
 * @param digit Indexing digit (between 1 and 6 inclusive)
 */
pub fn _rotate60ccw(digit: Direction) -> Direction {
    return match digit {
        Direction::KAxesDigit => Direction::IKAxesDigit,
        Direction::IKAxesDigit => Direction::IAxesDigit,
        Direction::IAxesDigit => Direction::IJAxesDigit,
        Direction::IJAxesDigit => Direction::JAxesDigit,
        Direction::JAxesDigit => Direction::JKAxesDigit,
        Direction::JKAxesDigit => Direction::KAxesDigit,
        Direction::CenterDigit => digit,
        Direction::InvalidDigit => digit,
    };
}

/**
 * Rotates indexing digit 60 degrees clockwise. Returns result.
 *
 * @param digit Indexing digit (between 1 and 6 inclusive)
 */
pub fn _rotate60cw(digit: Direction) -> Direction {
    return match digit {
        Direction::KAxesDigit => Direction::JKAxesDigit,
        Direction::JKAxesDigit => Direction::JAxesDigit,
        Direction::JAxesDigit => Direction::IJAxesDigit,
        Direction::IJAxesDigit => Direction::IAxesDigit,
        Direction::IAxesDigit => Direction::IKAxesDigit,
        Direction::IKAxesDigit => Direction::KAxesDigit,
        Direction::CenterDigit => digit,
        Direction::InvalidDigit => digit,
    };
}
