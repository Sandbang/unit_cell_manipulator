use unit_cell_manipulator::UnitCell;
use nalgebra::{Vector3, Vector4};
fn main() {
    println!("Hello, world!");
    let unitcell = UnitCell{
        vec1: Vector3::new(1.0,0.0,0.0),
        vec2: Vector3::new(0.0,1.0,0.0),
        vec3: Vector3::new(0.0,0.0,1.0),
        atoms: vec!(Vector4::new(0.0,0.0,0.0,1.0)),
        basis: false,

    };
}
