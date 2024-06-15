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

struct Player{
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
                ColliderArgs ::default(),
                ShapeArgs::Ball {  radius: 1.0 },
            )
        }
    }

    fn handle_movement(&mut self, rl: &RaylibHandle, collision_world: &mut CollisionWorld) {
        let player_acceleration = 0.01;
        let player_stopping_power = 0.005;
        let player_max_velocity: f32 = if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
            6.6
        } else {
            3.0
        };
        let mut player_input_movement = false;
        let mut movement_vector = Vector2::new(0.0, 0.0);

        if rl.is_key_down(KeyboardKey::KEY_W) {
            movement_vector.y -= player_acceleration;
            player_input_movement = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            movement_vector.y += player_acceleration;
            player_input_movement = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            movement_vector.x -= player_acceleration;
            player_input_movement = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            movement_vector.x += player_acceleration;
            player_input_movement = true;
        }

        if player_input_movement {
            // Normalize the movement vector if diagonal movement occurs
            if movement_vector.length() > player_acceleration {
                movement_vector = movement_vector.normalized() * player_acceleration;
            }

            // Apply acceleration in the input direction
            self.collider.add_linvel(movement_vector, collision_world);
            
            // Apply stopping power only to the components of the velocity that do not align with the input direction
            let current_velocity = self.collider.get_linvel(collision_world);
            let mut stopping_vector = current_velocity;

            if movement_vector.x != 0.0 {
                stopping_vector.x = 0.0; // No drag in the x-direction
            } else {
                stopping_vector.x *= player_stopping_power;
            }

            if movement_vector.y != 0.0 {
                stopping_vector.y = 0.0; // No drag in the y-direction
            } else {
                stopping_vector.y *= player_stopping_power;
            }

            self.collider.add_linvel(-stopping_vector, collision_world);
        } else {
            // Apply stopping power when no input is detected
            let current_velocity = self.collider.get_linvel(collision_world);
            let stopping_vector = current_velocity * player_stopping_power;
            self.collider.add_linvel(-stopping_vector, collision_world);
        }

        // Limit the player's velocity to the maximum velocity
        let current_velocity = self.collider.get_linvel(collision_world);
        if current_velocity.length() > player_max_velocity {
            let capped_velocity = current_velocity.normalized() * player_max_velocity;
            self.collider.set_linvel(capped_velocity, collision_world);
        }
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

    let _debug_collider = collision_world.spawn_collider(
        RigidBodyArgs {
            dynamic: false,
            pos: Vector2::zero(),
            vel: Vector2::zero(),
        },
        ColliderArgs ::default(),
        ShapeArgs::Cuboid { half_extents: Vector2::new(1.0, 1.0) },
    );

    let player_texture = rl.load_texture_from_image(&thread, &Image::load_image_from_mem(".png", include_bytes!("..//assets//rifle.png")).unwrap()).unwrap();


    while !rl.window_should_close() {
        /*
         * Update
         */

        debugger.update(&mut rl);
        player.handle_movement(&mut rl, &mut collision_world);
        camera.handle_camera_controls(&rl);
        camera.track(
            player.collider.get_center_of_mass(&collision_world),
            Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32),
        );
        
         

        debugger.add(format!("FPS: {}", rl.get_fps()));
        debugger.add(format!(
            "Num Colliders: {}",
            collision_world.colliders.len()
        ));
        debugger.add(format!(
            "Player Speed: {:?} m/s",
            player.collider.get_vel(&collision_world).length()
        ));

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
        //self.collider.draw(&collision_world, camera, &mut d);
        let player_screen_pos = camera.to_screen(player.collider.get_pos(&collision_world));
        let angle_to_mouse = player.collider.get_pos(&collision_world).angle_to(camera.to_world(d.get_mouse_position())).to_degrees() - 90.0;
        d.draw_texture_pro(
            &player_texture,
            Rectangle::new(0.0, 0.0, player_texture.width() as f32, player_texture.height() as f32),
            Rectangle::new(player_screen_pos.x, player_screen_pos.y, player_texture.width() as f32 / 10.0 * camera.zoom, player_texture.width() as f32 / 10.0 * camera.zoom),
            Vector2::new(3.2 * camera.zoom, 3.2 * camera.zoom),
            angle_to_mouse,
            Color::WHITE,
        );

        // UI
        debugger.draw(&mut d);
    }
}
