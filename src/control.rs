use raylib::math::Vector2;

pub struct Control {
    forces: Vec<f32>,
    pub body_count: usize,
}

impl Control {

    pub fn new(body_count: usize) -> Self {
        Self { forces: vec![0.0; body_count * 3], body_count: body_count }
    }

    pub fn get_native_forces(&self) -> &[f32] {
        &self.forces
    }

    pub fn set_native_forces(&mut self, forces: &[f32]) {
        self.forces.copy_from_slice(forces);
    }

    pub fn query_forces(&self, body_index: usize) -> (Vector2, f32) {
        let ti = body_index * 3;
        let x = self.forces[ti + 0];
        let y = self.forces[ti + 1];
        let r = self.forces[ti + 2];

        (Vector2::new(x, y), r)
    }

    pub fn update_forces(&mut self, body_index: usize, force: (Vector2, f32)) {
        let ti = body_index * 3;
        self.forces[ti + 0] = force.0.x;
        self.forces[ti + 1] = force.0.y;
        self.forces[ti + 2] = force.1;
    }
}