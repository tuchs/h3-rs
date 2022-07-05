use crate::constants::*;

#[doc = " @struct LatLng"]
#[doc = "@brief latitude/longitude in radians"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LatLng {
    #[doc = "< latitude in radians"]
    pub lat: f64,
    #[doc = "< longitude in radians"]
    pub lng: f64,
}

/**
 * Normalizes radians to a value between 0.0 and two PI.
 *
 * @param rads The input radians value.
 * @return The normalized radians value.
 */
pub fn _posAngleRads(rads: f64) -> f64 {
    let mut tmp: f64 = if rads < 0.0 { rads + M_2PI } else { rads };
    if rads >= M_2PI {
        tmp -= M_2PI;
    }
    return tmp;
}

/**
 * Determines the azimuth to p2 from p1 in radians.
 *
 * @param p1 The first spherical coordinates.
 * @param p2 The second spherical coordinates.
 * @return The azimuth in radians from p1 to p2.
 */
pub fn _geoAzimuthRads(p1: &LatLng, p2: &LatLng) -> f64 {
    return (p2.lat.cos() * (p2.lng - p1.lng).sin()).atan2(
        p1.lat.cos() * p2.lat.sin() - p1.lat.sin() * p2.lat.cos() * (p2.lng - p1.lng).cos(),
    );
}
