pub mod state {
    use raylib::math::{Vector2};

    const TRANSFORM_2D_FLOAT_SIZE: usize = 3;

    #[derive(Debug, Copy, Clone)]
    pub struct Transform2D {
        pub position: Vector2,
        pub rotation: f32,
    }

    impl Transform2D {
        pub fn new(pos: Vector2, rot: f32) -> Self {
            Self { position: pos, rotation: rot }
        }

        pub fn zero() -> Self {
            Self { position: Vector2::zero(), rotation: 0.0 }
        }
    }

    pub struct State {
        transforms: Vec<f32>,
        velocities: Vec<f32>,

        pub body_count: usize,
        finalized: bool
    }

    impl State {

        pub fn new(capacity: usize) -> Self {
            Self { transforms: Vec::with_capacity(capacity), velocities: Vec::with_capacity(capacity), body_count: 0, finalized: false }
        }

        pub fn from(other: &Self) -> Self {
            Self { transforms: other.transforms.clone(), velocities: other.velocities.clone(), body_count: other.body_count, finalized: other.finalized }
        }

        pub fn copy_from(&mut self, other: &Self) {
            self.transforms.copy_from_slice(&other.transforms);
            self.velocities.copy_from_slice(&other.velocities);
        }

        pub fn push_body(&mut self, trnasform: Transform2D, velocities: (Vector2, f32)) -> bool {
            if self.finalized {
                return false;
            }

            let tr_slice = [trnasform.position.x, trnasform.position.y, trnasform.rotation]; 
            let vl_slice = [velocities.0.x, velocities.0.y, velocities.1];            
            self.transforms.extend_from_slice(&tr_slice);
            self.velocities.extend_from_slice(&vl_slice);
            self.body_count += 1;

            return true;
        }

        pub fn finalize(&mut self) {
            self.finalized = true;
        }

        pub fn get_native_transforms(&self) -> &[f32] {
            &self.transforms
        }

        pub fn get_native_velocties(&self) -> &[f32] {
            &self.velocities
        }

        pub fn query_transform(&self, body_index: usize) -> Transform2D {
            let ti = body_index * TRANSFORM_2D_FLOAT_SIZE;
            let x = self.transforms[ti + 0];
            let y = self.transforms[ti + 1];
            let r = self.transforms[ti + 2];

            Transform2D { position: Vector2::new(x, y), rotation: r }
        }

        pub fn query_velocity(&self, body_index: usize) -> (Vector2, f32) {
            let ti = body_index * 3;
            let x = self.velocities[ti + 0];
            let y = self.velocities[ti + 1];
            let r = self.velocities[ti + 2];

            (Vector2::new(x, y), r)
        }

        pub fn update_transform(&mut self, body_index: usize, transform: Transform2D) {
            let ti = body_index * TRANSFORM_2D_FLOAT_SIZE;
            self.transforms[ti + 0] = transform.position.x;
            self.transforms[ti + 1] = transform.position.y;
            self.transforms[ti + 2] = transform.rotation;
        }

        pub fn update_velocity(&mut self, body_index: usize, velocities: (Vector2, f32)) {
            let ti = body_index * 3;
            self.velocities[ti + 0] = velocities.0.x;
            self.velocities[ti + 1] = velocities.0.y;
            self.velocities[ti + 2] = velocities.1;
        }
        
        pub(crate) fn replicate(&mut self, num_envs: u32) {
            if num_envs < 2 {
                return;
            }

            let best_env_transforms = self.transforms.len();
            let best_env_velocities = self.velocities.len();

            for _ in 1..num_envs {
                self.transforms.extend_from_within(0..best_env_transforms);
                self.velocities.extend_from_within(0..best_env_velocities);
            }
            
            self.body_count *= num_envs as usize;
        }
    }
}