use collision_world::WorldCollider;
use rapier2d::crossbeam::epoch::Shared;
use rapier2d::na::Isometry2;
use rapier2d::na::Vector2 as Vec2;
use rapier2d::prelude::*;
use raylib::prelude::*;

use crate::collision_world::*;
use crate::traits::*;

mod collision_world;
mod traits;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1080, 720)
        .title("Physics")
        .vsync()
        .build();
    let mut camera = Camera2D {
        zoom: 25.0,
        ..Default::default()
    };
    let mut collision_world = RapierCollisionWorld::default();
    let ground_collider = ColliderBuilder::cuboid(1000.0, 0.1).build();
    let ground_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![0.0, 20.0])
        .build();
    let ground_body_handle = collision_world.rigid_body_set.insert(ground_rigid_body);
    collision_world.collider_set.insert_with_parent(
        ground_collider,
        ground_body_handle,
        &mut collision_world.rigid_body_set,
    );

    /* Create the bouncing ball. */
    let mut colliders = vec![];

    for x in 0..10 {
        for y in 0..10 {
            if rl.get_random_value::<i32>(0..2) == 1 {
                colliders.push(WorldCollider::new_ball(
                    Vector2::new(x as f32, y as f32),
                    Vector2::zero(),
                    0.5,
                    false,
                    &mut collision_world.rigid_body_set,
                    &mut collision_world.collider_set,
                ));
            } else {
                colliders.push(WorldCollider::new_cuboid(
                    Vector2::new(x as f32, y as f32),
                    Vector2::zero(),
                    Vector2::new(0.5, 0.5),
                    false,
                    &mut collision_world.rigid_body_set,
                    &mut collision_world.collider_set,
                ));
            }
        }
    }

    let mut player = WorldCollider::new_cuboid(
        Vector2::new(20.0, 0.0),
        Vector2::zero(),
        Vector2::new(2.0, 2.0),
        false,
        &mut collision_world.rigid_body_set,
        &mut collision_world.collider_set,
    );

    let mut compund_shape = WorldCollider::new_compound(
        Vector2::new(30.0, 0.0),
        Vector2::zero(),
        vec![(
            Isometry2::new(
                rapier2d::na::Vector2::from_raylib_vector2(Vector2::new(0.0, 0.0)),
                0.0,
            ),
            SharedShape::new(Cuboid::new(Vec2::new(5.0, 5.0))),
        )],
        false,
        &mut collision_world.rigid_body_set,
        &mut collision_world.collider_set,
    );

    while !rl.window_should_close() {
        /*
         * Update
         */

        camera.handle_camera_controls(&rl);

        collision_world.integration_parameters.dt = rl.get_frame_time();
        collision_world.step();
        let player_pos = player.get_pos(&mut collision_world);
        let mouse_pos = camera.to_world(rl.get_mouse_position());
        let d = (mouse_pos - player_pos).normalized();
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            player.add_vel(d * GRAVITY, &mut collision_world);
        }

        let collisions = collision_world.get_collisions();

        /*
         * Drawing
         */
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        for collider in &colliders {
            collider.draw(&collision_world, camera, &mut d);
        }

        player.draw(&collision_world, camera, &mut d);
        compund_shape.draw(&collision_world, camera, &mut d);
    }
}
