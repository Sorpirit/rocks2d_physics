use std::cell::RefCell;

use raylib::prelude::*;
use raylib::math::{Vector2};

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Circle(f32), //radius
    Rectangle(f32, f32), //half w, h
}

#[derive(Debug, Copy, Clone)]
pub struct Transform2D {
    pub position: Vector2,
    pub rotation: f32,
}

impl Transform2D {
    pub fn zero() -> Transform2D {
        Transform2D { position: Vector2::zero(), rotation: 0.0 }
    }
}

pub struct Entity {
    pub transform: Transform2D,
    pub prev_transform: Transform2D,

    pub velocity: Vector2,
    pub angular_velocity: f32,

    pub mass: f32,
    pub inertia: f32,

    pub inv_mass: f32,
    pub inv_inertia: f32,

    pub shape: Shape,
    pub color: Color
}

pub struct DistanceConstraint {
    pub body_index_1: usize,
    pub body_index_2: usize,

    pub local_attach_1: Vector2,
    pub local_attach_2: Vector2,

    pub target_distance: f32,
    pub compliance: f32
}

fn calcualte_area(shape: Shape) -> f32 {
    match shape {
        Shape::Circle(r) => PI as f32 * r * r,
        Shape::Rectangle(wh, hh) => 4.0 * wh * hh
    }
}

fn calculate_mass(shape: Shape, density: f32) -> f32 {
    calcualte_area(shape) * density
}

fn calculate_inertia(shape: Shape, mass: f32) -> f32 {
    match shape {
        Shape::Circle(r) => mass * r * r / 2.0,
        Shape::Rectangle(wh, hh) => mass * (wh * wh + hh * hh) / 3.0
    }
}

fn cross2d(v1: Vector2, v2: Vector2) -> f32 {
    (v1.x * v2.y) - (v1.y * v2.x)
}


impl Entity {
    pub fn new(transform: Transform2D, shape: Shape, density: f32) -> Entity {
        let mut ent = Entity { 
            transform: transform,
            prev_transform: transform, 
            velocity: Vector2::zero(), 
            angular_velocity: 0.0, 
            mass: calculate_mass(shape, density), 
            inertia: 0.0, 
            inv_mass: 0.0, 
            inv_inertia: 0.0, 
            shape: shape, 
            color: Color::DARKSEAGREEN 
        };

        if ent.mass > 0.0 {
            ent.inertia = calculate_inertia(shape, ent.mass);
            
            ent.inv_mass = 1.0 / ent.mass;
            ent.inv_inertia = 1.0 / ent.inertia;
        }

        return ent;
    }
}

fn verify(ents: &[Entity])
{
    for ent in ents {
        if ent.transform.position.x.is_infinite() || ent.transform.position.x.is_nan() || 
            ent.transform.position.y.is_infinite() || ent.transform.position.y.is_nan() || 
            ent.transform.rotation.is_infinite() || ent.transform.rotation.is_nan()
        {
            panic!("Positional value out of range");
        }
    }
}

fn integrate(ents: &mut [Entity], dt: f32)
{
    for ent in ents {
        ent.prev_transform = ent.transform;
        
        ent.velocity += Vector2::new(0.0, -9.81) * dt;
        // ent.angular_velocity += .. * dt;

        ent.transform.position += ent.velocity * dt;
        ent.transform.rotation += ent.angular_velocity * dt;
    }
}

fn ditstance_constraint(dist_const: &[DistanceConstraint], ents: &mut [Entity], dt: f32)
{
    for dc in dist_const {
        let en1 = &ents[dc.body_index_1];
        let en2 = &ents[dc.body_index_2];

        let rel_1 =  dc.local_attach_1.rotated(en1.transform.rotation);
        let rel_2 =  dc.local_attach_2.rotated(en2.transform.rotation); 

        let attach_p1 = rel_1 + en1.transform.position;
        let attach_p2 = rel_2 + en2.transform.position;

        let dir = attach_p2 - attach_p1;
        let c = dc.target_distance - dir.length();
        
        if c.abs() < 0.0001 {
            continue;
        }

        let normal = dir.normalized();
        
        let alpha = dc.compliance / dt.powi(2);

        let w1 = en1.inv_mass + en1.inv_inertia * cross2d(rel_1, normal).powi(2);
        let w2 = en2.inv_mass + en2.inv_inertia * cross2d(rel_2, normal).powi(2);

        let lambda = -c / (w1 + w2 + alpha);
        
        {
            let en1 = &mut ents[dc.body_index_1];
            en1.transform.position += normal * lambda * en1.inv_mass;
            en1.transform.rotation += en1.inv_inertia * cross2d(rel_1, normal * lambda);
        }
        
        {
            let en2 = &mut ents[dc.body_index_2];
            en2.transform.position -= normal * lambda * en2.inv_mass;
            en2.transform.rotation += en2.inv_inertia * cross2d(rel_2, -normal * lambda);
        }
    }
}

fn prismatic_constraint(ents: &mut [Entity], dt: f32)
{
    let target_y = -1.0;
    let compliance = 0.0;
    for ent in ents {
        let c = target_y - ent.transform.position.y;
        
        if c.abs() < 0.001 {
            continue;
        }

        let normal = -Vector2::new(0.0, (target_y - ent.transform.position.y).abs()).normalized();
        
        let alpha = compliance / dt.powi(2);
        let w = ent.inv_mass;
        let lambda = -c / (w + alpha);
        
        ent.transform.position += normal * lambda * ent.inv_mass;
        // ent.transform.rotation += ent.inv_inertia * cross2d(rel, normal * lambda);
    }
}

