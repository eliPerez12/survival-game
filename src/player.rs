use raylib::prelude::*;
use crate::world_collider::*;
use crate::collision_world::*;
use crate::ImprovedCamera;
use crate::RaylibVector2;

pub struct Player {
    pub collider: WorldColliderHandle,
    pub health: f32,
}

impl Player {
    pub fn new(collision_world: &mut CollisionWorld) -> Self {
        Player {
            collider: collision_world.spawn_collider(
                RigidBodyArgs {
                    dynamic: true,
                    pos: Vector2::new(2.0, 2.0),
                    vel: Vector2::zero(),
                    user_data: 0
                },
                ColliderArgs::default(),
                ShapeArgs::Ball { radius: 1.0 },
            ),
            health: 100.0,
        }
    }

    pub fn control_movement(&mut self, rl: &RaylibHandle, collision_world: &mut CollisionWorld) {
        let mut movement_vector = Vector2::new(0.0, 0.0);
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
        self.handle_movement(rl, collision_world, &mut movement_vector);

    }

    pub fn handle_movement(&mut self, rl: &RaylibHandle, collision_world: &mut CollisionWorld, movement_vector: &mut Vector2) {
        let player_speed = 25.0 * self.collider.get_mass(collision_world);
        let player_acceleration = player_speed * rl.get_frame_time();
        let player_max_speed = match rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
            false => 3.0,
            true => 6.0,
        };
        let player_drag = player_speed / player_max_speed * rl.get_frame_time();
        let drag_vector = -self.collider.get_linvel(collision_world);
        self.collider
            .apply_impulse(drag_vector * player_drag, collision_world);
        movement_vector.normalize();
        self.collider
            .apply_impulse(*movement_vector * player_acceleration, collision_world);
    }

    pub fn apply_collision_damage(&mut self, collision_world: &mut CollisionWorld, bullets: &mut Vec<WorldColliderHandle>) {
        let mut bullet = None;
        for collision in collision_world.rapier.narrow_phase.contact_pairs_with(self.collider.collider_handle) {
            let other_collider_handle = if self.collider.collider_handle == collision.collider1 {
                collision.collider2
            } else {
                collision.collider1
            };
            let other_collider = &collision_world.rapier.collider_set.get(other_collider_handle);
            if other_collider.is_none() {
                break
            }
            let other_collider = other_collider.unwrap();
            let other_rigid_body_handle = other_collider.parent().unwrap();
            let other_rigid_body = &collision_world.rapier.rigid_body_set[other_rigid_body_handle];
            let other_collider_speed = other_rigid_body.linvel().to_raylib_vector2().length();
            let player_deflection_level = 50.0;
            if other_rigid_body.user_data == ColliderUserData::BULLET && dbg!(other_collider_speed) > player_deflection_level {
                let bullet_damage = (other_collider_speed - player_deflection_level).min(25.0);
                self.health -= dbg!(bullet_damage);
                bullet = Some((WorldColliderHandle {
                    rigid_body_handle: other_rigid_body_handle,
                    collider_handle: other_collider_handle
                }, 
                other_rigid_body.linvel().to_raylib_vector2().normalized() * other_collider_speed * other_collider.mass()
            ));
                break
            }
        }
        if let Some((bullet, force)) = bullet {
            bullets.retain(|b| *b != bullet);
            collision_world.delete_collider(bullet);
            self.collider.apply_impulse(force, collision_world);
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, camera: &Camera2D, collision_world: &mut CollisionWorld, player_texture: &Texture2D, aimed_at: Vector2) {
        let player_pos = self.collider.get_pos(collision_world);
        let angle_to_mouse = self
            .collider
            .get_pos(collision_world)
            .angle_to(camera.to_world(aimed_at))
            .to_degrees()
            - 90.0;
        d.draw_texture_pro(
            player_texture,
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
        d.draw_text(&self.health.to_string(), player_pos.x as i32, player_pos.y as i32, (20.0 * camera.zoom) as i32, Color::WHITE);
    }
}