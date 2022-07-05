use crate::lat_lng::LatLng;

#[derive(Copy, Clone)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

fn _square(x: f64) -> f64 {
    return x * x;
}

pub fn _pointSquareDist(v1: Vec3d, v2: Vec3d) -> f64 {
    return _square(v1.x - v2.x) + _square(v1.y - v2.y) + _square(v1.z - v2.z);
}

/**
 * Calculate the 3D coordinate on unit sphere from the latitude and longitude.
 *
 * @param geo The latitude and longitude of the point.
 * @param v The 3D coordinate of the point.
 */
pub fn _geoToVec3d(geo: &LatLng, v: &mut Vec3d) {
    let r: f64 = geo.lat.cos();

    v.z = geo.lat.sin();
    v.x = geo.lng.cos() * r;
    v.y = geo.lng.sin() * r;
}
