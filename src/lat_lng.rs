use num::Float;

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
 * Set the components of spherical coordinates in decimal degrees.
 *
 * @param p The spherical coordinates.
 * @param latDegs The desired latitude in decimal degrees.
 * @param lngDegs The desired longitude in decimal degrees.
 */
pub fn setGeoDegs(p: &mut LatLng, lat_degs: f64, lng_degs: f64) {
    _setGeoRads(p, lat_degs.to_radians(), lng_degs.to_radians());
}

/**
 * Set the components of spherical coordinates in radians.
 *
 * @param p The spherical coordinates.
 * @param latRads The desired latitude in decimal radians.
 * @param lngRads The desired longitude in decimal radians.
 */
pub fn _setGeoRads(p: &mut LatLng, lat_rads: f64, lng_rads: f64) {
    p.lat = lat_rads;
    p.lng = lng_rads;
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
