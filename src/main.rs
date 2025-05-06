use unit_cell_manipulator::UnitCell;
use unit_cell_manipulator::Atom;
use nalgebra::{Vector3};
fn main() {
    println!("Hello, world!");
    let atom1: Atom = Atom{position: Vector3::new(0.1, 0.2, 0.2), number: 1 };
    let atom2: Atom = Atom{position: Vector3::new(0.4, 0.4, 0.4), number: 2 };
    let unitcell = &mut UnitCell{
        vec1: Vector3::new(1.0,0.0,0.0),
        vec2: Vector3::new(0.0,1.0,0.0),
        vec3: Vector3::new(0.0,0.0,1.0),
        atoms: vec!(atom1, atom2),
        atom_count: 2,
        basis: false,
    };
    unitcell.extend(1,1,3);
    unitcell.print();
}
