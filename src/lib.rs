

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
        self.vec1.dot(&(self.vec3.cross(&self.vec2)))
    }
    pub fn extend(&mut self, a: i32, b: i32, c: i32){

        self.vec1 *= a as f32;
        self.vec2 *= a as f32;
        self.vec3 *= a as f32;
        for i in 1..a{
            for j in 0..self.base_atom_count{
                let new_atom = Atom{
                    position: self.atoms[j].position + self.vec1 * i as f32,
                    number: self.atoms[j].number
                };
                self.atoms.push(new_atom);
            }
        }
        for i in 1..b{
            for j in 0..self.base_atom_count{
                let new_atom = Atom{
                    position: self.atoms[j].position + self.vec2 * i as f32,
                    number: self.atoms[j].number
                };
                self.atoms.push(new_atom);
            }
        }
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
        let inverse = Matrix3::from_columns(&[a, b, c]).transpose().try_inverse().unwrap(); //remove unwrap later
        let mut counter = 0;
        self.extend(3, 3, 3);
        for i in 0..self.atoms.len(){
            let solution = inverse * self.atoms[i-counter].position;
            if (solution[0] >= 1.0) | (solution[1] >= 1.0) | (solution[2] >= 1.0){ 
                self.atoms.remove(i-counter);
                counter += 1;
            }
        }
        self.vec1 = a;
        self.vec2 = b;
        self.vec2 = c;
    }

}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extend_rotate(){
        let se: Atom = Atom{position: Vector3::new(0.0, 0.0, 0.0), number: 34 };
        let cu0: Atom = Atom{position: Vector3::new(0.25, 0.25, 0.25), number: 29 };
        let cu1: Atom = Atom{position: Vector3::new(0.75, 0.75, 0.75), number: 29 };
        let unitcell = &mut UnitCell{
            vec1: Vector3::new(0.5,0.5,0.0),
            vec2: Vector3::new(0.5,0.0,0.5),
            vec3: Vector3::new(0.0,0.5,0.5),
            atoms: vec!(se, cu0, cu1),
            base_atom_count: 2,
            basis: false,
        };
        unitcell.extend(1,1,1);
        unitcell.rotate('z', std::f32::consts::FRAC_PI_4);
        unitcell.rotate('x', (2.0 as f32).sqrt().atan());
        assert!(Matrix3::from_columns(&[unitcell.vec1, unitcell.vec2, unitcell.vec3]).transpose() == Matrix3::from_columns(&[
            Vector3::new(0.0, 1.0/(6.0 as f32).sqrt(), 1.0/(3.0 as f32).sqrt()), 
            Vector3::new(1.0/(8.0 as f32).sqrt(), -1.0/(24.0 as f32).sqrt(), 1.0/(3.0 as f32).sqrt()), 
            Vector3::new(-1.0/(8.0 as f32).sqrt(), -1.0/(24.0 as f32).sqrt(), 1.0/(3.0 as f32).sqrt()), 
            ]).transpose());
    }
}