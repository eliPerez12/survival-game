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

pub fn add_bounds(collision_world: &mut CollisionWorld) {
    collision_world.new_cuboid(
        Vector2::new(0.0, 100.0),
        Vector2::new(0.0, 0.0),
        1.0,
        Vector2::new(100.0, 1.0),
        true,
    );
    collision_world.new_cuboid(
        Vector2::new(0.0, -100.0),
        Vector2::new(0.0, 0.0),
        1.0,
        Vector2::new(100.0, 1.0),
        true,
    );
    collision_world.new_cuboid(
        Vector2::new(-100.0, 0.0),
        Vector2::new(0.0, 0.0),
        1.0,
        Vector2::new(1.0, 100.0),
        true,
    );
    collision_world.new_cuboid(
        Vector2::new(100.0, 0.0),
        Vector2::new(0.0, 0.0),
        1.0,
        Vector2::new(1.0, 100.0),
        true,
    );
}

fn add_random_colliders(collision_world: &mut CollisionWorld, rl: &RaylibHandle) {
    let offset = 80.0;
    for x in 0..40 {
        for y in 0..40 {
            let shape_half_size = rl.get_random_value::<i32>(5..10) as f32/10.0;
            let rand = rl.get_random_value::<i32>(1..3);
            if rand == 1 {
                collision_world.new_ball(
                    Vector2::new(x as f32 * 2.0 - offset, y as f32 * 2.0 - offset),
                    Vector2::zero(),
                    shape_half_size,
                    1.0,
                    false,
                );
            } else if rand == 2 {
                collision_world.new_cuboid(
                    Vector2::new(x as f32 * 2.0 - offset, y as f32 * 2.0 - offset),
                    Vector2::zero(),
                    1.0,
                    Vector2::new(shape_half_size, shape_half_size),
                    false,
                );
            } else if rand == 3 {
                collision_world.new_triangle(
                    Vector2::new(x as f32 * 2.0- offset, y as f32 * 2.0 - offset),
                    Vector2::zero(),
                    1.0,
                    (
                        Vector2::new(0.0, -shape_half_size),
                        Vector2::new(shape_half_size, shape_half_size),
                        Vector2::new(-shape_half_size, shape_half_size),
                    ),
                    false,
                );
            }
        }
    }
}


fn main() {
    let (mut rl, thread) = raylib::init().size(1080, 720).title("Physics").build();
    let mut camera = Camera2D {
        offset: Vector2::new(100.0, 100.0),
        zoom: 3.5,
        ..Default::default()
    };

    let mut collision_world = CollisionWorld::default();
    let mut debugger = DebugInfo::new();
    add_bounds(&mut collision_world);
    add_random_colliders(&mut collision_world, &rl);

    let mut player_collider = collision_world.new_compound(
        Vector2::new(20.0, 20.0),
        Vector2::zero(),
        1.0,
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

    let mut time_since_shot = 0.0;

    while !rl.window_should_close() {
        /*
         * Update
         */

        debugger.update(&mut rl);
        camera.handle_camera_controls(&rl);
        camera.track(player_collider.get_center_of_mass(&collision_world), Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32));


        let ang_vel = player_collider.get_angvel(&collision_world);
        let rcs_torque = 250.0;
        let stabilizer_torque = 500.0;
        let engine_force = 1000.0;

        let player_pos = player_collider.get_pos(&collision_world);
        time_since_shot += rl.get_frame_time();

        if rl.is_key_down(KeyboardKey::KEY_SPACE) && time_since_shot > 0.1 {
            time_since_shot = 0.0;
            let angle = player_collider.get_angle(&collision_world);
            let d = Vector2::new(angle.cos(), angle.sin());
            collision_world.new_ball(player_pos + d * 10.0, d * 250.0, 10000.0, 5.0, false);
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            let angle = player_collider.get_angle(&collision_world);
            let d = Vector2::new(angle.cos(), angle.sin());
            player_collider.add_linvel(d * engine_force * rl.get_frame_time(), &mut collision_world)
        }
        if rl.is_key_down(KeyboardKey::KEY_E) {
            let angle = player_collider.get_angle(&collision_world) + std::f32::consts::PI/2.0;
            let d = Vector2::new(angle.cos(), angle.sin());
            player_collider.add_linvel(d * engine_force * rl.get_frame_time(), &mut collision_world)
        }
        if rl.is_key_down(KeyboardKey::KEY_Q) {
            let angle = player_collider.get_angle(&collision_world) - std::f32::consts::PI/2.0;
            let d = Vector2::new(angle.cos(), angle.sin());
            player_collider.add_linvel(d * engine_force * rl.get_frame_time(), &mut collision_world)
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            let angle = player_collider.get_angle(&collision_world);
            let d = Vector2::new(angle.cos(), angle.sin());
            player_collider.add_linvel(-d * engine_force * rl.get_frame_time(), &mut collision_world)
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            player_collider.add_angvel(-stabilizer_torque* rl.get_frame_time(), &mut collision_world);
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            player_collider.add_angvel(stabilizer_torque * rl.get_frame_time(), &mut collision_world);
        }
        if ang_vel > 0.0 {
            player_collider.add_angvel(-rcs_torque * rl.get_frame_time(), &mut collision_world)
        }
        if ang_vel < 0.0 {
            player_collider.add_angvel(rcs_torque * rl.get_frame_time(), &mut collision_world)
        }
        if ang_vel > rcs_torque && ang_vel < -rcs_torque {
            player_collider.set_angvel(0.0, &mut collision_world)
        }

        debugger.add(format!("FPS: {}", rl.get_fps()));
        debugger.add(format!("Num Colliders: {}", collision_world.colliders.len()));
        debugger.add(format!("Player Speed: {:?} m/s", player_collider.get_vel(&collision_world.rapier).length()));

        player_collider.get_angle(&collision_world);
        collision_world.step(&rl);

        /*
         * Drawing
         */

        // World
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        for collider in &collision_world.colliders {
            collider.draw(&collision_world, camera, &mut d);
        }
        player_collider.draw(&collision_world, camera, &mut d);

        // UI
        debugger.draw(&mut d);
    }
}
