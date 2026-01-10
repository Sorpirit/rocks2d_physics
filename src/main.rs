use std::cell::RefCell;

use raylib::math::Vector2;
use rocks2d_physics::{DistanceConstraint, Entity, OnlineRenderer, Shape, State, Transform2D, XPBDSolver};

fn main() {
    let mut renderer = OnlineRenderer::default();
    let solver = XPBDSolver::new();

    
    let entities = vec![
        Entity::new(Transform2D { position: Vector2::new(0.00, -1.0), rotation: 0.0 }, Shape::Rectangle(0.8, 0.4), 1000.0),
        Entity::new(Transform2D { position: Vector2::new(1.5, -1.0), rotation: 0.0 }, Shape::Rectangle(1.5, 0.1), 1000.0),
    ];

    let dist_const = vec![
        DistanceConstraint { body_index_1: 0, body_index_2: 1, local_attach_1: Vector2::zero(), local_attach_2: Vector2::new(1.5, 0.0), target_distance: 0.001, compliance: 0.0 }
    ];

    let state = State {
        entities: RefCell::new(entities),
        dist_const: RefCell::new(dist_const),
    };

    
    let substeps = 16;
    let mut totral_time = 0.0;
    // let physics_dt: f32 = 1.0 / 60.0 / substeps as f32;

    while !renderer.should_close() {
        let dt = renderer.get_delta_time();
        totral_time += dt;
        if totral_time > 0.7
        {
            let physics_dt: f32 = dt / substeps as f32;
            for _ in 0..substeps {
                solver.step(&state, physics_dt);
            }
        }
        
        renderer.interact(&state);
        renderer.render(&state);
    
    }

}