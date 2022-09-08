use crate::{
    constants::H3_CELL_MODE,
    coord_ijk::CoordIJK,
    error::Error,
    face_ijk::FaceIJK,
    h3_index::{H3Index, H3_INIT, H3_SET_BASE_CELL, H3_SET_MODE},
};

/** @struct BaseCellData
 * @brief information on a single base cell
 */
#[derive(Copy, Clone)]
pub struct BaseCellData {
    pub homeFijk: FaceIJK, // < "home" face and normalized ijk coordinates on that face
    pub isPentagon: i32,   // < is this base cell a pentagon?
    pub cwOffsetPent: [i32; 2], // < if a pentagon, what are its two clockwise offset faces?
}

pub const INVALID_BASE_CELL: i32 = 127;

/** Maximum input for any component to face-to-base-cell lookup functions */
pub const MAX_FACE_COORD: i32 = 2;

/** @struct BaseCellOrient
 *  @brief base cell at a given ijk and required rotations into its system
 */
pub struct BaseCellOrient {
    pub baseCell: i32, // base cell number
    pub ccwRot60: i32, // number of ccw 60 degree rotations relative to current face
}

const NUM_ICOSA_FACES: i32 = 20;
/** The number of H3 base cells */
const NUM_BASE_CELLS: i32 = 122;

/** @brief Resolution 0 base cell lookup table for each face.
 *
 * Given the face number and a resolution 0 ijk+ coordinate in that face's
 * face-centered ijk coordinate system, gives the base cell located at that
 * coordinate and the number of 60 ccw rotations to rotate into that base
 * cell's orientation.
 *
 * Valid lookup coordinates are from (0, ccwRot60: 0, ccwRot60: 0) to (2, ccwRot60: 2, ccwRot60: 2).
 *
 * This table can be accessed using the functions `_faceIjkToBaseCell` and
 * `_faceIjkToBaseCellCCWrot60`
 */
