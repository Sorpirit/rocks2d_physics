pub mod scene {
    use raylib::ffi::PI;
    use raylib::math::{Vector2};
    use raylib::{color::Color};

    use crate::Control;
    use crate::state::state::{State, Transform2D};


    #[derive(Debug, Copy, Clone)]
    pub enum Shape {
        Circle(f32), //radius
        Rectangle(f32, f32), //half w, h
    }

    #[derive(Debug, Copy, Clone)]
    pub struct MassProperty {
        pub inv_mass: f32,
        pub inv_inertia: f32
    }

    impl MassProperty {
        pub fn new(shape: Shape, density: f32) -> Self {
            let area = match shape {
                Shape::Circle(r) => PI as f32 * r * r,
                Shape::Rectangle(wh, hh) => 4.0 * wh * hh
            };

            let mass = area * density;
            let inertia = match shape {
                Shape::Circle(r) => mass * r * r / 2.0,
                Shape::Rectangle(wh, hh) => mass * (wh * wh + hh * hh) / 3.0
            };

            if mass == 0.0 || inertia == 0.0 {
                return Self { inv_mass: 0.0, inv_inertia: 0.0 }
            }

            Self { inv_mass: 1.0 / mass, inv_inertia: 1.0 / inertia }
        }

        pub fn zero() -> Self {
            Self { inv_mass: 0.0, inv_inertia: 0.0 }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct RevoluteJoint 
    {
        pub body1_i: usize,
        pub body2_i: Option<usize>,

        pub local_attachment_b1: Vector2,
        pub local_attachment_b2: Vector2,

        pub compliance: f32
    }

    #[derive(Debug, Copy, Clone)]
    pub struct PrismaticJoint 
    {
        pub body_i: usize,
        pub target_y: f32,
        pub range_x: f32,
        pub compliance: f32
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Joint {
        RevoluteJoint(RevoluteJoint),
        PrismaticJoint(PrismaticJoint)
    }

    pub struct Scene {
        pub body_count: usize,
        pub initial_state: State,
        pub mass_properties: Vec<MassProperty>,
        pub body_shapes: Vec<Shape>,
        pub body_colors: Vec<Color>,
        
        pub joints: Vec<Joint>,

        pub env_count: u32,
        body_count_pre_env: usize,
    }

    pub struct RigidParameters {
        pub transform: Transform2D,
        pub velocities: (Vector2, f32),
        pub shape: Shape,
        pub density: f32,
        pub color: Color
    }

    impl RigidParameters {
        pub fn default() -> Self {
            Self { 
                transform: Transform2D::zero(), 
                velocities: (Vector2::ZERO, 0.0), 
                shape: Shape::Circle(0.5), 
                density: 1000.0,
                color: Color::DARKSEAGREEN
            }
        }
    }

    impl Scene {
        
        pub fn new(capacity: usize) -> Self {
            Self { 
                body_count: 0, 
                initial_state: State::new(capacity), 
                mass_properties: Vec::with_capacity(capacity), 
                body_shapes: Vec::with_capacity(capacity), 
                body_colors: Vec::with_capacity(capacity),
                joints: Vec::with_capacity(capacity),
                env_count: 1,
                body_count_pre_env: 0,
            }
        }

        pub fn add_ridig(&mut self, params: RigidParameters) -> usize {
            
            let ei = self.body_count;
            self.body_count += 1;

            self.initial_state.push_body(params.transform, params.velocities);
            self.mass_properties.push(MassProperty::new(params.shape, params.density));
            self.body_shapes.push(params.shape);
            self.body_colors.push(params.color);

            ei 
        }

        pub fn add_joint(&mut self, joint: Joint) {
            self.joints.push(joint);
        }

        pub fn replicate(&mut self, num_envs: u32) {
            if num_envs < 2 {
                return;
            }

            let base_joint_count = self.joints.len();

            for n in 1..num_envs {
                self.mass_properties.extend_from_within(0..self.body_count);
                self.body_shapes.extend_from_within(0..self.body_count);
                self.body_colors.extend_from_within(0..self.body_count);
                self.joints.extend_from_within(0..base_joint_count);

                let body_offset = self.body_count * n as usize;
                for joint in &mut self.joints[(base_joint_count * n as usize)..(base_joint_count * (n as usize + 1))] {
                    match joint {
                        Joint::RevoluteJoint(revolute_joint) => {
                            revolute_joint.body1_i += body_offset;
                            match &mut revolute_joint.body2_i {
                                Some(body2_i) => *body2_i += body_offset,
                                None => (),
                            }
                        },
                        Joint::PrismaticJoint(prismatic_joint) => prismatic_joint.body_i += body_offset,
                    }
                }
            }

            self.initial_state.replicate(num_envs);
            
            self.body_count_pre_env = self.body_count;
            self.body_count *= num_envs as usize;
            self.env_count = num_envs;
        }

        pub fn build_state(&self) -> State {
            let mut st = State::from(&self.initial_state);
            st.finalize();
            st
        }

        pub fn build_control(&self) -> Control {
            Control::new(self.body_count)
        }
        
        pub fn get_env_index(&self, ei: usize) -> usize {
            if self.env_count == 1 { return 0; }

            ei / self.body_count_pre_env
        }
    }

}