use std::f64::consts::{FRAC_PI_2, PI};

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

/** epsilon of ~0.1mm in degrees */
const EPSILON_DEG: f64 = 0.000000001;
/** epsilon of ~0.1mm in radians */
const EPSILON_RAD: f64 = EPSILON_DEG * M_PI_180;

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
 * Determines if the components of two spherical coordinates are within some
 * threshold distance of each other.
 *
 * @param p1 The first spherical coordinates.
 * @param p2 The second spherical coordinates.
 * @param threshold The threshold distance.
 * @return Whether or not the two coordinates are within the threshold distance
 *         of each other.
 */
pub fn geoAlmostEqualThreshold(p1: &LatLng, p2: &LatLng, threshold: f64) -> bool {
    return (p1.lat - p2.lat).abs() < threshold && (p1.lng - p2.lng).abs() < threshold;
}

/**
* Determines if the components of two spherical coordinates are within our
* standard epsilon distance of each other.
*
* @param p1 The first spherical coordinates.
* @param p2 The second spherical coordinates.
* @return Whether or not the two coordinates are within the epsilon distance
*         of each other.
*/
pub fn geoAlmostEqual(p1: &LatLng, p2: &LatLng) -> bool {
    return geoAlmostEqualThreshold(p1, p2, EPSILON_RAD);
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
 * constrainLat makes sure latitudes are in the proper bounds
 *
 * @param lat The original lat value
 * @return The corrected lat value
 */
pub fn constrainLat(mut lat: f64) -> f64 {
    while lat > FRAC_PI_2 {
        lat = lat - PI;
    }
    return lat;
}

/**
 * constrainLng makes sure longitudes are in the proper bounds
 *
 * @param lng The origin lng value
 * @return The corrected lng value
 */
pub fn constrainLng(mut lng: f64) -> f64 {
    while lng > PI {
        lng = lng - (2.0 * PI);
    }
    while lng < -PI {
        lng = lng + (2.0 * PI);
    }
    return lng;
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

/**
 * Computes the point on the sphere a specified azimuth and distance from
 * another point.
 *
 * @param p1 The first spherical coordinates.
 * @param az The desired azimuth from p1.
 * @param distance The desired distance from p1, must be non-negative.
 * @param p2 The spherical coordinates at the desired azimuth and distance from
 * p1.
 */
pub fn _geoAzDistanceRads(p1: &LatLng, mut az: f64, distance: f64) -> LatLng {
    let mut p2 = LatLng { lat: 0.0, lng: 0.0 };

    if distance < EPSILON {
        return *p1;
    }

    let mut sinlat = 0.0f64;
    let mut sinlng = 0.0f64;
    let mut coslng = 0.0f64;

    az = _posAngleRads(az);

    // check for due north/south azimuth
    if az < EPSILON || (az - PI).abs() < EPSILON {
        if az < EPSILON {
            // due north
            p2.lat = p1.lat + distance;
        } else {
            // due south
            p2.lat = p1.lat - distance;
        }

        if (p2.lat - FRAC_PI_2).abs() < EPSILON {
            // north pole
            p2.lat = FRAC_PI_2;
            p2.lng = 0.0;
        } else if (p2.lat + FRAC_PI_2).abs() < EPSILON {
            // south pole
            p2.lat = -FRAC_PI_2;
            p2.lng = 0.0;
        } else {
            p2.lng = constrainLng(p1.lng);
        }
    } else {
        // not due north or south
        sinlat = (p1.lat).sin() * (distance).cos() + (p1.lat).cos() * (distance).sin() * (az).cos();
        if sinlat > 1.0 {
            sinlat = 1.0;
        }
        if sinlat < -1.0 {
            sinlat = -1.0;
        }
        p2.lat = (sinlat).asin();
        if (p2.lat - FRAC_PI_2).abs() < EPSILON {
            // north pole
            p2.lat = FRAC_PI_2;
            p2.lng = 0.0;
        } else if (p2.lat + FRAC_PI_2).abs() < EPSILON {
            // south pole
            p2.lat = -FRAC_PI_2;
            p2.lng = 0.0;
        } else {
            sinlng = (az).sin() * (distance).sin() / (p2.lat).cos();
            coslng = ((distance).cos() - (p1.lat).sin() * (p2.lat).sin())
                / (p1.lat).cos()
                / (p2.lat).cos();
            if sinlng > 1.0 {
                sinlng = 1.0;
            }
            if sinlng < -1.0 {
                sinlng = -1.0;
            }
            if coslng > 1.0 {
                coslng = 1.0;
            }
            if coslng < -1.0 {
                coslng = -1.0;
            }
            p2.lng = constrainLng(p1.lng + (sinlng.atan2(coslng)));
        }
    }
    return p2;
}
