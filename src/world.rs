use crate::collision_world::*;
use raylib::prelude::*;

pub fn add_bounds(collision_world: &mut CollisionWorld) {
    let border_size = 1000.0;
    let border_thickness = 1.0;
    collision_world.spawn_collider(
        RigidBodyArgs {
            dynamic: false,
            pos: Vector2::new(0.0, border_size),
            vel: Vector2::new(0.0, 0.0),
        },
        ColliderArgs::default(),
        ShapeArgs::Cuboid {
            half_extents: Vector2::new(border_size, border_thickness),
        },
    );
    collision_world.spawn_collider(
        RigidBodyArgs {
            dynamic: false,
            pos: Vector2::new(0.0, -border_size),
            vel: Vector2::new(0.0, 0.0),
        },
        ColliderArgs::default(),
        ShapeArgs::Cuboid {
            half_extents: Vector2::new(border_size, border_thickness),
        },
    );
    collision_world.spawn_collider(
        RigidBodyArgs {
            dynamic: false,
            pos: Vector2::new(-border_size, 0.0),
            vel: Vector2::new(0.0, 0.0),
        },
        ColliderArgs::default(),
        ShapeArgs::Cuboid {
            half_extents: Vector2::new(border_thickness, border_size),
        },
    );
    collision_world.spawn_collider(
        RigidBodyArgs {
            dynamic: false,
            pos: Vector2::new(border_size, 0.0),
            vel: Vector2::new(0.0, 0.0),
        },
        ColliderArgs::default(),
        ShapeArgs::Cuboid {
            half_extents: Vector2::new(border_thickness, border_size),
        },
    );
}

pub fn add_random_colliders(collision_world: &mut CollisionWorld, rl: &RaylibHandle) {
    let offset = 80.0;
    for x in 0..40 {
        for y in 0..40 {
            let shape_half_size = rl.get_random_value::<i32>(5..10) as f32 / 10.0;
            let rand = rl.get_random_value::<i32>(1..3);
            if rand == 1 {
                collision_world.spawn_collider(
                    RigidBodyArgs {
                        dynamic: true,
                        pos: Vector2::new(x as f32 * 2.0 - offset, y as f32 * 2.0 - offset),
                        vel: Vector2::zero(),
                    },
                    ColliderArgs::default(),
                    ShapeArgs::Ball { radius: 1.0 },
                );
            } else if rand == 2 {
                collision_world.spawn_collider(
                    RigidBodyArgs {
                        dynamic: true,
                        pos: Vector2::new(x as f32 * 2.0 - offset, y as f32 * 2.0 - offset),
                        vel: Vector2::zero(),
                    },
                    ColliderArgs::default(),
                    ShapeArgs::Cuboid {
                        half_extents: Vector2::new(shape_half_size, shape_half_size),
                    },
                );
            } else if rand == 3 {
                collision_world.spawn_collider(
                    RigidBodyArgs {
                        dynamic: true,
                        pos: Vector2::new(x as f32 * 2.0 - offset, y as f32 * 2.0 - offset),
                        vel: Vector2::zero(),
                    },
                    ColliderArgs::default(),
                    ShapeArgs::Triangle {
                        points: (
                            Vector2::new(0.0, -shape_half_size),
                            Vector2::new(shape_half_size, shape_half_size),
                            Vector2::new(-shape_half_size, shape_half_size),
                        ),
                    },
                );
            }
        }
    }
}