fn update_velocities(ents: &mut [Entity], dt: f32)
{
    for ent in ents {
        ent.velocity = (ent.transform.position - ent.prev_transform.position) / dt;
        ent.angular_velocity = (ent.transform.rotation - ent.prev_transform.rotation) / dt;
    }
}

pub struct State
{
    pub entities: RefCell<Vec<Entity>>,
    pub dist_const: RefCell<Vec<DistanceConstraint>>,
}

pub struct XPBDSolver 
{

}

impl XPBDSolver {
    pub fn new() -> XPBDSolver {
        XPBDSolver {  }
    }

    pub fn step(&self, state: &State, physics_dt: f32) {
        let mut entities = state.entities.borrow_mut();
        let dist_const = state.dist_const.borrow();

        integrate(&mut entities, physics_dt);
        ditstance_constraint(&dist_const, &mut entities, physics_dt);
        prismatic_constraint(&mut entities[0..1], physics_dt);
        update_velocities(&mut entities, physics_dt);
        verify(&entities);
    }

}

pub struct OnlineRenderer
{
    rl: raylib::RaylibHandle,
    thread: raylib::RaylibThread,

    bg_texture: Option<raylib::texture::Texture2D>,

    world_scalar: Vector2,
    camera_speed: f32,
    camera: Camera2D,

    total_time: f32
}

impl OnlineRenderer {
    pub fn new(window_w: i32, window_h: i32, world_size: f32) -> OnlineRenderer {
        let (mut rl, thread) = raylib::init()
            .size(window_w, window_h)
            .title("Rocks!")
            .vsync()
            .msaa_4x()
            .build();

        let bg_texture = rl.load_texture(&thread, "./assets/bg_grid.jpg").ok();
        
        let world_scalar = Vector2::new(window_w as f32 / world_size, -window_h as f32 / world_size);
        let camera_speed = 600.0;
        let camera = Camera2D {
            offset: Vector2 { x: window_w as f32 / 2.0, y: window_h as f32 / 2.0},
            target: Vector2::zero(),
            rotation: 0.0,
            zoom: 1.0,
        }; 

        OnlineRenderer { 
            rl,
            thread,

            bg_texture,

            world_scalar,
            camera_speed,
            camera,

            total_time: 0.0
        }
    }

    pub fn default() -> OnlineRenderer {
        OnlineRenderer::new(1024, 1024, 10.0)
    }

    pub fn should_close(&self) -> bool {
        self.rl.window_should_close()
    }

    pub fn get_delta_time(&self) -> f32 {
        self.rl.get_frame_time()
    } 

    pub fn interact(&self, state: &State) {
        let mut entities = state.entities.borrow_mut();
        let dt = self.rl.get_frame_time();
        let ent = &mut entities[0];
        let speed = 5.0;

        if self.rl.is_key_down(KeyboardKey::KEY_LEFT) { ent.velocity.x -= speed * dt; }
        if self.rl.is_key_down(KeyboardKey::KEY_RIGHT) { ent.velocity.x += speed * dt; }
    }

    pub fn render(&mut self, state: &State) {
        let entities = state.entities.borrow();
        let dt = self.rl.get_frame_time();
        self.total_time += dt;

        //input
        if self.rl.is_key_down(KeyboardKey::KEY_A) { self.camera.target.x -= self.camera_speed * dt; }
        if self.rl.is_key_down(KeyboardKey::KEY_D) { self.camera.target.x += self.camera_speed * dt; }
        if self.rl.is_key_down(KeyboardKey::KEY_W) { self.camera.target.y -= self.camera_speed * dt; }
        if self.rl.is_key_down(KeyboardKey::KEY_S) { self.camera.target.y += self.camera_speed * dt; }

        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::WHITE);

        if let Some(bg) = &self.bg_texture {
            d.draw_texture_rec(
                bg, 
                Rectangle::new(0.0, 0.0, 6.0*1024.0, 6.0* 1024.0), 
                -self.camera.target - Vector2::new(1024.0, 1024.0) * 3.0 - Vector2::new(-7., -7.), 
                Color::new(35, 35, 50, 255));
        };

        let mut r2d = d.begin_mode2D(self.camera);
        // r2d.draw_rectangle(x, y, width, height, color);
        for ent in entities.iter() {
            let view_pos = ent.transform.position * self.world_scalar;
            match ent.shape {
                Shape::Circle(r) => {
                    r2d.draw_circle_v(view_pos, r * self.world_scalar.x, ent.color);
                    r2d.draw_line_v(view_pos, view_pos + Vector2::new(r, 0.0).rotated(ent.transform.rotation) * self.world_scalar, Color::BLACK);
                },
                Shape::Rectangle(width_half, height_half) => {
                    let wh_view = width_half * self.world_scalar.x;
                    let hh_view = height_half * -self.world_scalar.y;
                    r2d.draw_rectangle_pro(Rectangle { x:  view_pos.x, y: view_pos.y, width: wh_view * 2.0, height: hh_view * 2.0 }, Vector2::new(width_half, -height_half) * self.world_scalar, -ent.transform.rotation.to_degrees(), ent.color);
                    
                    r2d.draw_circle_v(view_pos, 10.0, Color::YELLOW);

                    r2d.draw_line_v(view_pos, view_pos + Vector2::new(width_half, 0.0).rotated(ent.transform.rotation) * self.world_scalar, Color::BLACK);
                }
            }
        }
    }
}