use crate::constants::*;
use crate::coord_ijk::{
    CoordIJK, _hex2dToCoordIJK, _ijkAdd, _ijkNormalize, _ijkRotate60ccw, _ijkRotate60cw, _ijkScale,
    _ijkSub, _ijkToHex2d, _setIJK,
};
use crate::h3_index::isResolutionClassIII;
use crate::lat_lng::{LatLng, _geoAzDistanceRads, _geoAzimuthRads, _posAngleRads};
use crate::vec2d::{Vec2d, _v2dMag};
use crate::vec3d::{Vec3d, _geoToVec3d, _pointSquareDist};

// indexes for faceNeighbors table
/** IJ quadrant faceNeighbors table direction */
const IJ: usize = 1;
/** KI quadrant faceNeighbors table direction */
const KI: usize = 2;
/** JK quadrant faceNeighbors table direction */
const JK: usize = 3;

/** Invalid face index */
const INVALID_FACE: i32 = -1;

/** @brief icosahedron face centers in lat/lon radians */
const faceCenterGeo: [LatLng; NUM_ICOSA_FACES as usize] = [
    LatLng {
        lat: 0.803582649718989942,
        lng: 1.248397419617396099,
    }, // face  0
    LatLng {
        lat: 1.307747883455638156,
        lng: 2.536945009877921159,
    }, // face  1
    LatLng {
        lat: 1.054751253523952054,
        lng: -1.347517358900396623,
    }, // face  2
    LatLng {
        lat: 0.600191595538186799,
        lng: -0.450603909469755746,
    }, // face  3
    LatLng {
        lat: 0.491715428198773866,
        lng: 0.401988202911306943,
    }, // face  4
    LatLng {
        lat: 0.172745327415618701,
        lng: 1.678146885280433686,
    }, // face  5
    LatLng {
        lat: 0.605929321571350690,
        lng: 2.953923329812411617,
    }, // face  6
    LatLng {
        lat: 0.427370518328979641,
        lng: -1.888876200336285401,
    }, // face  7
    LatLng {
        lat: -0.079066118549212831,
        lng: -0.733429513380867741,
    }, // face  8
    LatLng {
        lat: -0.230961644455383637,
        lng: 0.506495587332349035,
    }, // face  9
    LatLng {
        lat: 0.079066118549212831,
        lng: 2.408163140208925497,
    }, // face 10
    LatLng {
        lat: 0.230961644455383637,
        lng: -2.635097066257444203,
    }, // face 11
    LatLng {
        lat: -0.172745327415618701,
        lng: -1.463445768309359553,
    }, // face 12
    LatLng {
        lat: -0.605929321571350690,
        lng: -0.187669323777381622,
    }, // face 13
    LatLng {
        lat: -0.427370518328979641,
        lng: 1.252716453253507838,
    }, // face 14
    LatLng {
        lat: -0.600191595538186799,
        lng: 2.690988744120037492,
    }, // face 15
    LatLng {
        lat: -0.491715428198773866,
        lng: -2.739604450678486295,
    }, // face 16
    LatLng {
        lat: -0.803582649718989942,
        lng: -1.893195233972397139,
    }, // face 17
    LatLng {
        lat: -1.307747883455638156,
        lng: -0.604647643711872080,
    }, // face 18
    LatLng {
        lat: -1.054751253523952054,
        lng: 1.794075294689396615,
    }, // face 19
];

