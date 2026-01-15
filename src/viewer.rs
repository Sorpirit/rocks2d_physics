use raylib::prelude::*;

use crate::{scene::scene::{Scene, Shape}, state::state::State};

pub struct RaylibViewer
{
    rl: raylib::RaylibHandle,
    thread: raylib::RaylibThread,

    bg_texture: Option<raylib::texture::Texture2D>,

    world_scalar: Vector2,
    camera_speed: f32,
    camera: Camera2D,

    total_time: f32
}

impl RaylibViewer {
    pub fn new(window_w: i32, window_h: i32, world_size: f32) -> Self {
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

        Self { 
            rl,
            thread,

            bg_texture,

            world_scalar,
            camera_speed,
            camera,

            total_time: 0.0
        }
    }

    pub fn default() -> Self {
        Self::new(1024, 1024, 10.0)
    }

    pub fn should_close(&self) -> bool {
        self.rl.window_should_close()
    }

    pub fn get_delta_time(&self) -> f32 {
        self.rl.get_frame_time()
    } 

    // pub fn interact(&self, state: &State) {
    //     let mut entities = state.entities.borrow_mut();
    //     let dt = self.rl.get_frame_time();
    //     let ent = &mut entities[0];
    //     let speed = 5.0;

    //     if self.rl.is_key_down(KeyboardKey::KEY_LEFT) { ent.velocity.x -= speed * dt; }
    //     if self.rl.is_key_down(KeyboardKey::KEY_RIGHT) { ent.velocity.x += speed * dt; }
    // }

    pub fn render(&mut self, scene: &Scene, state: &State) {
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
        for ei in 0..scene.body_count {
            let transform = state.query_transform(ei);
            let shape = scene.body_shapes[ei];
            let color = scene.body_colors[ei];

            let view_pos = transform.position * self.world_scalar;
            match shape {
                Shape::Circle(r) => {
                    r2d.draw_circle_v(view_pos, r * self.world_scalar.x, color);
                    r2d.draw_line_v(view_pos, view_pos + Vector2::new(r, 0.0).rotated(transform.rotation) * self.world_scalar, Color::BLACK);
                },
                Shape::Rectangle(width_half, height_half) => {
                    let wh_view = width_half * self.world_scalar.x;
                    let hh_view = height_half * -self.world_scalar.y;
                    r2d.draw_rectangle_pro(Rectangle { x:  view_pos.x, y: view_pos.y, width: wh_view * 2.0, height: hh_view * 2.0 }, 
                        Vector2::new(width_half, -height_half) * self.world_scalar, -transform.rotation.to_degrees(), color);
                    
                    r2d.draw_circle_v(view_pos, 10.0, Color::YELLOW);

                    r2d.draw_line_v(view_pos, 
                        view_pos + Vector2::new(width_half, 0.0).rotated(transform.rotation) * self.world_scalar, Color::BLACK);
                }
            }
        }
    }
}