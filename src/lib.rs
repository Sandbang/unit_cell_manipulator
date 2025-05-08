

use nalgebra::{Matrix, Matrix3, Rotation3, Vector3, Vector4};
pub struct Atom {
    pub position: Vector3<f32>,
    pub number: i16
}
#[derive(Default)]
pub struct UnitCell{
    pub vec1: Vector3<f32>,
    pub vec2: Vector3<f32>,
    pub vec3: Vector3<f32>,
    pub atoms: Vec<Atom>,
    pub base_atom_count: usize,
    pub basis: bool,
}
impl UnitCell {
    pub fn print(&mut self){
        let m = Matrix3::from_columns(&[self.vec1, self.vec2, self.vec3]).transpose();
        println!("{}", m);
        for i in 0..self.atoms.len(){
            print!("{}:", self.atoms[i].number);
            println!("{}", self.atoms[i].position);
        }
    }
    pub fn vol(&mut self) -> f32{
        self.vec1.dot(&(self.vec2.cross(&self.vec3)))
    }
    pub fn extend(&mut self, a: i32, b: i32, c: i32){

        // For first extension dimension
        for i in 1..a{
            for j in 0..self.base_atom_count{
                let new_atom = Atom{
                    position: self.atoms[j].position + self.vec1 * i as f32,
                    number: self.atoms[j].number
                };
                self.atoms.push(new_atom);
            }
        }
        // For second extension dimension
        for i in 1..b{
            for j in 0..self.base_atom_count{
                let new_atom = Atom{
                    position: self.atoms[j].position + self.vec2 * i as f32,
                    number: self.atoms[j].number
                };
                self.atoms.push(new_atom);
            }
        }
        // For third extension dimension
        for i in 1..c{
            for j in 0..self.base_atom_count{
                let new_atom = Atom{
                    position: self.atoms[j].position + self.vec3 * i as f32,
                    number: self.atoms[j].number
                };
                self.atoms.push(new_atom);
            }
        }
    }
    pub fn rotate(&mut self, direction: char, angle: f32){
        let mut axisangle: Vector3<f32> = Vector3::x();
        match direction {
            'x' => axisangle = Vector3::x(),
            'y' => axisangle = Vector3::y(),
            'z' => axisangle = Vector3::z(),
            _ => println!("Defaulting to x")
        }
        axisangle = axisangle * angle;
        let rotation = Rotation3::new(axisangle);
        self.vec1 = rotation * self.vec1;
        self.vec2 = rotation * self.vec2;
        self.vec3 = rotation * self.vec3;
        for i in 0..self.atoms.len() {
            self.atoms[i].position = rotation * self.atoms[i].position;
        }
    }
    
    pub fn morph(&mut self, a: Vector3<f32>, b: Vector3<f32>, c: Vector3<f32>) {

    }

}