/** @brief icosahedron face centers in x/y/z on the unit sphere */
const faceCenterPoint: [Vec3d; NUM_ICOSA_FACES as usize] = [
    Vec3d {
        x: 0.2199307791404606,
        y: 0.6583691780274996,
        z: 0.7198475378926182,
    }, // face  0
    Vec3d {
        x: -0.2139234834501421,
        y: 0.1478171829550703,
        z: 0.9656017935214205,
    }, // face  1
    Vec3d {
        x: 0.1092625278784797,
        y: -0.4811951572873210,
        z: 0.8697775121287253,
    }, // face  2
    Vec3d {
        x: 0.7428567301586791,
        y: -0.3593941678278028,
        z: 0.5648005936517033,
    }, // face  3
    Vec3d {
        x: 0.8112534709140969,
        y: 0.3448953237639384,
        z: 0.4721387736413930,
    }, // face  4
    Vec3d {
        x: -0.1055498149613921,
        y: 0.9794457296411413,
        z: 0.1718874610009365,
    }, // face  5
    Vec3d {
        x: -0.8075407579970092,
        y: 0.1533552485898818,
        z: 0.5695261994882688,
    }, // face  6
    Vec3d {
        x: -0.2846148069787907,
        y: -0.8644080972654206,
        z: 0.4144792552473539,
    }, // face  7
    Vec3d {
        x: 0.7405621473854482,
        y: -0.6673299564565524,
        z: -0.0789837646326737,
    }, // face  8
    Vec3d {
        x: 0.8512303986474293,
        y: 0.4722343788582681,
        z: -0.2289137388687808,
    }, // face  9
    Vec3d {
        x: -0.7405621473854481,
        y: 0.6673299564565524,
        z: 0.0789837646326737,
    }, // face 10
    Vec3d {
        x: -0.8512303986474292,
        y: -0.4722343788582682,
        z: 0.2289137388687808,
    }, // face 11
    Vec3d {
        x: 0.1055498149613919,
        y: -0.9794457296411413,
        z: -0.1718874610009365,
    }, // face 12
    Vec3d {
        x: 0.8075407579970092,
        y: -0.1533552485898819,
        z: -0.5695261994882688,
    }, // face 13
    Vec3d {
        x: 0.2846148069787908,
        y: 0.8644080972654204,
        z: -0.4144792552473539,
    }, // face 14
    Vec3d {
        x: -0.7428567301586791,
        y: 0.3593941678278027,
        z: -0.5648005936517033,
    }, // face 15
    Vec3d {
        x: -0.8112534709140971,
        y: -0.3448953237639382,
        z: -0.4721387736413930,
    }, // face 16
    Vec3d {
        x: -0.2199307791404607,
        y: -0.6583691780274996,
        z: -0.7198475378926182,
    }, // face 17
    Vec3d {
        x: 0.2139234834501420,
        y: -0.1478171829550704,
        z: -0.9656017935214205,
    }, // face 18
    Vec3d {
        x: -0.1092625278784796,
        y: 0.4811951572873210,
        z: -0.8697775121287253,
    }, // face 19
];

const faceAxesAzRadsCII: [[f64; 3]; NUM_ICOSA_FACES as usize] = [
    [
        5.619958268523939882,
        3.525563166130744542,
        1.431168063737548730,
    ], // face  0
    [
        5.760339081714187279,
        3.665943979320991689,
        1.571548876927796127,
    ], // face  1
    [
        0.780213654393430055,
        4.969003859179821079,
        2.874608756786625655,
    ], // face  2
    [
        0.430469363979999913,
        4.619259568766391033,
        2.524864466373195467,
    ], // face  3
    [
        6.130269123335111400,
        4.035874020941915804,
        1.941478918548720291,
    ], // face  4
    [
        2.692877706530642877,
        0.598482604137447119,
        4.787272808923838195,
    ], // face  5
    [
        2.982963003477243874,
        0.888567901084048369,
        5.077358105870439581,
    ], // face  6
    [
        3.532912002790141181,
        1.438516900396945656,
        5.627307105183336758,
    ], // face  7
    [
        3.494305004259568154,
        1.399909901866372864,
        5.588700106652763840,
    ], // face  8
    [
        3.003214169499538391,
        0.908819067106342928,
        5.097609271892733906,
    ], // face  9
    [
        5.930472956509811562,
        3.836077854116615875,
        1.741682751723420374,
    ], // face 10
    [
        0.138378484090254847,
        4.327168688876645809,
        2.232773586483450311,
    ], // face 11
    [
        0.448714947059150361,
        4.637505151845541521,
        2.543110049452346120,
    ], // face 12
    [
        0.158629650112549365,
        4.347419854898940135,
        2.253024752505744869,
    ], // face 13
    [
        5.891865957979238535,
        3.797470855586042958,
        1.703075753192847583,
    ], // face 14
    [
        2.711123289609793325,
        0.616728187216597771,
        4.805518392002988683,
    ], // face 15
    [
        3.294508837434268316,
        1.200113735041072948,
        5.388903939827463911,
    ], // face 16
    [
        3.804819692245439833,
        1.710424589852244509,
        5.899214794638635174,
    ], // face 17
    [
        3.664438879055192436,
        1.570043776661997111,
        5.758833981448388027,
    ], // face 18
    [
        2.361378999196363184,
        0.266983896803167583,
        4.455774101589558636,
    ], // face 19
];

