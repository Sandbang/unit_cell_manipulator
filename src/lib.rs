use nalgebra::{Matrix, Rotation3, Vector3, Vector4};
#[derive(Default)]
pub struct UnitCell{
    pub vec1: Vector3<f32>,
    pub vec2: Vector3<f32>,
    pub vec3: Vector3<f32>,
    pub atoms: Vec<Vector4<f32>>,
    pub basis: bool,
}
impl UnitCell {
    fn vol(&mut self) -> f32{
        self.vec1.dot(&(self.vec2.cross(&self.vec3)))
    }
}