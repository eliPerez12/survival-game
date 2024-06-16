use collision_world::*;
use debug::DebugInfo;
use raylib::prelude::*;
use crate::rapier_world::*;
use crate::traits::*;
use crate::player::*;

mod collision_world;
mod debug;
mod draw_collider;
mod rapier_world;
mod traits;
mod world;
mod world_collider;
mod player;


fn main() {
    let (mut rl, thread) = raylib::init().size(1080, 720).title("Physics").vsync().build();
    let mut camera = Camera2D {
        offset: Vector2::new(0.0, 0.0),
        zoom: 100.0,
        ..Default::default()
    };
    let mut collision_world = CollisionWorld::default();
    let mut debugger = DebugInfo::new();
    let mut player = Player::new(&mut collision_world);
    let mut bullets = Vec::with_capacity(1000);

    let mut dummies: Vec<Player> = vec![];

    let debug_collider = collision_world.spawn_collider(
        RigidBodyArgs {
            dynamic: false,
            pos: Vector2::zero(),
            vel: Vector2::zero(),
            user_data: 0
        },
        ColliderArgs {
            density: 1.0,
            restitution: 0.5,
            friction: 0.5,
        },
        ShapeArgs::Cuboid {
            half_extents: Vector2::new(1.0, 10.0),
        },
    );
    

    let player_texture = rl
        .load_texture_from_image(
            &thread,
            &Image::load_image_from_mem(".png", include_bytes!("..//assets//rifle.png")).unwrap(),
        )
        .unwrap();

    while !rl.window_should_close() {
        /*
         * Update
         */
        debugger.update(&mut rl);
        collision_world.step(&rl);
        player.apply_collision_damage(&mut collision_world, &mut bullets);
        for dummy in &mut dummies {
            dummy.apply_collision_damage(&mut collision_world, &mut bullets);
            dummy.handle_movement(&rl, &mut collision_world, &mut Vector2::zero());
        }
        player.control_movement(&rl, &mut collision_world);
        camera.handle_camera_controls(&rl);
        camera.track(
            player.collider.get_center_of_mass(&collision_world),
            Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32),
        );

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let d = (camera.to_world(rl.get_mouse_position())
                - player.collider.get_pos(&collision_world))
            .normalized();
            let bullet_speed = 140.0;
            let bullet_radius = 0.1;
            bullets.push(collision_world.spawn_collider(
                RigidBodyArgs {
                    dynamic: true,
                    pos: player.collider.get_pos(&collision_world) + d * 1.5,
                    vel: d * bullet_speed + player.collider.get_linvel(&collision_world),
                    user_data: ColliderUserData::BULLET,
                },
                ColliderArgs {
                    density: 1.5,
                    restitution: 0.0,
                    friction: 0.0,
                },
                ShapeArgs::Ball {
                    radius: bullet_radius,
                },
            ));
        }

        if rl.is_key_pressed(KeyboardKey::KEY_G) {
            dummies.push(
                {
                    let dummy = Player::new(&mut collision_world);
                    dummy.collider.set_pos(camera.to_world(rl.get_mouse_position()), &mut collision_world);
                    dummy
                }
            )
        }

        for bullet in &mut bullets {
            let bullet_speed = bullet.get_vel(&collision_world);
            let bullet_inerta = bullet_speed * bullet.get_mass(&collision_world);
            let drag = 1.1;
            bullet.apply_impulse(
                -bullet_inerta/drag*rl.get_frame_time(),
                &mut collision_world,
            )
        }

        bullets.retain(|bullet_handle| {
            if bullet_handle.get_vel(&collision_world).length() < 5.0 {
                collision_world.delete_collider(bullet_handle.clone());
                false
            } else {
                true
            }
        });

        debugger.add(format!("Game FPS: {}", rl.get_fps()));
        debugger.add(format!(
            "Physics FPS: {}",
            (1.0 / collision_world.rapier.integration_parameters.dt) as i32
        ));
        debugger.add(format!(
            "Num Colliders: {}",
            collision_world.rapier.rigid_body_set.len()
        ));
        debugger.add(format!(
            "Player Speed: {:?} m/s",
            player.collider.get_vel(&collision_world).length()
        ));
        debugger.add(format!(
            "Health: {:?} ",
            player.health
        ));

        /*
         * Drawing
         */

        // World
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        let camera_world_rect = camera.to_world_rect(&Rectangle::new(
            0.0,
            0.0,
            d.get_screen_width() as f32,
            d.get_screen_height() as f32,
        ));
        let mut rendering_colliders = 0;
        for bullet in &bullets {
            let bounding_sphere = bullet.get_bounding_sphere(&collision_world);
            if camera_world_rect.check_collision_circle_rec(
                bounding_sphere.center().coords.to_raylib_vector2(),
                bounding_sphere.radius,
            ) {
                bullet.draw(&collision_world, camera, &mut d);
                rendering_colliders += 1;
            }
        }
        debug_collider.draw(&collision_world, camera, &mut d);
        let player_screen_pos = camera.to_screen(player.collider.get_pos(&collision_world));
        let mouse_pos = d.get_mouse_position();
        for dummy in &dummies {
            dummy.render(&mut d, &camera, &mut collision_world, &player_texture, player_screen_pos)
        }
        player.render(&mut d, &camera, &mut collision_world, &player_texture, mouse_pos);
        debugger.add(format!("Drawing colliders: {:?}", rendering_colliders));

        // UI
        debugger.draw(&mut d);
    }
}