/** @brief overage distance table */
const maxDimByCIIres: [i32; 17] = [
    2,        // res  0
    -1,       // res  1
    14,       // res  2
    -1,       // res  3
    98,       // res  4
    -1,       // res  5
    686,      // res  6
    -1,       // res  7
    4802,     // res  8
    -1,       // res  9
    33614,    // res 10
    -1,       // res 11
    235298,   // res 12
    -1,       // res 13
    1647086,  // res 14
    -1,       // res 15
    11529602, // res 16
];

/** @brief unit scale distance table */
const unitScaleByCIIres: [i32; 17] = [
    1,       // res  0
    -1,      // res  1
    7,       // res  2
    -1,      // res  3
    49,      // res  4
    -1,      // res  5
    343,     // res  6
    -1,      // res  7
    2401,    // res  8
    -1,      // res  9
    16807,   // res 10
    -1,      // res 11
    117649,  // res 12
    -1,      // res 13
    823543,  // res 14
    -1,      // res 15
    5764801, // res 16
];

/** @brief Definition of which faces neighbor each other. */
const faceNeighbors: [[FaceOrientIJK; 4]; NUM_ICOSA_FACES as usize] = [
    [
        // face 0
        FaceOrientIJK {
            face: 0,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 4,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 1,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 5,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 1
        FaceOrientIJK {
            face: 1,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 0,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 2,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 6,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 2
        FaceOrientIJK {
            face: 2,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 1,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 3,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 7,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 3
        FaceOrientIJK {
            face: 3,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 2,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 4,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 8,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 4
        FaceOrientIJK {
            face: 4,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 3,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 0,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 9,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 5
        FaceOrientIJK {
            face: 5,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 10,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 14,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 0,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 6
        FaceOrientIJK {
            face: 6,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 11,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 10,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 1,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 7
        FaceOrientIJK {
            face: 7,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 12,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 11,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 2,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 8
        FaceOrientIJK {
            face: 8,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 13,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 12,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 3,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 9
        FaceOrientIJK {
            face: 9,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 14,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 13,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 4,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 10
        FaceOrientIJK {
            face: 10,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 5,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 6,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 15,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 11
        FaceOrientIJK {
            face: 11,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 6,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 7,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 16,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 12
        FaceOrientIJK {
            face: 12,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 7,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 8,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 17,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 13
        FaceOrientIJK {
            face: 13,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 8,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 9,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 18,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 14
        FaceOrientIJK {
            face: 14,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 9,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 5,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 19,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 15
        FaceOrientIJK {
            face: 15,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 16,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 19,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 10,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 16
        FaceOrientIJK {
            face: 16,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 17,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 15,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 11,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 17
        FaceOrientIJK {
            face: 17,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 18,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 16,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 12,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 18
        FaceOrientIJK {
            face: 18,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 19,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 17,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 13,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 19
        FaceOrientIJK {
            face: 19,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 15,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 18,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 14,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
];

#[derive(Copy, Clone)]
pub struct FaceIJK {
    pub face: i32,
    pub coord: CoordIJK,
}

/** @struct FaceOrientIJK
 * @brief Information to transform into an adjacent face IJK system
 */
pub struct FaceOrientIJK {
    ///< face number
    pub face: i32,
    ///< res 0 translation relative to primary face
    pub translate: CoordIJK,
    ///< number of 60 degree ccw rotations relative to primary face
    pub ccwRot60: i32,
}

/** Digit representing overage type */
enum_from_primitive! {
    #[derive(PartialEq, PartialOrd, Copy, Clone)]
    pub enum Overage {
           /** No overage (on original face) */
    NoOverage = 0,
    /** On face edge (only occurs on substrate grids) */
    FaceEdge = 1,
    /** Overage on new face interior */
    NewFace = 2
    }
}

pub fn _geoToFaceIjk(g: &LatLng, res: i32) -> FaceIJK {
    // first convert to hex2d
    let mut v: Vec2d = Vec2d { x: 0.0, y: 0.0 };
    let mut h: FaceIJK = FaceIJK {
        face: 0,
        coord: CoordIJK { i: 0, j: 0, k: 0 },
    };
    _geoToHex2d(g, res, &mut h.face, &mut v);

    // then convert to ijk+
    _hex2dToCoordIJK(v, &mut h.coord);
    return h;
}

fn _geoToHex2d(g: &LatLng, res: i32, face: &mut i32, v: &mut Vec2d) {
    let mut sqd: f64 = 0.0;
    _geoToClosestFace(g, face, &mut sqd);

    // cos(r) = 1 - 2 * sin^2(r/2) = 1 - 2 * (sqd / 4) = 1 - sqd/2
    let mut r: f64 = (1.0 - sqd / 2.0).acos();

    if r < EPSILON {
        v.x = 0.0;
        v.y = 0.0;
        return;
    }

    // now have face and r, now find CCW theta from CII i-axis
    let mut theta: f64 = _posAngleRads(
        faceAxesAzRadsCII[*face as usize][0]
            - _posAngleRads(_geoAzimuthRads(&faceCenterGeo[*face as usize], &g)),
    );

    // adjust theta for Class III (odd resolutions)
    if isResolutionClassIII(res) {
        theta = _posAngleRads(theta - M_AP7_ROT_RADS);
    }

    // perform gnomonic scaling of r
    r = r.tan();

    // scale for current resolution length u
    r /= RES0_U_GNOMONIC;
    for _i in 0..res {
        r *= M_SQRT7;
    }

    // we now have (r, theta) in hex2d with theta ccw from x-axes

    // convert to local x,y
    v.x = r * theta.cos();
    v.y = r * theta.sin();
}

/**
 * Determines the center point in spherical coordinates of a cell given by 2D
 * hex coordinates on a particular icosahedral face.
 *
 * @param v The 2D hex coordinates of the cell.
 * @param face The icosahedral face upon which the 2D hex coordinate system is
 *             centered.
 * @param res The H3 resolution of the cell.
 * @param substrate Indicates whether or not this grid is actually a substrate
 *        grid relative to the specified resolution.
 * @param g The spherical coordinates of the cell center point.
 */
pub fn _hex2dToGeo(v: &Vec2d, face: i32, res: i32, substrate: bool) -> LatLng {
    // calculate (r, theta) in hex2d
    let mut r = _v2dMag(v);

    if r < EPSILON {
        return faceCenterGeo[face as usize];
    }

    let mut theta = (v.y).atan2(v.x);

    // scale for current resolution length u
    for i in 0..res {
        r /= M_SQRT7;
    }

    // scale accordingly if this is a substrate grid
    if substrate {
        r /= 3.0;
        if isResolutionClassIII(res) {
            r /= M_SQRT7;
        }
    }

    r *= RES0_U_GNOMONIC;

    // perform inverse gnomonic scaling of r
    r = r.atan();

    // adjust theta for Class III
    // if a substrate grid, then it's already been adjusted for Class III
    if !substrate && isResolutionClassIII(res) {
        theta = _posAngleRads(theta + M_AP7_ROT_RADS);
    }

    // find theta as an azimuth
    theta = _posAngleRads(faceAxesAzRadsCII[face as usize][0] - theta);

    // now find the point at (r,theta) from the face center
    return _geoAzDistanceRads(&faceCenterGeo[face as usize], theta, r);
}

/**
 * Determines the center point in spherical coordinates of a cell given by
 * a FaceIJK address at a specified resolution.
 *
 * @param h The FaceIJK address of the cell.
 * @param res The H3 resolution of the cell.
 * @param g The spherical coordinates of the cell center point.
 */
pub fn _faceIjkToGeo(h: FaceIJK, res: i32) -> LatLng {
    //let mut v: Vec2d = Vec2d {x: 0, y: 0};
    let v = _ijkToHex2d(&h.coord);
    return _hex2dToGeo(&v, h.face, res, false);
}

/**
 * Encodes a coordinate on the sphere to the corresponding icosahedral face and
 * containing the squared euclidean distance to that face center.
 *
 * @param g The spherical coordinates to encode.
 * @param face The icosahedral face containing the spherical coordinates.
 * @param sqd The squared euclidean distance to its icosahedral face center.
 */
fn _geoToClosestFace(g: &LatLng, face: &mut i32, sqd: &mut f64) {
    let mut v3d: Vec3d = Vec3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    _geoToVec3d(g, &mut v3d);

    // determine the icosahedron face
    *face = 0;
    // The distance between two farthest points is 2.0, therefore the square of
    // the distance between two points should always be less or equal than 4.0 .
    *sqd = 5.0f64;
    for f in 0..NUM_ICOSA_FACES {
        let sqdT: f64 = _pointSquareDist(faceCenterPoint[f as usize], v3d);
        if sqdT < *sqd {
            *face = f;
            *sqd = sqdT;
        }
    }
}

/**
 * Adjusts a FaceIJK address in place so that the resulting cell address is
 * relative to the correct icosahedral face.
 *
 * @param fijk The FaceIJK address of the cell.
 * @param res The H3 resolution of the cell.
 * @param pentLeading4 Whether or not the cell is a pentagon with a leading
 *        digit 4.
 * @param substrate Whether or not the cell is in a substrate grid.
 * @return 0 if on original face (no overage); 1 if on face edge (only occurs
 *         on substrate grids); 2 if overage on new face interior
 */
pub fn _adjustOverageClassII(
    fijk: &mut FaceIJK,
    res: i32,
    pentLeading4: bool,
    substrate: bool,
) -> Overage {
    let mut overage = Overage::NoOverage;

    let ijk: &mut CoordIJK = &mut fijk.coord;

    // get the maximum dimension value; scale if a substrate grid
    let mut maxDim = maxDimByCIIres[res as usize];
    if substrate {
        maxDim *= 3;
    }

    // check for overage
    if substrate && ijk.i + ijk.j + ijk.k == maxDim {
        // on edge
        overage = Overage::FaceEdge;
    } else if ijk.i + ijk.j + ijk.k > maxDim {
        // overage

        overage = Overage::NewFace;

        let mut fijkOrient: &FaceOrientIJK = &faceNeighbors[0][0];

        if ijk.k > 0 {
            if ijk.j > 0 {
                // jk "quadrant"
                fijkOrient = &faceNeighbors[fijk.face as usize][JK];
            } else {
                // ik "quadrant"
                fijkOrient = &faceNeighbors[fijk.face as usize][KI];

                // adjust for the pentagonal missing sequence
                if pentLeading4 {
                    // translate origin to center of pentagon
                    let mut origin: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
                    _setIJK(&mut origin, maxDim, 0, 0);
                    let mut tmp: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
                    _ijkSub(*ijk, origin, &mut tmp);
                    // rotate to adjust for the missing sequence
                    _ijkRotate60cw(&mut tmp);
                    // translate the origin back to the center of the triangle
                    _ijkAdd(tmp, origin, ijk);
                }
            }
        } else {
            // ij "quadrant"
            fijkOrient = &faceNeighbors[fijk.face as usize][IJ];
        }

        fijk.face = fijkOrient.face;

        // rotate and translate for adjacent face
        for i in 0..fijkOrient.ccwRot60 {
            _ijkRotate60ccw(ijk);
        }

        let mut transVec: CoordIJK = fijkOrient.translate;
        let mut unitScale = unitScaleByCIIres[res as usize];
        if substrate {
            unitScale *= 3;
        }
        _ijkScale(&mut transVec, unitScale);
        _ijkAdd(*ijk, transVec, ijk);
        _ijkNormalize(ijk);

        // overage points on pentagon boundaries can end up on edges
        if substrate && ijk.i + ijk.j + ijk.k == maxDim {
            // on edge
            overage = Overage::FaceEdge;
        }
    }

    return overage;
}
