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

struct Player {
    collider: WorldColliderHandle,
}

impl Player {
    pub fn new(collision_world: &mut CollisionWorld) -> Self {
        Player {
            collider: collision_world.spawn_collider(
                RigidBodyArgs {
                    dynamic: true,
                    pos: Vector2::new(2.0, 2.0),
                    vel: Vector2::zero(),
                },
                ColliderArgs::default(),
                ShapeArgs::Ball { radius: 1.0 },
            ),
        }
    }

    fn handle_movement(&mut self, rl: &RaylibHandle, collision_world: &mut CollisionWorld) {
        let player_max_speed = match rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
            false => 3.0,
            true => 6.0,
        };
        let player_speed = 25.0;
        let player_acceleration = player_speed * rl.get_frame_time();
        let player_drag = player_speed / player_max_speed * rl.get_frame_time();
        let mut movement_vector = Vector2::new(0.0, 0.0);
        let drag_vector = -self.collider.get_linvel(collision_world);
        self.collider
            .add_linvel(drag_vector * player_drag, collision_world);

        if rl.is_key_down(KeyboardKey::KEY_W) {
            movement_vector.y -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            movement_vector.y += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            movement_vector.x -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            movement_vector.x += 1.0;
        }
        movement_vector.normalize();
        self.collider
            .add_linvel(movement_vector * player_acceleration, collision_world);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(1080, 720).title("Physics").build();
    let mut camera = Camera2D {
        offset: Vector2::new(0.0, 0.0),
        zoom: 100.0,
        ..Default::default()
    };

    let mut collision_world = CollisionWorld::default();
    let mut debugger = DebugInfo::new();
    let mut player = Player::new(&mut collision_world);
    let mut bullets = Vec::with_capacity(1000);

    let debug_collider = collision_world.spawn_collider(
        RigidBodyArgs {
            dynamic: false,
            pos: Vector2::zero(),
            vel: Vector2::zero(),
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

        collision_world.step(&rl);
        debugger.update(&mut rl);
        player.handle_movement(&rl, &mut collision_world);
        camera.handle_camera_controls(&rl);
        camera.track(
            player.collider.get_center_of_mass(&collision_world),
            Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32),
        );

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let d = (camera.to_world(rl.get_mouse_position())
                - player.collider.get_pos(&collision_world))
            .normalized();
            let bullet_speed = 110.0;
            let bullet_radius = 0.1;
            bullets.push(collision_world.spawn_collider(
                RigidBodyArgs {
                    dynamic: true,
                    pos: player.collider.get_pos(&collision_world) + d * 1.5,
                    vel: d * bullet_speed + player.collider.get_linvel(&collision_world),
                },
                ColliderArgs {
                    density: 1.0,
                    restitution: 0.0,
                    friction: 0.0,
                },
                ShapeArgs::Ball {
                    radius: bullet_radius,
                },
            ));
        }

        for bullet in &mut bullets {
            let bullet_speed = bullet.get_vel(&collision_world);
            let drag = 30.0;
            bullet.add_linvel(
                -bullet_speed / (drag / rl.get_frame_time()),
                &mut collision_world,
            )
        }

        bullets.retain(|bullet_handle| {
            if bullet_handle.get_vel(&collision_world).length() < 8.0 {
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
        let player_pos = player.collider.get_pos(&collision_world);
        let angle_to_mouse = player
            .collider
            .get_pos(&collision_world)
            .angle_to(camera.to_world(d.get_mouse_position()))
            .to_degrees()
            - 90.0;
        d.draw_texture_pro(
            &player_texture,
            Rectangle::new(
                0.0,
                0.0,
                player_texture.width() as f32,
                player_texture.height() as f32,
            ),
            camera.to_screen_rect(&Rectangle::new(
                player_pos.x,
                player_pos.y,
                player_texture.width() as f32 / 10.0,
                player_texture.width() as f32 / 10.0,
            )),
            Vector2::new(3.2 * camera.zoom, 3.2 * camera.zoom),
            angle_to_mouse,
            Color::WHITE,
        );

        debugger.add(format!("Drawing colliders: {:?}", rendering_colliders));

        // UI
        debugger.draw(&mut d);
    }
}
