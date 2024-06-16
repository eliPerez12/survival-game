use crate::player::*;
use crate::rapier_world::*;
use crate::traits::*;
use collision_world::*;
use debug::DebugInfo;
use raylib::prelude::*;
use world::*;

mod collision_world;
mod debug;
mod draw_collider;
mod player;
mod rapier_world;
mod traits;
mod world;
mod world_collider;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1080, 720)
        .title("Physics")
        .vsync()
        .build();
    let mut camera = Camera2D {
        offset: Vector2::new(0.0, 0.0),
        zoom: 100.0,
        ..Default::default()
    };
    let mut collision_world = CollisionWorld::default();
    let mut game_world = GameWorld::new();
    let mut debugger = DebugInfo::new();
    let mut player = Player::new(&mut collision_world);
    let mut debug_colliders = vec![];

    spawn_debug_colldier_world(&mut debug_colliders, &mut collision_world);

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
        let mouse_pos = rl.get_mouse_position();
        debugger.update(&mut rl);
        collision_world.step(&rl);
        player.apply_collision_damage(&mut collision_world, &mut game_world.bullets);
        game_world.apply_damage_dummies(&mut rl, &mut collision_world);
        player.control_movement(&rl, &mut collision_world);
        player.handle_shooting(
            &mut rl,
            &mut collision_world,
            &mut game_world.bullets,
            camera.to_world(mouse_pos),
        );
        camera.handle_camera_controls(&rl);
        camera.track(
            player.collider.get_center_of_mass(&collision_world),
            Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32),
        );

        if rl.is_key_pressed(KeyboardKey::KEY_G) {
            game_world.dummies.push({
                let dummy = Player::new(&mut collision_world);
                dummy
                    .collider
                    .set_pos(camera.to_world(mouse_pos), &mut collision_world);
                dummy
            })
        }

        for bullet in &mut game_world.bullets {
            let bullet_speed = bullet.get_vel(&collision_world);
            let bullet_inerta = bullet_speed * bullet.get_mass(&collision_world);
            let drag = 1.1;
            bullet.apply_impulse(
                -bullet_inerta / drag * rl.get_frame_time(),
                &mut collision_world,
            )
        }

        game_world.bullets.retain(|bullet_handle| {
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
        debugger.add(format!("Health: {:?} ", player.health));

        /*
         * Drawing
         */

        // World
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        let camera_world_rect = camera.to_world_rect(&Rectangle::new(
            0.0,
            0.0,
            d.get_screen_width() as f32 * 1.25,
            d.get_screen_height() as f32 * 1.25,
        ));
        let mut rendering_colliders = 0;
        for bullet in &game_world.bullets {
            let bounding_sphere = bullet.get_bounding_sphere(&collision_world);
            if camera_world_rect.check_collision_circle_rec(
                bounding_sphere.center().coords.to_raylib_vector2(),
                bounding_sphere.radius,
            ) {
                bullet.draw(&collision_world, camera, &mut d);
                rendering_colliders += 1;
            }
        }
        for debug_collider in &debug_colliders {
            debug_collider.draw(&collision_world, camera, &mut d);
        }
        let player_screen_pos = camera.to_screen(player.collider.get_pos(&collision_world));

        for dummy in &game_world.dummies {
            let bounding_sphere = dummy.collider.get_bounding_sphere(&collision_world);
            let player_bounding_sphere = player.collider.get_bounding_sphere(&collision_world);
            let player_pos = player.collider.get_pos(&collision_world);
            let dummy_pos = dummy.collider.get_pos(&collision_world);
            let dx = dummy_pos - player_pos;
            let dn = dx.normalized();

            let ray_origin = player_pos + dn * 2.0 * player_bounding_sphere.radius;
            let ray = &rapier2d::geometry::Ray::new(
                rapier2d::na::Vector2::new(ray_origin.x, ray_origin.y).into(), 
                rapier2d::na::Vector2::new(dn.x, dn.y)
            );
            let ray_length = dx.length() - (bounding_sphere.radius + player_bounding_sphere.radius);
            fn predicate(_handle: rapier2d::geometry::ColliderHandle, collider: &rapier2d::geometry::Collider) -> bool {
                collider.shape().as_cuboid().is_some()
            }
            let intersection = collision_world.rapier.query_pipeline.cast_ray_and_get_normal(
                &collision_world.rapier.rigid_body_set,
                &collision_world.rapier.collider_set,
                ray,
                ray_length,
                true,       
                rapier2d::pipeline::QueryFilter {
                    exclude_rigid_body: Some(dummy.collider.rigid_body_handle),
                    predicate: Some(&predicate),
                    ..Default::default()
                }
            );
            if camera_world_rect.check_collision_circle_rec(
                bounding_sphere.center().coords.to_raylib_vector2(),
                bounding_sphere.radius,
            ){
                if let Some(_intersection) = intersection {
                    d.draw_circle_v(camera.to_screen(ray.origin.coords.to_raylib_vector2()), 0.1 * camera.zoom, Color::YELLOW);
                } else {
                    dummy.render(
                        &mut d,
                        &camera,
                        &mut collision_world,
                        &player_texture,
                        player_screen_pos,
                    );
                    rendering_colliders += 1;
                }
            }
        }

        player.render(
            &mut d,
            &camera,
            &mut collision_world,
            &player_texture,
            mouse_pos,
        );
        debugger.add(format!("Drawing colliders: {:?}", rendering_colliders));
        debugger.add(format!("Mouse_pos: ({:?}, {:?})", camera.to_world(mouse_pos).x as i32, camera.to_world(mouse_pos).y as i32));

        // UI
        debugger.draw(&mut d);
    }
}
