use crate::{collision_world::*, world_collider::WorldColliderHandle, Player};
use rand::Rng;
use raylib::prelude::*;


pub struct GameWorld {
    pub bullets: Vec<WorldColliderHandle>,
    pub dummies: Vec<Player>,
}

impl GameWorld {
    pub fn new() -> Self {
        GameWorld {
            bullets: vec![],
            dummies: vec![],
        }
    }

    pub fn apply_damage_dummies(&mut self, rl: &mut RaylibHandle, collision_world: &mut CollisionWorld) {
        for dummy in &mut self.dummies {
            dummy.apply_collision_damage(collision_world, &mut self.bullets);
            dummy.handle_movement(rl, collision_world, &mut Vector2::zero());
        }
    }
}



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
                user_data: ColliderUserData::WALL,
            },
            ColliderArgs {
                density: 1.0,
                restitution: 0.5,
                friction: 0.5,
                user_data: ColliderUserData::WALL,
            },
            ShapeArgs::Cuboid {
                half_extents: Vector2::new(size_x, size_y),
            },
        ));
    }
}
