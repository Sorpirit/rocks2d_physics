use raylib::prelude::*;
use raylib::math::{Vector2};

pub enum Shape {
    Circle(f32), //radius
    Rectangle(f32, f32), //half w, h
}

pub struct Entity {
    pub position: Vector2,
    pub rotation: f32,

    pub shape: Shape,
    pub color: Color
}

fn main() {
    let window_w = 1024;
    let window_h = 1024;
    let (mut rl, thread) = raylib::init()
        .size(window_w, window_h)
        .title("Rocks!")
        .vsync()
        .build();

    let bg_texture = rl.load_texture(&thread, "./assets/bg_grid.jpg");

    let world_size = 10.0;
    let world_scalar = Vector2::new(window_w as f32 / world_size, -window_h as f32 / world_size);
    let camera_speed = 600.0;
    let mut camera = Camera2D {
        offset: Vector2 { x: window_w as f32 / 2.0, y: window_h as f32 / 2.0},
        target: Vector2::zero(),
        rotation: 0.0,
        zoom: 1.0,
    }; 

    let mut entities = vec![
        Entity { position: Vector2::new(2.0, 0.0), rotation: 0.0, shape: Shape::Circle(0.5), color: Color::BLUE },
        Entity { position: Vector2::new(-1.0, 1.0), rotation: 0.0, shape: Shape::Circle(1.0), color: Color::RED },
        Entity { position: Vector2::new(0.5, 0.0), rotation: 0.0, shape: Shape::Rectangle(1.0, 0.5), color: Color::GREEN }

    ];

    let mut total_time = 0.0;

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        total_time += dt;

        //input
        if rl.is_key_down(KeyboardKey::KEY_A) { camera.target.x -= camera_speed * dt; }
        if rl.is_key_down(KeyboardKey::KEY_D) { camera.target.x += camera_speed * dt; }
        if rl.is_key_down(KeyboardKey::KEY_W) { camera.target.y -= camera_speed * dt; }
        if rl.is_key_down(KeyboardKey::KEY_S) { camera.target.y += camera_speed * dt; }
        
        let mouse_pos = rl.get_mouse_position();
        let mouse_delta = rl.get_mouse_delta();

        // todo physics sim
        for ent in &mut entities {
            ent.position.y = (total_time).sin();
            ent.rotation = (total_time).cos() * 1.0 * 3.14;
        }

        let target_ent = &mut entities[0];
        target_ent.position = (mouse_pos + camera.target - camera.offset) / world_scalar;

        //rendering
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        if let Ok(bg) = &bg_texture {
            d.draw_texture_rec(
                bg, 
                Rectangle::new(0.0, 0.0, 6.0*1024.0, 6.0* 1024.0), 
                -camera.target - Vector2::new(1024.0, 1024.0) * 3.0 - Vector2::new(-7., -7.), 
                Color::new(35, 35, 50, 255));
        };

        let mut r2d = d.begin_mode2D(camera);
        for ent in &entities {
            let view_pos = ent.position * world_scalar;
            match ent.shape {
                Shape::Circle(r) => {
                    r2d.draw_circle_v(view_pos, r * world_scalar.x, ent.color);
                    r2d.draw_line_v(view_pos, view_pos + Vector2::new(r * world_scalar.x, 0.0).rotated(ent.rotation), Color::BLACK);
                },
                Shape::Rectangle(width_half, height_half) => {
                    let wh_view = width_half * world_scalar.x;
                    let hh_view = height_half * world_scalar.x;
                    r2d.draw_rectangle_pro(Rectangle { x:  view_pos.x, y: view_pos.y, width: wh_view * 2.0, height: hh_view * 2.0 }, Vector2::new(wh_view, hh_view), ent.rotation.to_degrees(), ent.color);
                    r2d.draw_circle_v(view_pos, 10.0, Color::YELLOW);
                    r2d.draw_line_v(view_pos, view_pos + Vector2::new(wh_view, 0.0).rotated(ent.rotation), Color::BLACK);
                }
            }
        }
    }
}

