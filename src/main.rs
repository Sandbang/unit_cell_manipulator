use unit_cell_manipulator::UnitCell;
use unit_cell_manipulator::Atom;
use nalgebra::{Vector3};
fn main() {
    println!("Hello, world!");
    let atom1: Atom = Atom{position: Vector3::new(0.0, 0.0, 0.0), number: 34 };
    let atom2: Atom = Atom{position: Vector3::new(0.25, 0.25, 0.25), number: 29 };
    let atom3: Atom = Atom{position: Vector3::new(0.75, 0.75, 0.75), number: 29 };
    let unitcell = &mut UnitCell{
        vec1: Vector3::new(0.5,0.5,0.0),
        vec2: Vector3::new(0.5,0.0,0.5),
        vec3: Vector3::new(0.0,0.5,0.5),
        atoms: vec!(atom1, atom2, atom3),
        base_atom_count: 2,
        basis: false,
    };
    //unitcell.extend(1,1,1);
    let x: f32 = 2.0;
    unitcell.rotate('z', std::f32::consts::FRAC_PI_4);
    unitcell.rotate('x', x.sqrt().atan());
    unitcell.print();
}
