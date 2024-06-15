use collision_world::*;
use debug::DebugInfo;
use raylib::prelude::*;
use world::*;
use world_collider::WorldColliderHandle;

use crate::rapier_world::*;
use crate::traits::*;

mod collision_world;
mod debug;
mod draw_collider;
mod rapier_world;
mod traits;
mod world;
mod world_collider;

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

    let mut player_collider = collision_world.spawn_compound(
        RigidBodyArgs {
            dynamic: true,
            pos: Vector2::new(20.0, 20.0),
            vel: Vector2::zero(),
        },
        ColliderArgs::default(),
        vec![
            (
                Vector2::zero(),
                ShapeArgs::Cuboid {
                    half_extents: Vector2::new(2.0, 2.0),
                },
            ),
            (Vector2::new(2.0, 0.0), ShapeArgs::Ball { radius: 2.0 }),
        ],
    );

    let mut time_since_shot = 0.0;

    while !rl.window_should_close() {
        /*
         * Update
         */

        debugger.update(&mut rl);
        camera.handle_camera_controls(&rl);
        camera.track(
            player_collider.get_center_of_mass(&collision_world),
            Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32),
        );

        let ang_vel = player_collider.get_angvel(&collision_world);
        let rcs_torque = 2500.0;
        let stabilizer_torque = 800.0;
        let engine_force = 1000.0;

        let mut player_moving_ship = false;

        let player_pos = player_collider.get_pos(&collision_world);
        time_since_shot += rl.get_frame_time();

        if rl.is_key_down(KeyboardKey::KEY_SPACE) && time_since_shot > 0.1 {
            time_since_shot = 0.0;
            let angle = player_collider.get_angle(&collision_world);
            let d = Vector2::new(angle.cos(), angle.sin());
            collision_world.spawn_collider(
                RigidBodyArgs {
                    dynamic: true,
                    pos: player_pos + d * 10.0,
                    vel: d * 250.0,
                },
                ColliderArgs {
                    density: 10.0,
                    ..Default::default()
                },
                ShapeArgs::Ball { radius: 0.5 },
            );
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            let angle = player_collider.get_angle(&collision_world);
            let d = Vector2::new(angle.cos(), angle.sin());
            player_collider.add_linvel(d * engine_force * rl.get_frame_time(), &mut collision_world)
        }
        if rl.is_key_down(KeyboardKey::KEY_E) {
            let angle = player_collider.get_angle(&collision_world) + std::f32::consts::PI / 2.0;
            let d = Vector2::new(angle.cos(), angle.sin());
            player_collider.add_linvel(d * engine_force * rl.get_frame_time(), &mut collision_world)
        }
        if rl.is_key_down(KeyboardKey::KEY_Q) {
            let angle = player_collider.get_angle(&collision_world) - std::f32::consts::PI / 2.0;
            let d = Vector2::new(angle.cos(), angle.sin());
            player_collider.add_linvel(d * engine_force * rl.get_frame_time(), &mut collision_world)
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            let angle = player_collider.get_angle(&collision_world);
            let d = Vector2::new(angle.cos(), angle.sin());
            player_collider.add_linvel(
                -d * engine_force * rl.get_frame_time(),
                &mut collision_world,
            )
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            player_moving_ship = true;
            player_collider.add_angvel(
                -stabilizer_torque * rl.get_frame_time(),
                &mut collision_world,
            );
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            player_moving_ship = true;
            player_collider.add_angvel(
                stabilizer_torque * rl.get_frame_time(),
                &mut collision_world,
            );
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let mouse_angle = player_pos.angle_to(camera.to_world(rl.get_mouse_position()));
            let ship_angle = player_collider.get_angle(&collision_world);
            dbg!(mouse_angle, ship_angle);
        }

        if ang_vel > 0.0 && !player_moving_ship {
            player_collider.add_angvel(-rcs_torque * rl.get_frame_time(), &mut collision_world)
        }
        if ang_vel < 0.0 && !player_moving_ship {
            player_collider.add_angvel(rcs_torque * rl.get_frame_time(), &mut collision_world)
        }
        if ang_vel > rcs_torque && ang_vel < -rcs_torque {
            player_collider.set_angvel(0.0, &mut collision_world)
        }

        debugger.add(format!("FPS: {}", rl.get_fps()));
        debugger.add(format!(
            "Num Colliders: {}",
            collision_world.colliders.len()
        ));
        debugger.add(format!(
            "Player Speed: {:?} m/s",
            player_collider.get_vel(&collision_world.rapier).length()
        ));

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
