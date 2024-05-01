use collision_world::*;
use debug::DebugInfo;
use rapier2d::na::Isometry2;
use rapier2d::na::Vector2 as Vec2;
use rapier2d::prelude::*;
use raylib::prelude::*;
use world_collider::WorldColliderHandle;

use crate::rapier_world::*;
use crate::traits::*;

mod collision_world;
mod debug;
mod draw_collider;
mod rapier_world;
mod traits;
mod world_collider;

pub fn add_bounds(collision_world: &mut CollisionWorld, colliders: &mut Vec<WorldColliderHandle>) {
    colliders.push(collision_world.new_cuboid(
        Vector2::new(0.0, 100.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(100.0, 1.0),
        true,
    ));
    colliders.push(collision_world.new_cuboid(
        Vector2::new(0.0, -100.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(100.0, 1.0),
        true,
    ));
    colliders.push(collision_world.new_cuboid(
        Vector2::new(-100.0, 0.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, 100.0),
        true,
    ));
    colliders.push(collision_world.new_cuboid(
        Vector2::new(100.0, 0.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, 100.0),
        true,
    ));
}

fn main() {
    let (mut rl, thread) = raylib::init().size(1080, 720).title("Physics").build();
    let mut camera = Camera2D {
        zoom: 25.0,
        ..Default::default()
    };
    let mut collision_world = CollisionWorld::default();
    let mut colliders = vec![];
    let mut debugger = DebugInfo::new();
    add_bounds(&mut collision_world, &mut colliders);

    for x in 0..50 {
        for y in 0..50 {
            let rand = rl.get_random_value::<i32>(1..3);
            if rand == 1 {
                colliders.push(collision_world.new_ball(
                    Vector2::new(x as f32, y as f32),
                    Vector2::zero(),
                    0.5,
                    false,
                ));
            } else if rand == 2 {
                colliders.push(collision_world.new_cuboid(
                    Vector2::new(x as f32, y as f32),
                    Vector2::zero(),
                    Vector2::new(0.5, 0.5),
                    false,
                ));
            } else if rand == 3 {
                colliders.push(collision_world.new_triangle(
                    Vector2::new(x as f32, y as f32),
                    Vector2::new(0.0, 0.0),
                    (
                        Vector2::new(0.0, -0.5),
                        Vector2::new(0.5, 0.5),
                        Vector2::new(-0.5, 0.5),
                    ),
                    false,
                ))
            }
        }
    }

    let mut player_collider = collision_world.new_compound(
        Vector2::new(-10.0, 0.0),
        Vector2::zero(),
        vec![
            (
                Isometry2::new(
                    rapier2d::na::Vector2::from_raylib_vector2(Vector2::new(0.0, 0.0)),
                    0.0,
                ),
                SharedShape::new(Cuboid::new(Vec2::new(2.0, 2.0))),
            ),
            (
                Isometry2::new(
                    rapier2d::na::Vector2::from_raylib_vector2(Vector2::new(2.0, 0.0)),
                    0.0,
                ),
                SharedShape::new(Ball::new(2.0)),
            ),
        ],
        false,
    );

    while !rl.window_should_close() {
        /*
         * Update
         */

        debugger.update(&mut rl);
        camera.handle_camera_controls(&rl);

        let player_pos = player_collider.get_pos(&collision_world);
        let mouse_pos = camera.to_world(rl.get_mouse_position());
        let d = (mouse_pos - player_pos).normalized();
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            player_collider.add_vel(d * 5000.0 * rl.get_frame_time(), &mut collision_world);
        }

        debugger.add(format!("FPS: {}", rl.get_fps()));
        debugger.add(format!("Num Colliders: {}", colliders.len()));

        player_collider.get_angle(&collision_world);
        collision_world.step(&rl);

        /*
         * Drawing
         */

        // World
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        for collider in &colliders {
            collider.draw(&collision_world, camera, &mut d);
        }
        player_collider.draw(&collision_world, camera, &mut d);

        // UI
        debugger.draw(&mut d);
    }
}
