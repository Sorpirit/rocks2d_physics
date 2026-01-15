use std::hint::black_box;
use rand::Rng;
use criterion::{criterion_group, criterion_main, Criterion};
use raylib::math::Vector2;
use rocks2d_physics::{Control, Joint, PrismaticJoint, RevoluteJoint, RigidParameters, Scene, Shape, State, Transform2D, XPBDSolver};

fn create_env(num_envs: u32) -> (State, State, Control, Scene, XPBDSolver) {
    let solver = XPBDSolver::new();
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

    scene.replicate(num_envs);

    let control = scene.build_control();
    let state_in = scene.build_state();
    let state_out = scene.build_state();

    (state_in, state_out, control, scene, solver)
}

fn simulate<'a>(mut state_in: &'a mut State, mut state_out: &'a mut State, control: &mut Control, scene: &Scene, solver: &XPBDSolver, steps: u32, dt: f32) {
    let substeps = 16;
    let physics_dt: f32 = dt / substeps as f32;

    let mut rng = rand::rng();
    for bi in 0..state_in.body_count {
        if bi % 2 == 0 {
            continue;
        }
        control.update_forces(bi, (Vector2::new(rng.random_range(-20.0..20.0), 0.0), 0.0));
    }

    for _ in 0..steps {
        for _ in 0..substeps {
            solver.step(&mut state_in, &mut state_out, &control, &scene, physics_dt);
            std::mem::swap(&mut state_in, &mut state_out);
        }
    }
}

fn bench_scene_creation(c: &mut Criterion) {
    c.bench_function("create_pendelum_64", |b| b.iter(|| create_env(64)));
    c.bench_function("create_pendelum_128", |b| b.iter(|| create_env(128)));
    c.bench_function("create_pendelum_2048", |b| b.iter(|| create_env(2048)));
}

fn bench_pendelum(c: &mut Criterion) {
    let (mut state_in, mut state_out, mut control, scene, solver) = create_env(64);
    c.bench_function("sim_pendelum_64", |b| b.iter(|| simulate(&mut state_in, &mut state_out, &mut control, &scene, &solver, 1024, 1.0 / 60.0)));

    let (mut state_in, mut state_out, mut control, scene, solver) = create_env(128);
    c.bench_function("sim_pendelum_128", |b| b.iter(|| simulate(&mut state_in, &mut state_out, &mut control, &scene, &solver, 1024, 1.0 / 60.0)));

    let (mut state_in, mut state_out, mut control, scene, solver) = create_env(2048);
    c.bench_function("sim_pendelum_2048", |b| b.iter(|| simulate(&mut state_in, &mut state_out, &mut control, &scene, &solver, 1024, 1.0 / 60.0)));
}

criterion_group!(physics_sim, 
    bench_pendelum,
    bench_scene_creation,
);
criterion_main!(physics_sim);