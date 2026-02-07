use raylib::{math::Vector2};

use crate::{Control, Joint, MassProperty, Scene, Transform2D, state::state::State};


// fn verify(ents: &[Entity])
// {
//     for ent in ents {
//         if ent.transform.position.x.is_infinite() || ent.transform.position.x.is_nan() || 
//             ent.transform.position.y.is_infinite() || ent.transform.position.y.is_nan() || 
//             ent.transform.rotation.is_infinite() || ent.transform.rotation.is_nan()
//         {
//             panic!("Positional value out of range");
//         }
//     }
// }

fn cross2d(v1: Vector2, v2: Vector2) -> f32 {
    (v1.x * v2.y) - (v1.y * v2.x)
}

fn integrate(state_in: &mut State, control: &Control, dt: f32)
{
    for bi in 0..state_in.body_count {
        let mut transform = state_in.query_transform(bi);
        let (mut velocity, mut angular_velocity) = state_in.query_velocity(bi);
        let (vforce, aforce) = control.query_forces(bi);

        velocity += (Vector2::new(0.0, -9.81) + vforce ) * dt;
        angular_velocity += aforce * dt;

        transform.position += velocity * dt;
        transform.rotation += angular_velocity * dt;

        state_in.update_transform(bi, transform);
        state_in.update_velocity(bi, (velocity, angular_velocity));
    }
}

fn joint_contraints(state_in: &mut State, scene: &Scene, dt: f32) {
    for joint in &scene.joints {
        match joint {
            Joint::PrismaticJoint(prismatic_joint) => {
                let mut transform = state_in.query_transform(prismatic_joint.body_i);

                let x = if transform.position.x.abs() > prismatic_joint.range_x { prismatic_joint.range_x * transform.position.x.signum() } else { transform.position.x };
                let delta = transform.position - Vector2::new(x, prismatic_joint.target_y);
                let c = delta.length();
                
                if c.abs() < 0.00001 {
                    continue;
                }

                let normal = delta.normalize();
                
                let alpha = prismatic_joint.compliance / dt.powi(2);
                let inv_mass = scene.mass_properties[prismatic_joint.body_i].inv_mass;
                let lambda = -c / (inv_mass + alpha);
                
                transform.position += normal * lambda * inv_mass;

                state_in.update_transform(prismatic_joint.body_i, transform);
            },
            Joint::RevoluteJoint(revolute_joint) => {
                let mut b1_transform = state_in.query_transform(revolute_joint.body1_i);
                let mut b2_transform = revolute_joint.body2_i.map(|b2i| state_in.query_transform(b2i)).unwrap_or(Transform2D::zero());

                let rel_1 =  Vector2::from_angle(b1_transform.rotation).rotate(revolute_joint.local_attachment_b1);
                let rel_2 =  Vector2::from_angle(b2_transform.rotation).rotate(revolute_joint.local_attachment_b2);

                let attach_p1 = rel_1 + b1_transform.position;
                let attach_p2 = rel_2 + b2_transform.position;

                let dir = attach_p2 - attach_p1;
                let c = dir.length();
                
                if c.abs() < 0.00001 {
                    continue;
                }

                let normal = -dir.normalize();
                
                let alpha = revolute_joint.compliance / dt.powi(2);

                let b1_mass_prop = scene.mass_properties[revolute_joint.body1_i];
                let b2_mass_prop = revolute_joint.body2_i.map(|b2i| scene.mass_properties[b2i]).unwrap_or(MassProperty::zero());

                let w1 = b1_mass_prop.inv_mass + b1_mass_prop.inv_inertia * cross2d(rel_1, normal).powi(2);
                let w2 = b2_mass_prop.inv_mass + b2_mass_prop.inv_inertia * cross2d(rel_2, normal).powi(2);

                let lambda = -c / (w1 + w2 + alpha);
                
                b1_transform.position += normal * lambda * b1_mass_prop.inv_mass;
                b1_transform.rotation += b1_mass_prop.inv_inertia * cross2d(rel_1, normal * lambda);
                
                if let Some(b2i) = revolute_joint.body2_i {
                    b2_transform.position -= normal * lambda * b2_mass_prop.inv_mass;
                    b2_transform.rotation += b2_mass_prop.inv_inertia * cross2d(rel_2, -normal * lambda);

                    state_in.update_transform(b2i, b2_transform);
                }

                state_in.update_transform(revolute_joint.body1_i, b1_transform);
            },
        }
    }
}

fn update_velocities(state_in: &mut State, state_out: &mut State, dt: f32)
{
    for ei in 0..state_in.body_count {
        let transform = state_in.query_transform(ei);
        let prev_transform = state_out.query_transform(ei);

        let velocity = ((transform.position - prev_transform.position) / dt) * 0.99995;
        let angular_velocity = ((transform.rotation - prev_transform.rotation) / dt) * 0.9998;
        
        state_out.update_transform(ei, transform);
        state_out.update_velocity(ei, (velocity, angular_velocity));
    }
}

pub struct XPBDSolver 
{

}

impl XPBDSolver {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn step(&self, state_in: &mut State, state_out: &mut State, control: &Control, scene: &Scene, physics_dt: f32) {
        integrate(state_in, control, physics_dt);
        joint_contraints(state_in, scene, physics_dt);
        update_velocities(state_in, state_out, physics_dt);
        // verify(&entities);
    }

}
