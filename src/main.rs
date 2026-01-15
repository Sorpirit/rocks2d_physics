use rand::Rng;
use raylib::math::Vector2;

use rocks2d_physics::{Joint, PrismaticJoint, RevoluteJoint, RaylibViewer, RigidParameters, Scene, Shape, Transform2D, XPBDSolver};

fn main() {
    let mut renderer = RaylibViewer::default();
    let solver = XPBDSolver::new();

    let n_envs = 1024;

    let mut scene = Scene::new(2);
    
    let mut rb = RigidParameters::default();
    rb.transform = Transform2D::new(Vector2::new(0.00, -1.0), 0.0);
    rb.shape = Shape::Rectangle(0.8, 0.4);
    let b1 = scene.add_ridig(rb);

    let mut rb = RigidParameters::default();
    rb.transform = Transform2D::new(Vector2::new(1.5, -1.0), 0.0);
    rb.shape = Shape::Rectangle(1.5, 0.1);
    let b2 = scene.add_ridig(rb);

    scene.add_joint(Joint::PrismaticJoint(PrismaticJoint {
        body_i: b1,
        target_y: -1.0,
        range_x: 5.0,
        compliance: 0.0
    }));

    scene.add_joint(Joint::RevoluteJoint(RevoluteJoint {
        body1_i: b1,
        body2_i: Some(b2),
        local_attachment_b1: Vector2::zero(),
        local_attachment_b2: Vector2::new(1.5, 0.0),
        compliance: 0.0,
    }));

    scene.replicate(n_envs);

    let mut control = scene.build_control();
    let mut state_in = scene.build_state();
    let mut state_out = scene.build_state();

    let mut rng = rand::rng();
    for bi in 0..state_in.body_count {
        if bi % 2 == 0 {
            continue;
        }
        control.update_forces(bi, (Vector2::new(rng.random_range(-20.0..20.0), 0.0), 0.0));
    }
    
    let substeps = 16;
    let mut totral_time = 0.0;
    let mut frame = 0;
    // let physics_dt: f32 = 1.0 / 60.0 / substeps as f32;

    while !renderer.should_close() {
        let dt = renderer.get_delta_time();
        frame += 1;
        totral_time += dt;

        if frame % 120 == 0 {
            for bi in 0..state_in.body_count {
                if bi % 2 == 0 {
                    continue;
                }
                control.update_forces(bi, (Vector2::new(rng.random_range(-7.0..7.0), 0.0), 0.0));
            }
        }

        if totral_time > 0.7
        {
            let physics_dt: f32 = dt / substeps as f32;
            for _ in 0..substeps {
                solver.step(&mut state_in, &mut state_out, &control, &scene, physics_dt);
                std::mem::swap(&mut state_in, &mut state_out);
            }
        }
        
        // renderer.interact(&state);
        renderer.render(&scene, &state_in);        
    }

}