pub const faceIjkBaseCells: [[[[BaseCellOrient; 3]; 3]; 3]; NUM_ICOSA_FACES as usize] = [
    [
        // face 0
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 16,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 18,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 24,
                    ccwRot60: 0,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 33,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 30,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 32,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 49,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 48,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 50,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 8,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 5,
                    ccwRot60: 5,
                },
                BaseCellOrient {
                    baseCell: 10,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 22,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 16,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 18,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 41,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 33,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 30,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 4,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 0,
                    ccwRot60: 5,
                },
                BaseCellOrient {
                    baseCell: 2,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 15,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 8,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 5,
                    ccwRot60: 5,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 31,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 22,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 16,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 1
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 2,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 6,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 14,
                    ccwRot60: 0,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 10,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 11,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 17,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 24,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 23,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 25,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 0,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 1,
                    ccwRot60: 5,
                },
                BaseCellOrient {
                    baseCell: 9,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 5,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 2,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 6,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 18,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 10,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 11,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 4,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 3,
                    ccwRot60: 5,
                },
                BaseCellOrient {
                    baseCell: 7,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 8,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 0,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 1,
                    ccwRot60: 5,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 16,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 5,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 2,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 2
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 7,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 21,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 38,
                    ccwRot60: 0,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 9,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 19,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 34,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 14,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 20,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 36,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 3,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 13,
                    ccwRot60: 5,
                },
                BaseCellOrient {
                    baseCell: 29,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 1,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 7,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 21,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 6,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 9,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 19,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 4,
                    ccwRot60: 2,
                },
                BaseCellOrient {
                    baseCell: 12,
                    ccwRot60: 5,
                },
                BaseCellOrient {
                    baseCell: 26,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 0,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 3,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 13,
                    ccwRot60: 5,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 2,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 1,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 7,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 3
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 26,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 42,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 58,
                    ccwRot60: 0,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 29,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 43,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 62,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 38,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 47,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 64,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 12,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 28,
                    ccwRot60: 5,
                },
                BaseCellOrient {
                    baseCell: 44,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 13,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 26,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 42,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 21,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 29,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 43,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 4,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 15,
                    ccwRot60: 5,
                },
                BaseCellOrient {
                    baseCell: 31,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 3,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 12,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 28,
                    ccwRot60: 5,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 7,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 13,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 26,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 4
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 31,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 41,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 49,
                    ccwRot60: 0,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 44,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 53,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 61,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 58,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 65,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 75,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 15,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 22,
                    ccwRot60: 5,
                },
                BaseCellOrient {
                    baseCell: 33,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 28,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 31,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 41,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 42,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 44,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 53,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 4,
                    ccwRot60: 4,
                },
                BaseCellOrient {
                    baseCell: 8,
                    ccwRot60: 5,
                },
                BaseCellOrient {
                    baseCell: 16,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 12,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 15,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 22,
                    ccwRot60: 5,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 26,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 28,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 31,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 5
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 50,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 48,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 49,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 32,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 30,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 33,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 24,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 18,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 16,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 70,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 67,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 66,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 52,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 50,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 48,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 37,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 32,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 30,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 83,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 87,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 85,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 74,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 70,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 67,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 57,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 52,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 50,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 6
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 25,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 23,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 24,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 17,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 11,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 10,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 14,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 6,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 2,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 45,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 39,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 37,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 35,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 25,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 23,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 27,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 17,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 11,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 63,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 59,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 57,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 56,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 45,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 39,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 46,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 35,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 25,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 7
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 36,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 20,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 14,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 34,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 19,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 9,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 38,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 21,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 7,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 55,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 40,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 27,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 54,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 36,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 20,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 51,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 34,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 19,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 72,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 60,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 46,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 73,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 55,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 40,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 71,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 54,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 36,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 8
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 64,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 47,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 38,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 62,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 43,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 29,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 58,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 42,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 26,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 84,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 69,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 51,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 82,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 64,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 47,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 76,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 62,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 43,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 97,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 89,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 71,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 98,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 84,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 69,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 96,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 82,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 64,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 9
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 75,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 65,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 58,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 61,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 53,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 44,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 49,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 41,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 31,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 94,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 86,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 76,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 81,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 75,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 65,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 66,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 61,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 53,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 107,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 104,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 96,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 101,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 94,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 86,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 85,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 81,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 75,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 10
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 57,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 59,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 63,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 74,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 78,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 79,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 83,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 92,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 95,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 37,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 39,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 45,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 52,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 57,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 59,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 70,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 74,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 78,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 24,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 23,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 25,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 32,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 37,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 39,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 50,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 52,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 57,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 11
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 46,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 60,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 72,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 56,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 68,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 80,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 63,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 77,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 90,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 27,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 40,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 55,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 35,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 46,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 60,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 45,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 56,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 68,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 14,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 20,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 36,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 17,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 27,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 40,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 25,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 35,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 46,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 12
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 71,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 89,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 97,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 73,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 91,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 103,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 72,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 88,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 105,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 51,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 69,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 84,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 54,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 71,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 89,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 55,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 73,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 91,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 38,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 47,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 64,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 34,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 51,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 69,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 36,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 54,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 71,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 13
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 96,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 104,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 107,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 98,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 110,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 115,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 97,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 111,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 119,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 76,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 86,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 94,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 82,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 96,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 104,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 84,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 98,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 110,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 58,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 65,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 75,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 62,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 76,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 86,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 64,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 82,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 96,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 14
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 85,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 87,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 83,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 101,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 102,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 100,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 107,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 112,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 114,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 66,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 67,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 70,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 81,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 85,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 87,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 94,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 101,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 102,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 49,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 48,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 50,
                    ccwRot60: 3,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 61,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 66,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 67,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 75,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 81,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 85,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 15
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 95,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 92,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 83,
                    ccwRot60: 0,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 79,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 78,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 74,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 63,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 59,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 57,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 109,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 108,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 100,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 93,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 95,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 92,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 77,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 79,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 78,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 117,
                    ccwRot60: 4,
                },
                BaseCellOrient {
                    baseCell: 118,
                    ccwRot60: 5,
                },
                BaseCellOrient {
                    baseCell: 114,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 106,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 109,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 108,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 90,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 93,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 95,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 16
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 90,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 77,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 63,
                    ccwRot60: 0,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 80,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 68,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 56,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 72,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 60,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 46,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 106,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 93,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 79,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 99,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 90,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 77,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 88,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 80,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 68,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 117,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 109,
                    ccwRot60: 5,
                },
                BaseCellOrient {
                    baseCell: 95,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 113,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 106,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 93,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 105,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 99,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 90,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 17
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 105,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 88,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 72,
                    ccwRot60: 0,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 103,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 91,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 73,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 97,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 89,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 71,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 113,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 99,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 80,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 116,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 105,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 88,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 111,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 103,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 91,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 117,
                    ccwRot60: 2,
                },
                BaseCellOrient {
                    baseCell: 106,
                    ccwRot60: 5,
                },
                BaseCellOrient {
                    baseCell: 90,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 121,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 113,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 99,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 119,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 116,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 105,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 18
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 119,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 111,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 97,
                    ccwRot60: 0,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 115,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 110,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 98,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 107,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 104,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 96,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 121,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 116,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 103,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 120,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 119,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 111,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 112,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 115,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 110,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 117,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 113,
                    ccwRot60: 5,
                },
                BaseCellOrient {
                    baseCell: 105,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 118,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 121,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 116,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 114,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 120,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 119,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
    [
        // face 19
        [
            // i 0
            [
                BaseCellOrient {
                    baseCell: 114,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 112,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 107,
                    ccwRot60: 0,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 100,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 102,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 101,
                    ccwRot60: 3,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 83,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 87,
                    ccwRot60: 3,
                },
                BaseCellOrient {
                    baseCell: 85,
                    ccwRot60: 3,
                },
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellOrient {
                    baseCell: 118,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 120,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 115,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 108,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 114,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 112,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 92,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 100,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 102,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellOrient {
                    baseCell: 117,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 121,
                    ccwRot60: 5,
                },
                BaseCellOrient {
                    baseCell: 119,
                    ccwRot60: 5,
                },
            ], // j 0
            [
                BaseCellOrient {
                    baseCell: 109,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 118,
                    ccwRot60: 0,
                },
                BaseCellOrient {
                    baseCell: 120,
                    ccwRot60: 0,
                },
            ], // j 1
            [
                BaseCellOrient {
                    baseCell: 95,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 108,
                    ccwRot60: 1,
                },
                BaseCellOrient {
                    baseCell: 114,
                    ccwRot60: 0,
                },
            ], // j 2
        ],
    ],
];

/** @brief Neighboring base cell ID in each IJK direction.
 *
 * For each base cell, for each direction, the neighboring base
 * cell ID is given. 127 indicates there is no neighbor in that direction.
 */
pub const baseCellNeighbors: [[i32; 7]; NUM_BASE_CELLS as usize] = [
    [0, 1, 5, 2, 4, 3, 8],                             // base cell 0
    [1, 7, 6, 9, 0, 3, 2],                             // base cell 1
    [2, 6, 10, 11, 0, 1, 5],                           // base cell 2
    [3, 13, 1, 7, 4, 12, 0],                           // base cell 3
    [4, INVALID_BASE_CELL, 15, 8, 3, 0, 12],           // base cell 4 (pentagon)
    [5, 2, 18, 10, 8, 0, 16],                          // base cell 5
    [6, 14, 11, 17, 1, 9, 2],                          // base cell 6
    [7, 21, 9, 19, 3, 13, 1],                          // base cell 7
    [8, 5, 22, 16, 4, 0, 15],                          // base cell 8
    [9, 19, 14, 20, 1, 7, 6],                          // base cell 9
    [10, 11, 24, 23, 5, 2, 18],                        // base cell 10
    [11, 17, 23, 25, 2, 6, 10],                        // base cell 11
    [12, 28, 13, 26, 4, 15, 3],                        // base cell 12
    [13, 26, 21, 29, 3, 12, 7],                        // base cell 13
    [14, INVALID_BASE_CELL, 17, 27, 9, 20, 6],         // base cell 14 (pentagon)
    [15, 22, 28, 31, 4, 8, 12],                        // base cell 15
    [16, 18, 33, 30, 8, 5, 22],                        // base cell 16
    [17, 11, 14, 6, 35, 25, 27],                       // base cell 17
    [18, 24, 30, 32, 5, 10, 16],                       // base cell 18
    [19, 34, 20, 36, 7, 21, 9],                        // base cell 19
    [20, 14, 19, 9, 40, 27, 36],                       // base cell 20
    [21, 38, 19, 34, 13, 29, 7],                       // base cell 21
    [22, 16, 41, 33, 15, 8, 31],                       // base cell 22
    [23, 24, 11, 10, 39, 37, 25],                      // base cell 23
    [24, INVALID_BASE_CELL, 32, 37, 10, 23, 18],       // base cell 24 (pentagon)
    [25, 23, 17, 11, 45, 39, 35],                      // base cell 25
    [26, 42, 29, 43, 12, 28, 13],                      // base cell 26
    [27, 40, 35, 46, 14, 20, 17],                      // base cell 27
    [28, 31, 42, 44, 12, 15, 26],                      // base cell 28
    [29, 43, 38, 47, 13, 26, 21],                      // base cell 29
    [30, 32, 48, 50, 16, 18, 33],                      // base cell 30
    [31, 41, 44, 53, 15, 22, 28],                      // base cell 31
    [32, 30, 24, 18, 52, 50, 37],                      // base cell 32
    [33, 30, 49, 48, 22, 16, 41],                      // base cell 33
    [34, 19, 38, 21, 54, 36, 51],                      // base cell 34
    [35, 46, 45, 56, 17, 27, 25],                      // base cell 35
    [36, 20, 34, 19, 55, 40, 54],                      // base cell 36
    [37, 39, 52, 57, 24, 23, 32],                      // base cell 37
    [38, INVALID_BASE_CELL, 34, 51, 29, 47, 21],       // base cell 38 (pentagon)
    [39, 37, 25, 23, 59, 57, 45],                      // base cell 39
    [40, 27, 36, 20, 60, 46, 55],                      // base cell 40
    [41, 49, 53, 61, 22, 33, 31],                      // base cell 41
    [42, 58, 43, 62, 28, 44, 26],                      // base cell 42
    [43, 62, 47, 64, 26, 42, 29],                      // base cell 43
    [44, 53, 58, 65, 28, 31, 42],                      // base cell 44
    [45, 39, 35, 25, 63, 59, 56],                      // base cell 45
    [46, 60, 56, 68, 27, 40, 35],                      // base cell 46
    [47, 38, 43, 29, 69, 51, 64],                      // base cell 47
    [48, 49, 30, 33, 67, 66, 50],                      // base cell 48
    [49, INVALID_BASE_CELL, 61, 66, 33, 48, 41],       // base cell 49 (pentagon)
    [50, 48, 32, 30, 70, 67, 52],                      // base cell 50
    [51, 69, 54, 71, 38, 47, 34],                      // base cell 51
    [52, 57, 70, 74, 32, 37, 50],                      // base cell 52
    [53, 61, 65, 75, 31, 41, 44],                      // base cell 53
    [54, 71, 55, 73, 34, 51, 36],                      // base cell 54
    [55, 40, 54, 36, 72, 60, 73],                      // base cell 55
    [56, 68, 63, 77, 35, 46, 45],                      // base cell 56
    [57, 59, 74, 78, 37, 39, 52],                      // base cell 57
    [58, INVALID_BASE_CELL, 62, 76, 44, 65, 42],       // base cell 58 (pentagon)
    [59, 63, 78, 79, 39, 45, 57],                      // base cell 59
    [60, 72, 68, 80, 40, 55, 46],                      // base cell 60
    [61, 53, 49, 41, 81, 75, 66],                      // base cell 61
    [62, 43, 58, 42, 82, 64, 76],                      // base cell 62
    [63, INVALID_BASE_CELL, 56, 45, 79, 59, 77],       // base cell 63 (pentagon)
    [64, 47, 62, 43, 84, 69, 82],                      // base cell 64
    [65, 58, 53, 44, 86, 76, 75],                      // base cell 65
    [66, 67, 81, 85, 49, 48, 61],                      // base cell 66
    [67, 66, 50, 48, 87, 85, 70],                      // base cell 67
    [68, 56, 60, 46, 90, 77, 80],                      // base cell 68
    [69, 51, 64, 47, 89, 71, 84],                      // base cell 69
    [70, 67, 52, 50, 83, 87, 74],                      // base cell 70
    [71, 89, 73, 91, 51, 69, 54],                      // base cell 71
    [72, INVALID_BASE_CELL, 73, 55, 80, 60, 88],       // base cell 72 (pentagon)
    [73, 91, 72, 88, 54, 71, 55],                      // base cell 73
    [74, 78, 83, 92, 52, 57, 70],                      // base cell 74
    [75, 65, 61, 53, 94, 86, 81],                      // base cell 75
    [76, 86, 82, 96, 58, 65, 62],                      // base cell 76
    [77, 63, 68, 56, 93, 79, 90],                      // base cell 77
    [78, 74, 59, 57, 95, 92, 79],                      // base cell 78
    [79, 78, 63, 59, 93, 95, 77],                      // base cell 79
    [80, 68, 72, 60, 99, 90, 88],                      // base cell 80
    [81, 85, 94, 101, 61, 66, 75],                     // base cell 81
    [82, 96, 84, 98, 62, 76, 64],                      // base cell 82
    [83, INVALID_BASE_CELL, 74, 70, 100, 87, 92],      // base cell 83 (pentagon)
    [84, 69, 82, 64, 97, 89, 98],                      // base cell 84
    [85, 87, 101, 102, 66, 67, 81],                    // base cell 85
    [86, 76, 75, 65, 104, 96, 94],                     // base cell 86
    [87, 83, 102, 100, 67, 70, 85],                    // base cell 87
    [88, 72, 91, 73, 99, 80, 105],                     // base cell 88
    [89, 97, 91, 103, 69, 84, 71],                     // base cell 89
    [90, 77, 80, 68, 106, 93, 99],                     // base cell 90
    [91, 73, 89, 71, 105, 88, 103],                    // base cell 91
    [92, 83, 78, 74, 108, 100, 95],                    // base cell 92
    [93, 79, 90, 77, 109, 95, 106],                    // base cell 93
    [94, 86, 81, 75, 107, 104, 101],                   // base cell 94
    [95, 92, 79, 78, 109, 108, 93],                    // base cell 95
    [96, 104, 98, 110, 76, 86, 82],                    // base cell 96
    [97, INVALID_BASE_CELL, 98, 84, 103, 89, 111],     // base cell 97 (pentagon)
    [98, 110, 97, 111, 82, 96, 84],                    // base cell 98
    [99, 80, 105, 88, 106, 90, 113],                   // base cell 99
    [100, 102, 83, 87, 108, 114, 92],                  // base cell 100
    [101, 102, 107, 112, 81, 85, 94],                  // base cell 101
    [102, 101, 87, 85, 114, 112, 100],                 // base cell 102
    [103, 91, 97, 89, 116, 105, 111],                  // base cell 103
    [104, 107, 110, 115, 86, 94, 96],                  // base cell 104
    [105, 88, 103, 91, 113, 99, 116],                  // base cell 105
    [106, 93, 99, 90, 117, 109, 113],                  // base cell 106
    [107, INVALID_BASE_CELL, 101, 94, 115, 104, 112],  // base cell 107 (pentagon)
    [108, 100, 95, 92, 118, 114, 109],                 // base cell 108
    [109, 108, 93, 95, 117, 118, 106],                 // base cell 109
    [110, 98, 104, 96, 119, 111, 115],                 // base cell 110
    [111, 97, 110, 98, 116, 103, 119],                 // base cell 111
    [112, 107, 102, 101, 120, 115, 114],               // base cell 112
    [113, 99, 116, 105, 117, 106, 121],                // base cell 113
    [114, 112, 100, 102, 118, 120, 108],               // base cell 114
    [115, 110, 107, 104, 120, 119, 112],               // base cell 115
    [116, 103, 119, 111, 113, 105, 121],               // base cell 116
    [117, INVALID_BASE_CELL, 109, 118, 113, 121, 106], // base cell 117 (pentagon)
    [118, 120, 108, 114, 117, 121, 109],               // base cell 118
    [119, 111, 115, 110, 121, 116, 120],               // base cell 119
    [120, 115, 114, 112, 121, 119, 118],               // base cell 120
    [121, 116, 120, 119, 117, 113, 118],               // base cell 121
];

/** @brief Neighboring base cell rotations in each IJK direction.
 *
 * For each base cell, for each direction, the number of 60 degree
 * CCW rotations to the coordinate system of the neighbor is given.
 * -1 indicates there is no neighbor in that direction.
 */
pub const baseCellNeighbor60CCWRots: [[i32; 7]; NUM_BASE_CELLS as usize] = [
    [0, 5, 0, 0, 1, 5, 1],  // base cell 0
    [0, 0, 1, 0, 1, 0, 1],  // base cell 1
    [0, 0, 0, 0, 0, 5, 0],  // base cell 2
    [0, 5, 0, 0, 2, 5, 1],  // base cell 3
    [0, -1, 1, 0, 3, 4, 2], // base cell 4 (pentagon)
    [0, 0, 1, 0, 1, 0, 1],  // base cell 5
    [0, 0, 0, 3, 5, 5, 0],  // base cell 6
    [0, 0, 0, 0, 0, 5, 0],  // base cell 7
    [0, 5, 0, 0, 0, 5, 1],  // base cell 8
    [0, 0, 1, 3, 0, 0, 1],  // base cell 9
    [0, 0, 1, 3, 0, 0, 1],  // base cell 10
    [0, 3, 3, 3, 0, 0, 0],  // base cell 11
    [0, 5, 0, 0, 3, 5, 1],  // base cell 12
    [0, 0, 1, 0, 1, 0, 1],  // base cell 13
    [0, -1, 3, 0, 5, 2, 0], // base cell 14 (pentagon)
    [0, 5, 0, 0, 4, 5, 1],  // base cell 15
    [0, 0, 0, 0, 0, 5, 0],  // base cell 16
    [0, 3, 3, 3, 3, 0, 3],  // base cell 17
    [0, 0, 0, 3, 5, 5, 0],  // base cell 18
    [0, 3, 3, 3, 0, 0, 0],  // base cell 19
    [0, 3, 3, 3, 0, 3, 0],  // base cell 20
    [0, 0, 0, 3, 5, 5, 0],  // base cell 21
    [0, 0, 1, 0, 1, 0, 1],  // base cell 22
    [0, 3, 3, 3, 0, 3, 0],  // base cell 23
    [0, -1, 3, 0, 5, 2, 0], // base cell 24 (pentagon)
    [0, 0, 0, 3, 0, 0, 3],  // base cell 25
    [0, 0, 0, 0, 0, 5, 0],  // base cell 26
    [0, 3, 0, 0, 0, 3, 3],  // base cell 27
    [0, 0, 1, 0, 1, 0, 1],  // base cell 28
    [0, 0, 1, 3, 0, 0, 1],  // base cell 29
    [0, 3, 3, 3, 0, 0, 0],  // base cell 30
    [0, 0, 0, 0, 0, 5, 0],  // base cell 31
    [0, 3, 3, 3, 3, 0, 3],  // base cell 32
    [0, 0, 1, 3, 0, 0, 1],  // base cell 33
    [0, 3, 3, 3, 3, 0, 3],  // base cell 34
    [0, 0, 3, 0, 3, 0, 3],  // base cell 35
    [0, 0, 0, 3, 0, 0, 3],  // base cell 36
    [0, 3, 0, 0, 0, 3, 3],  // base cell 37
    [0, -1, 3, 0, 5, 2, 0], // base cell 38 (pentagon)
    [0, 3, 0, 0, 3, 3, 0],  // base cell 39
    [0, 3, 0, 0, 3, 3, 0],  // base cell 40
    [0, 0, 0, 3, 5, 5, 0],  // base cell 41
    [0, 0, 0, 3, 5, 5, 0],  // base cell 42
    [0, 3, 3, 3, 0, 0, 0],  // base cell 43
    [0, 0, 1, 3, 0, 0, 1],  // base cell 44
    [0, 0, 3, 0, 0, 3, 3],  // base cell 45
    [0, 0, 0, 3, 0, 3, 0],  // base cell 46
    [0, 3, 3, 3, 0, 3, 0],  // base cell 47
    [0, 3, 3, 3, 0, 3, 0],  // base cell 48
    [0, -1, 3, 0, 5, 2, 0], // base cell 49 (pentagon)
    [0, 0, 0, 3, 0, 0, 3],  // base cell 50
    [0, 3, 0, 0, 0, 3, 3],  // base cell 51
    [0, 0, 3, 0, 3, 0, 3],  // base cell 52
    [0, 3, 3, 3, 0, 0, 0],  // base cell 53
    [0, 0, 3, 0, 3, 0, 3],  // base cell 54
    [0, 0, 3, 0, 0, 3, 3],  // base cell 55
    [0, 3, 3, 3, 0, 0, 3],  // base cell 56
    [0, 0, 0, 3, 0, 3, 0],  // base cell 57
    [0, -1, 3, 0, 5, 2, 0], // base cell 58 (pentagon)
    [0, 3, 3, 3, 3, 3, 0],  // base cell 59
    [0, 3, 3, 3, 3, 3, 0],  // base cell 60
    [0, 3, 3, 3, 3, 0, 3],  // base cell 61
    [0, 3, 3, 3, 3, 0, 3],  // base cell 62
    [0, -1, 3, 0, 5, 2, 0], // base cell 63 (pentagon)
    [0, 0, 0, 3, 0, 0, 3],  // base cell 64
    [0, 3, 3, 3, 0, 3, 0],  // base cell 65
    [0, 3, 0, 0, 0, 3, 3],  // base cell 66
    [0, 3, 0, 0, 3, 3, 0],  // base cell 67
    [0, 3, 3, 3, 0, 0, 0],  // base cell 68
    [0, 3, 0, 0, 3, 3, 0],  // base cell 69
    [0, 0, 3, 0, 0, 3, 3],  // base cell 70
    [0, 0, 0, 3, 0, 3, 0],  // base cell 71
    [0, -1, 3, 0, 5, 2, 0], // base cell 72 (pentagon)
    [0, 3, 3, 3, 0, 0, 3],  // base cell 73
    [0, 3, 3, 3, 0, 0, 3],  // base cell 74
    [0, 0, 0, 3, 0, 0, 3],  // base cell 75
    [0, 3, 0, 0, 0, 3, 3],  // base cell 76
    [0, 0, 0, 3, 0, 5, 0],  // base cell 77
    [0, 3, 3, 3, 0, 0, 0],  // base cell 78
    [0, 0, 1, 3, 1, 0, 1],  // base cell 79
    [0, 0, 1, 3, 1, 0, 1],  // base cell 80
    [0, 0, 3, 0, 3, 0, 3],  // base cell 81
    [0, 0, 3, 0, 3, 0, 3],  // base cell 82
    [0, -1, 3, 0, 5, 2, 0], // base cell 83 (pentagon)
    [0, 0, 3, 0, 0, 3, 3],  // base cell 84
    [0, 0, 0, 3, 0, 3, 0],  // base cell 85
    [0, 3, 0, 0, 3, 3, 0],  // base cell 86
    [0, 3, 3, 3, 3, 3, 0],  // base cell 87
    [0, 0, 0, 3, 0, 5, 0],  // base cell 88
    [0, 3, 3, 3, 3, 3, 0],  // base cell 89
    [0, 0, 0, 0, 0, 0, 1],  // base cell 90
    [0, 3, 3, 3, 0, 0, 0],  // base cell 91
    [0, 0, 0, 3, 0, 5, 0],  // base cell 92
    [0, 5, 0, 0, 5, 5, 0],  // base cell 93
    [0, 0, 3, 0, 0, 3, 3],  // base cell 94
    [0, 0, 0, 0, 0, 0, 1],  // base cell 95
    [0, 0, 0, 3, 0, 3, 0],  // base cell 96
    [0, -1, 3, 0, 5, 2, 0], // base cell 97 (pentagon)
    [0, 3, 3, 3, 0, 0, 3],  // base cell 98
    [0, 5, 0, 0, 5, 5, 0],  // base cell 99
    [0, 0, 1, 3, 1, 0, 1],  // base cell 100
    [0, 3, 3, 3, 0, 0, 3],  // base cell 101
    [0, 3, 3, 3, 0, 0, 0],  // base cell 102
    [0, 0, 1, 3, 1, 0, 1],  // base cell 103
    [0, 3, 3, 3, 3, 3, 0],  // base cell 104
    [0, 0, 0, 0, 0, 0, 1],  // base cell 105
    [0, 0, 1, 0, 3, 5, 1],  // base cell 106
    [0, -1, 3, 0, 5, 2, 0], // base cell 107 (pentagon)
    [0, 5, 0, 0, 5, 5, 0],  // base cell 108
    [0, 0, 1, 0, 4, 5, 1],  // base cell 109
    [0, 3, 3, 3, 0, 0, 0],  // base cell 110
    [0, 0, 0, 3, 0, 5, 0],  // base cell 111
    [0, 0, 0, 3, 0, 5, 0],  // base cell 112
    [0, 0, 1, 0, 2, 5, 1],  // base cell 113
    [0, 0, 0, 0, 0, 0, 1],  // base cell 114
    [0, 0, 1, 3, 1, 0, 1],  // base cell 115
    [0, 5, 0, 0, 5, 5, 0],  // base cell 116
    [0, -1, 1, 0, 3, 4, 2], // base cell 117 (pentagon)
    [0, 0, 1, 0, 0, 5, 1],  // base cell 118
    [0, 0, 0, 0, 0, 0, 1],  // base cell 119
    [0, 5, 0, 0, 5, 5, 0],  // base cell 120
    [0, 0, 1, 0, 1, 5, 1],  // base cell 121
];

/** @brief Resolution 0 base cell data table.
 *
 * For each base cell, gives the "home" face and ijk+ coordinates on that face,
 * whether or not the base cell is a pentagon. Additionally, if the base cell
 * is a pentagon, the two cw offset rotation adjacent faces are given (-1
 * indicates that no cw offset rotation faces exist for this base cell).
 */
pub const baseCellData: [BaseCellData; NUM_BASE_CELLS as usize] = [
    BaseCellData {
        homeFijk: FaceIJK {
            face: 1,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 0
    BaseCellData {
        homeFijk: FaceIJK {
            face: 2,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 1
    BaseCellData {
        homeFijk: FaceIJK {
            face: 1,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 2
    BaseCellData {
        homeFijk: FaceIJK {
            face: 2,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 3
    BaseCellData {
        homeFijk: FaceIJK {
            face: 0,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        isPentagon: 1,
        cwOffsetPent: [-1, -1],
    }, // base cell 4
    BaseCellData {
        homeFijk: FaceIJK {
            face: 1,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 5
    BaseCellData {
        homeFijk: FaceIJK {
            face: 1,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 6
    BaseCellData {
        homeFijk: FaceIJK {
            face: 2,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 7
    BaseCellData {
        homeFijk: FaceIJK {
            face: 0,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 8
    BaseCellData {
        homeFijk: FaceIJK {
            face: 2,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 9
    BaseCellData {
        homeFijk: FaceIJK {
            face: 1,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 10
    BaseCellData {
        homeFijk: FaceIJK {
            face: 1,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 11
    BaseCellData {
        homeFijk: FaceIJK {
            face: 3,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 12
    BaseCellData {
        homeFijk: FaceIJK {
            face: 3,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 13
    BaseCellData {
        homeFijk: FaceIJK {
            face: 11,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        isPentagon: 1,
        cwOffsetPent: [2, 6],
    }, // base cell 14
    BaseCellData {
        homeFijk: FaceIJK {
            face: 4,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 15
    BaseCellData {
        homeFijk: FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 16
    BaseCellData {
        homeFijk: FaceIJK {
            face: 6,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 17
    BaseCellData {
        homeFijk: FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 18
    BaseCellData {
        homeFijk: FaceIJK {
            face: 2,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 19
    BaseCellData {
        homeFijk: FaceIJK {
            face: 7,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 20
    BaseCellData {
        homeFijk: FaceIJK {
            face: 2,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 21
    BaseCellData {
        homeFijk: FaceIJK {
            face: 0,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 22
    BaseCellData {
        homeFijk: FaceIJK {
            face: 6,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 23
    BaseCellData {
        homeFijk: FaceIJK {
            face: 10,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        isPentagon: 1,
        cwOffsetPent: [1, 5],
    }, // base cell 24
    BaseCellData {
        homeFijk: FaceIJK {
            face: 6,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 25
    BaseCellData {
        homeFijk: FaceIJK {
            face: 3,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 26
    BaseCellData {
        homeFijk: FaceIJK {
            face: 11,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 27
    BaseCellData {
        homeFijk: FaceIJK {
            face: 4,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 28
    BaseCellData {
        homeFijk: FaceIJK {
            face: 3,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 29
    BaseCellData {
        homeFijk: FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 30
    BaseCellData {
        homeFijk: FaceIJK {
            face: 4,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 31
    BaseCellData {
        homeFijk: FaceIJK {
            face: 5,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 32
    BaseCellData {
        homeFijk: FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 33
    BaseCellData {
        homeFijk: FaceIJK {
            face: 7,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 34
    BaseCellData {
        homeFijk: FaceIJK {
            face: 11,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 35
    BaseCellData {
        homeFijk: FaceIJK {
            face: 7,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 36
    BaseCellData {
        homeFijk: FaceIJK {
            face: 10,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 37
    BaseCellData {
        homeFijk: FaceIJK {
            face: 12,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        isPentagon: 1,
        cwOffsetPent: [3, 7],
    }, // base cell 38
    BaseCellData {
        homeFijk: FaceIJK {
            face: 6,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 39
    BaseCellData {
        homeFijk: FaceIJK {
            face: 7,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 40
    BaseCellData {
        homeFijk: FaceIJK {
            face: 4,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 41
    BaseCellData {
        homeFijk: FaceIJK {
            face: 3,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 42
    BaseCellData {
        homeFijk: FaceIJK {
            face: 3,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 43
    BaseCellData {
        homeFijk: FaceIJK {
            face: 4,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 44
    BaseCellData {
        homeFijk: FaceIJK {
            face: 6,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 45
    BaseCellData {
        homeFijk: FaceIJK {
            face: 11,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 46
    BaseCellData {
        homeFijk: FaceIJK {
            face: 8,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 47
    BaseCellData {
        homeFijk: FaceIJK {
            face: 5,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 48
    BaseCellData {
        homeFijk: FaceIJK {
            face: 14,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        isPentagon: 1,
        cwOffsetPent: [0, 9],
    }, // base cell 49
    BaseCellData {
        homeFijk: FaceIJK {
            face: 5,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 50
    BaseCellData {
        homeFijk: FaceIJK {
            face: 12,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 51
    BaseCellData {
        homeFijk: FaceIJK {
            face: 10,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 52
    BaseCellData {
        homeFijk: FaceIJK {
            face: 4,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 53
    BaseCellData {
        homeFijk: FaceIJK {
            face: 12,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 54
    BaseCellData {
        homeFijk: FaceIJK {
            face: 7,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 55
    BaseCellData {
        homeFijk: FaceIJK {
            face: 11,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 56
    BaseCellData {
        homeFijk: FaceIJK {
            face: 10,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 57
    BaseCellData {
        homeFijk: FaceIJK {
            face: 13,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        isPentagon: 1,
        cwOffsetPent: [4, 8],
    }, // base cell 58
    BaseCellData {
        homeFijk: FaceIJK {
            face: 10,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 59
    BaseCellData {
        homeFijk: FaceIJK {
            face: 11,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 60
    BaseCellData {
        homeFijk: FaceIJK {
            face: 9,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 61
    BaseCellData {
        homeFijk: FaceIJK {
            face: 8,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 62
    BaseCellData {
        homeFijk: FaceIJK {
            face: 6,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        isPentagon: 1,
        cwOffsetPent: [11, 15],
    }, // base cell 63
    BaseCellData {
        homeFijk: FaceIJK {
            face: 8,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 64
    BaseCellData {
        homeFijk: FaceIJK {
            face: 9,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 65
    BaseCellData {
        homeFijk: FaceIJK {
            face: 14,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 66
    BaseCellData {
        homeFijk: FaceIJK {
            face: 5,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 67
    BaseCellData {
        homeFijk: FaceIJK {
            face: 16,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 68
    BaseCellData {
        homeFijk: FaceIJK {
            face: 8,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 69
    BaseCellData {
        homeFijk: FaceIJK {
            face: 5,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 70
    BaseCellData {
        homeFijk: FaceIJK {
            face: 12,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 71
    BaseCellData {
        homeFijk: FaceIJK {
            face: 7,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        isPentagon: 1,
        cwOffsetPent: [12, 16],
    }, // base cell 72
    BaseCellData {
        homeFijk: FaceIJK {
            face: 12,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 73
    BaseCellData {
        homeFijk: FaceIJK {
            face: 10,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 74
    BaseCellData {
        homeFijk: FaceIJK {
            face: 9,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 75
    BaseCellData {
        homeFijk: FaceIJK {
            face: 13,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 76
    BaseCellData {
        homeFijk: FaceIJK {
            face: 16,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 77
    BaseCellData {
        homeFijk: FaceIJK {
            face: 15,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 78
    BaseCellData {
        homeFijk: FaceIJK {
            face: 15,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 79
    BaseCellData {
        homeFijk: FaceIJK {
            face: 16,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 80
    BaseCellData {
        homeFijk: FaceIJK {
            face: 14,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 81
    BaseCellData {
        homeFijk: FaceIJK {
            face: 13,
            coord: CoordIJK { i: 1, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 82
    BaseCellData {
        homeFijk: FaceIJK {
            face: 5,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        isPentagon: 1,
        cwOffsetPent: [10, 19],
    }, // base cell 83
    BaseCellData {
        homeFijk: FaceIJK {
            face: 8,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 84
    BaseCellData {
        homeFijk: FaceIJK {
            face: 14,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 85
    BaseCellData {
        homeFijk: FaceIJK {
            face: 9,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 86
    BaseCellData {
        homeFijk: FaceIJK {
            face: 14,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 87
    BaseCellData {
        homeFijk: FaceIJK {
            face: 17,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 88
    BaseCellData {
        homeFijk: FaceIJK {
            face: 12,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 89
    BaseCellData {
        homeFijk: FaceIJK {
            face: 16,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 90
    BaseCellData {
        homeFijk: FaceIJK {
            face: 17,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 91
    BaseCellData {
        homeFijk: FaceIJK {
            face: 15,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 92
    BaseCellData {
        homeFijk: FaceIJK {
            face: 16,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 93
    BaseCellData {
        homeFijk: FaceIJK {
            face: 9,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 94
    BaseCellData {
        homeFijk: FaceIJK {
            face: 15,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 95
    BaseCellData {
        homeFijk: FaceIJK {
            face: 13,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 96
    BaseCellData {
        homeFijk: FaceIJK {
            face: 8,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        isPentagon: 1,
        cwOffsetPent: [13, 17],
    }, // base cell 97
    BaseCellData {
        homeFijk: FaceIJK {
            face: 13,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 98
    BaseCellData {
        homeFijk: FaceIJK {
            face: 17,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 99
    BaseCellData {
        homeFijk: FaceIJK {
            face: 19,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 100
    BaseCellData {
        homeFijk: FaceIJK {
            face: 14,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 101
    BaseCellData {
        homeFijk: FaceIJK {
            face: 19,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 102
    BaseCellData {
        homeFijk: FaceIJK {
            face: 17,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 103
    BaseCellData {
        homeFijk: FaceIJK {
            face: 13,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 104
    BaseCellData {
        homeFijk: FaceIJK {
            face: 17,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 105
    BaseCellData {
        homeFijk: FaceIJK {
            face: 16,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 106
    BaseCellData {
        homeFijk: FaceIJK {
            face: 9,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        isPentagon: 1,
        cwOffsetPent: [14, 18],
    }, // base cell 107
    BaseCellData {
        homeFijk: FaceIJK {
            face: 15,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 108
    BaseCellData {
        homeFijk: FaceIJK {
            face: 15,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 109
    BaseCellData {
        homeFijk: FaceIJK {
            face: 18,
            coord: CoordIJK { i: 0, j: 1, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 110
    BaseCellData {
        homeFijk: FaceIJK {
            face: 18,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 111
    BaseCellData {
        homeFijk: FaceIJK {
            face: 19,
            coord: CoordIJK { i: 0, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 112
    BaseCellData {
        homeFijk: FaceIJK {
            face: 17,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 113
    BaseCellData {
        homeFijk: FaceIJK {
            face: 19,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 114
    BaseCellData {
        homeFijk: FaceIJK {
            face: 18,
            coord: CoordIJK { i: 0, j: 1, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 115
    BaseCellData {
        homeFijk: FaceIJK {
            face: 18,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 116
    BaseCellData {
        homeFijk: FaceIJK {
            face: 19,
            coord: CoordIJK { i: 2, j: 0, k: 0 },
        },
        isPentagon: 1,
        cwOffsetPent: [-1, -1],
    }, // base cell 117
    BaseCellData {
        homeFijk: FaceIJK {
            face: 19,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 118
    BaseCellData {
        homeFijk: FaceIJK {
            face: 18,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 119
    BaseCellData {
        homeFijk: FaceIJK {
            face: 19,
            coord: CoordIJK { i: 1, j: 0, k: 1 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 120
    BaseCellData {
        homeFijk: FaceIJK {
            face: 18,
            coord: CoordIJK { i: 1, j: 0, k: 0 },
        },
        isPentagon: 0,
        cwOffsetPent: [0, 0],
    }, // base cell 121
];

/** @brief Return whether or not the indicated base cell is a pentagon. */
pub fn _isBaseCellPentagon(baseCell: i32) -> bool {
    if baseCell < 0 || baseCell >= NUM_BASE_CELLS {
        // Base cells less than zero can not be represented in an index
        return false;
    }
    return baseCellData[baseCell as usize].isPentagon != 0;
}

/** @brief Return whether the indicated base cell is a pentagon where all
 * neighbors are oriented towards it. */
pub fn _isBaseCellPolarPentagon(baseCell: i32) -> bool {
    return baseCell == 4 || baseCell == 117;
}

/** @brief Find base cell given FaceIJK.
 *
 * Given the face number and a resolution 0 ijk+ coordinate in that face's
 * face-centered ijk coordinate system, return the base cell located at that
 * coordinate.
 *
 * Valid ijk+ lookup coordinates are from (0, 0, 0) to (2, 2, 2).
 */
pub fn _faceIjkToBaseCell(h: &FaceIJK) -> i32 {
    return faceIjkBaseCells[h.face as usize][h.coord.i as usize][h.coord.j as usize]
        [h.coord.k as usize]
        .baseCell;
}

/** @brief Find base cell given FaceIJK.
 *
 * Given the face number and a resolution 0 ijk+ coordinate in that face's
 * face-centered ijk coordinate system, return the number of 60' ccw rotations
 * to rotate into the coordinate system of the base cell at that coordinates.
 *
 * Valid ijk+ lookup coordinates are from (0, 0, 0) to (2, 2, 2).
 */
pub fn _faceIjkToBaseCellCCWrot60(h: &FaceIJK) -> i32 {
    return faceIjkBaseCells[h.face as usize][h.coord.i as usize][h.coord.j as usize]
        [h.coord.k as usize]
        .ccwRot60;
}

/** @brief Return whether or not the tested face is a cw offset face.
 */
pub fn _baseCellIsCwOffset(baseCell: i32, testFace: i32) -> bool {
    return baseCellData[baseCell as usize].cwOffsetPent[0] == testFace
        || baseCellData[baseCell as usize].cwOffsetPent[1] == testFace;
}

/**
 * res0CellCount returns the number of resolution 0 cells
 *
 * @return int count of resolution 0 cells
 */
pub fn res0CellCount() -> i32 {
    return NUM_BASE_CELLS;
}

/**
 * getRes0Cells generates all base cells storing them into the provided
 * memory pointer. Buffer must be of size NUM_BASE_CELLS * sizeof(H3Index).
 *
 * @param out H3Index* the memory to store the resulting base cells in
 * @returns E_SUCCESS.
 */
pub fn getRes0Cells() -> Result<Vec<H3Index>, Error> {
    let mut out = Vec::<H3Index>::new();
    out.resize(NUM_BASE_CELLS as usize, 0);
    for bc in 0..(NUM_BASE_CELLS as usize) {
        //(int bc = 0; bc < NUM_BASE_CELLS; bc++) {
        let mut baseCell: H3Index = H3_INIT;
        H3_SET_MODE(&mut baseCell, H3_CELL_MODE);
        H3_SET_BASE_CELL(&mut baseCell, bc as i32);
        out[bc] = baseCell;
    }
    return Ok(out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getRes0Cells() {
        let count = res0CellCount();
        let indexes = super::getRes0Cells().unwrap();
        assert_eq!(indexes[0], 0x8001fffffffffff, "correct first basecell");
        assert_eq!(indexes[121], 0x80f3fffffffffff, "correct last basecell");
    }
}
