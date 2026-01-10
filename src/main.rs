use std::cell::RefCell;

use rand::Rng;
use raylib::math::Vector2;
use rocks2d_physics::{DistanceConstraint, Entity, OnlineRenderer, Shape, State, Transform2D, XPBDSolver};

fn main() {
    let mut renderer = OnlineRenderer::default();
    let solver = XPBDSolver::new();

    let n_envs = 4;
    let mut entities: Vec<Entity> = Vec::new();
    let mut dist_const: Vec<DistanceConstraint> = Vec::new();
    for _ in 0..n_envs {
        entities.push(Entity::new(
            Transform2D { position: Vector2::new(0.00, -1.0), rotation: 0.0 }, 
            Shape::Rectangle(0.8, 0.4), 
            1000.0));
    }
    for i in 0..n_envs {
        let mut ent = Entity::new(
            Transform2D { position: Vector2::new(1.5, -1.0), rotation: 0.0 }, 
            Shape::Rectangle(1.5, 0.1), 
            1000.0);

        ent.angular_velocity = (i as f32).sin();
        entities.push(ent);

        dist_const.push(DistanceConstraint { 
            body_index_1: i, 
            body_index_2: n_envs + i, 
            local_attach_1: Vector2::zero(), 
            local_attach_2: Vector2::new(1.5, 0.0), 
            target_distance: 0.001, 
            compliance: 0.0 });
    }

    let state = State {
        entities: RefCell::new(entities),
        dist_const: RefCell::new(dist_const),
    };

    let mut rng = rand::rng();
    let mut controls = vec![Vector2::zero(); n_envs];
    for ctr in &mut controls {
        ctr.x = rng.random_range(-10.0..10.0);
    }
    
    let substeps = 16;
    let mut totral_time = 0.0;
    let mut frame = 0;
    // let physics_dt: f32 = 1.0 / 60.0 / substeps as f32;

    while !renderer.should_close() {
        let dt = renderer.get_delta_time();
        frame += 1;
        totral_time += dt;

        if frame % 5 == 0 {
            for ctr in &mut controls {
                ctr.x += rng.random_range(-1.0..1.0);
            }
        }

        if totral_time > 0.7
        {
            let physics_dt: f32 = dt / substeps as f32;
            for _ in 0..substeps {
                
                //apply control
                {
                    let mut ents = state.entities.borrow_mut();
                    for ie in 0..n_envs {
                        let ent = &mut ents[ie];
                        ent.velocity += controls[ie] * physics_dt;
                    }
                }

                solver.step(&state, physics_dt);
            }
        }
        
        renderer.interact(&state);
        renderer.render(&state);
    
    }

}