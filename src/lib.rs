use h3_index::H3Index;

pub mod algos;
mod base_cells;
mod constants;
mod coord_ijk;
pub mod directed_edge;
pub mod error;
mod face_ijk;
pub mod h3_index;
pub mod lat_lng;
pub mod vec2d;
pub mod vec3d;

#[macro_use]
extern crate enum_primitive;
extern crate num;

pub const H3_NULL: H3Index = 0;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 4; //add(2, 2);
        assert_eq!(result, 4);
    }
}
