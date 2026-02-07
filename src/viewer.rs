use raylib::prelude::*;

use crate::{imgui::RaylibImguiSupport, scene::scene::{Scene, Shape}, state::state::State};

pub struct RaylibViewer
{
    rl: raylib::RaylibHandle,
    thread: raylib::RaylibThread,
    imgui_rl: RaylibImguiSupport,

    bg_texture: Option<raylib::texture::Texture2D>,

    world_scalar: Vector2,
    camera_speed: f32,
    camera: Camera2D,

    total_time: f32,

    env_offset: Vector2
}

impl RaylibViewer {
    pub fn new(window_w: i32, window_h: i32, world_size: f32) -> Self {

        // raylib::
        // raylib::fla
        // unsafe { raylib::ffi::SetConfigFlags(raylib::ffi::ConfigFlags::FLAG_WINDOW_HIGHDPI as u32) };
        // raylib::
        let (mut rl, thread) = raylib::init()
            .size(window_w, window_h)
            .title("Rocks!")
            .vsync()
            .msaa_4x().resizable()
            .build();

        let imgui_rl = RaylibImguiSupport::setup(&mut rl, &thread);
        // rl.get_window_state().set_window_highdpi(true);
        // rl.get_window_handle().

        // let imgui = RayImGUIHandle::
        // rl.begin_drawing(_)

        let bg_texture = rl.load_texture(&thread, "./assets/bg_grid.jpg").ok();
        
        let world_scalar = Vector2::new(window_w as f32 / world_size, -window_h as f32 / world_size);
        let camera_speed = 600.0;
        let camera = Camera2D {
            offset: Vector2 { x: window_w as f32 / 2.0, y: window_h as f32 / 2.0},
            target: Vector2::ZERO,
            rotation: 0.0,
            zoom: 1.0,
        }; 

        Self { 
            rl,
            thread,
            imgui_rl,

            bg_texture,

            world_scalar,
            camera_speed,
            camera,

            total_time: 0.0,

            env_offset: Vector2::ZERO
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

    pub fn set_env_offset(&mut self, env_offset: Vector2) {
        self.env_offset = env_offset;
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
        let mut demo = true;

        //input
        if self.rl.is_key_down(KeyboardKey::KEY_A) { self.camera.target.x -= self.camera_speed * dt; }
        if self.rl.is_key_down(KeyboardKey::KEY_D) { self.camera.target.x += self.camera_speed * dt; }
        if self.rl.is_key_down(KeyboardKey::KEY_W) { self.camera.target.y -= self.camera_speed * dt; }
        if self.rl.is_key_down(KeyboardKey::KEY_S) { self.camera.target.y += self.camera_speed * dt; }

        let ui = self.imgui_rl.start_frame(&mut self.rl);
        let mut d = self.rl.begin_drawing(&self.thread);
        // d.draw_imgui(demo);
        // d.gui_panel(bounds, "demo");
        // let mut imgui = d.begin_imgui();
        d.clear_background(Color::WHITE);
        // imgui.unwrap().show_demo_window(&mut demo);

        if let Some(bg) = &self.bg_texture {
            d.draw_texture_rec(
                bg, 
                Rectangle::new(0.0, 0.0, 6.0*1024.0, 6.0* 1024.0), 
                -self.camera.target - Vector2::new(1024.0, 1024.0) * 3.0 - Vector2::new(-7., -7.), 
                Color::new(35, 35, 50, 255));
        };

        //draw scene
        {
            let mut r2d = d.begin_mode2D(self.camera);
            // r2d.draw_rectangle(x, y, width, height, color);
            for ei in 0..scene.body_count {
                let env_index = scene.get_env_index(ei);
                let mut transform = state.query_transform(ei);
                transform.position += self.env_offset * env_index as f32;
                let shape = scene.body_shapes[ei];
                let color = scene.body_colors[ei];

                let view_pos = transform.position * self.world_scalar;
                match shape {
                    Shape::Circle(r) => {
                        r2d.draw_circle_v(view_pos, r * self.world_scalar.x, color);
                        r2d.draw_line_v(view_pos, view_pos + Vector2::from_angle(transform.rotation).rotate(Vector2::new(r, 0.0)) * self.world_scalar, Color::BLACK);
                    },
                    Shape::Rectangle(width_half, height_half) => {
                        let wh_view = width_half * self.world_scalar.x;
                        let hh_view = height_half * -self.world_scalar.y;
                        r2d.draw_rectangle_pro(Rectangle { x:  view_pos.x, y: view_pos.y, width: wh_view * 2.0, height: hh_view * 2.0 }, 
                            Vector2::new(width_half, -height_half) * self.world_scalar, -transform.rotation.to_degrees(), color);
                        
                        r2d.draw_circle_v(view_pos, 10.0, Color::YELLOW);

                        r2d.draw_line_v(view_pos, 
                            view_pos + Vector2::from_angle(transform.rotation).rotate(Vector2::new(width_half, 0.0)) * self.world_scalar, Color::BLACK);
                    }
                }
            }
        }
        

        //draw ui
        {
            let width = d.get_render_width();
            let height = d.get_render_height();
            let status = SimulationMode::Editing;

            let ws = ui.push_style_color(imgui::StyleColor::WindowBg, Color::GREEN);
            if let Some(wt) = ui.window("simulation_status").bg_alpha(0.15).position(Vector2::new(width as f32 * 0.5, height as f32 * 0.0), imgui::Condition::Always).position_pivot(Vector2::new(0.5, -0.1)).no_decoration().no_nav().begin() {
               
                {
                    let _ = ui.push_style_color(imgui::StyleColor::Button, Color::RED);
                    ui.button("Play");
                }
                
                
                
                ui.same_line();
                ui.button("Next");
                ui.same_line();
                ui.button("Reset");
                
                wt.end(); 
            }
            ws.pop();

            // let s =ui.push_style_var(imgui::StyleVar::WindowPadding([15.0, 15.0]));
            if let Some(wt) = ui.window("performance_parameters").bg_alpha(0.15).position(Vector2::new(width as f32 * 1.0, height as f32 * 0.0), imgui::Condition::Always).position_pivot(Vector2::new(1.0, -0.1)).no_decoration().no_nav().begin() {
               
                {
                    let _ = ui.push_style_color(imgui::StyleColor::Button, Color::RED);
                    ui.button("Play");
                }
                
                
                
                ui.same_line();
                ui.button("Next");
                ui.same_line();
                ui.button("Reset");
                
                wt.end(); 
            }
            
            if let Some(wt) = ui
                .window("Sim controls")
                .position(Vector2::new(1.0, 1.0), imgui::Condition::Appearing)
                // .position_pivot(Vector2::new(0.5, 0.1))
                .size([200.0, 60.0], imgui::Condition::Appearing)
                .begin()
            {
                ui.button("so");
                        // ui.same_line();
                    ui.button("la");
                if let Some(tb) = ui.tab_bar("Properties Manager") {
                    if let Some (ti) = ui.tab_item("General Properties") {
                        // ui.same_line();
                        ui.button("hei");
                        // ui.same_line();
                        ui.button("ho");
                        ti.end();
                    }

                    if let Some (ti) = ui.tab_item("Particles") {
                        // ui.same_line();
                        ui.button("pr1");
                        // ui.same_line();
                        ui.button("p2");
                        ti.end();
                    }

                    if let Some (ti) = ui.tab_item("Rigid & Joints") {
                        // ui.same_line();
                        ui.button("pr1");
                        // ui.same_line();
                        ui.button("p2");
                        ti.end();
                    }

                    tb.end();
                }
                wt.end();
            };

            

            ui.show_demo_window(&mut demo);
        }

        self.imgui_rl.end_frame(&mut d);
    }
}

enum SimulationMode {
    Editing,
    Playing,
    Puased,
    Onestep,
}

struct EditorProperties{

}