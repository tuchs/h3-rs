/** 2.0 * PI */
pub const M_2PI: f64 = 6.28318530717958647692528676655900576839433f64;

/** pi / 180 */
pub const M_PI_180: f64 = 0.0174532925199432957692369076848861271111f64;
/** pi * 180 */
pub const M_180_PI: f64 = 57.29577951308232087679815481410517033240547f64;

/** threshold epsilon */
pub const EPSILON: f64 = 0.0000000000000001f64;
/** sqrt(3) / 2.0 */
pub const M_SQRT3_2: f64 = 0.8660254037844386467637231707529361834714f64;
pub const M_SQRT7: f64 = 2.6457513110645905905016157536392604257102f64;
/** sin(60') */
pub const M_SIN60: f64 = M_SQRT3_2;

/** rotation angle between Class II and Class III resolution axes
 * (asin(sqrt(3.0 / 28.0))) */
pub const M_AP7_ROT_RADS: f64 = 0.333473172251832115336090755351601070065900389f64;

/** sin(M_AP7_ROT_RADS) */
pub const M_SIN_AP7_ROT: f64 = 0.3273268353539885718950318f64;

/** cos(M_AP7_ROT_RADS) */
pub const M_COS_AP7_ROT: f64 = 0.9449111825230680680167902f64;

/** earth radius in kilometers using WGS84 authalic radius */
pub const EARTH_RADIUS_KM: f64 = 6371.007180918475f64;

/** scaling factor from hex2d resolution 0 unit length
 * (or distance between adjacent cell center points
 * on the plane) to gnomonic unit length. */
pub const RES0_U_GNOMONIC: f64 = 0.38196601125010500003f64;

/** max H3 resolution; H3 version 1 has 16 resolutions, numbered 0 through 15 */
pub const MAX_H3_RES: i32 = 15;

/** The number of faces on an icosahedron */
pub const NUM_ICOSA_FACES: i32 = 20;
/** The number of H3 base cells */
pub const NUM_BASE_CELLS: i32 = 122;
/** The number of vertices in a hexagon */
pub const NUM_HEX_VERTS: i32 = 6;
/** The number of vertices in a pentagon */
pub const NUM_PENT_VERTS: i32 = 5;
/** The number of pentagons per resolution **/
pub const NUM_PENTAGONS: i32 = 12;

/** H3 index modes */
pub const H3_CELL_MODE: i32 = 1;
pub const H3_DIRECTEDEDGE_MODE: i32 = 2;
pub const H3_EDGE_MODE: i32 = 3;
pub const H3_VERTEX_MODE: i32 = 4;
