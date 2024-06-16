use crate::{collision_world::*, world_collider::WorldColliderHandle};
use rand::Rng;
use raylib::prelude::*;

pub fn spawn_debug_colldier_world(
    debug_colliders: &mut Vec<WorldColliderHandle>,
    collision_world: &mut CollisionWorld,
) {
    for _ in 0..100 {
        let size_x = rand::thread_rng().gen_range(2.0..5.0);
        let size_y = rand::thread_rng().gen_range(2.0..5.0);
        let pos_x = rand::thread_rng().gen_range(-50.0..50.0);
        let pos_y = rand::thread_rng().gen_range(-50.0..50.0);
        debug_colliders.push(collision_world.spawn_collider(
            RigidBodyArgs {
                dynamic: false,
                pos: Vector2::new(pos_x, pos_y),
                vel: Vector2::zero(),
                user_data: 0,
            },
            ColliderArgs {
                density: 1.0,
                restitution: 0.5,
                friction: 0.5,
            },
            ShapeArgs::Cuboid {
                half_extents: Vector2::new(size_x, size_y),
            },
        ));
    }
}
