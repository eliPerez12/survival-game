use crate::collision_world::*;
use crate::lighting::Light;
use crate::lighting::LightEngine;
use crate::lighting::LightHandle;
use crate::world_collider::*;
use crate::Assets;
use crate::GameWorld;
use crate::ImprovedCamera;
use crate::RaylibVector2;
use rand::Rng;
use raylib::prelude::*;

// TODO: make private
pub struct Corpse {
    pub pos: Vector2,
    pub animation_stage: i32,
    pub time_elapsed: f32,
    pub angle: f32,
}

impl Corpse {
    const ANIMATION_FRAME_TIME: f32 = 0.1;
    pub fn update_animation(&mut self, rl: &RaylibHandle) {
        if (1..4).contains(&self.animation_stage) {
            if self.time_elapsed > Self::ANIMATION_FRAME_TIME {
                self.animation_stage += 1;
                self.time_elapsed = 0.0;
            } else {
                self.time_elapsed += rl.get_frame_time();
            }
        }
    }

    pub fn render(
        &self,
        d: &mut RaylibDrawHandle,
        assets: &Assets,
        camera: &Camera2D,
        thread: &RaylibThread,
        target: &mut RenderTexture2D,
    ) {
        let mut d = d.begin_texture_mode(thread, target);
        let corpse_texture =
            assets.get_texture(&format!("corpses/corpse{}.png", self.animation_stage));
        let scale = 0.1;
        d.draw_texture_pro(
            corpse_texture,
            Rectangle::new(
                0.0,
                0.0,
                corpse_texture.width() as f32,
                corpse_texture.height() as f32,
            ),
            camera.to_screen_rect(&Rectangle::new(
                self.pos.x,
                self.pos.y,
                corpse_texture.width() as f32 * scale,
                corpse_texture.width() as f32 * scale,
            )),
            Vector2::new(
                corpse_texture.width() as f32 * scale / 2.0 * camera.zoom,
                corpse_texture.height() as f32 * scale / 2.0 * camera.zoom,
            ),
            self.angle,
            Color::WHITE,
        );
    }
}

pub struct Player {
    pub collider: WorldColliderHandle,
    pub angle: f32,
    pub health: f32,
    pub time_since_shot: f32,
    pub inventory_open: bool,
    pub player_light: LightHandle,
}

impl Player {
    const WALKING_SPEED: f32 = 4.5;
    const SPRINTING_SPEED: f32 = 8.5;
    const WALKING_ACCELERATION: f32 = 20.0;
    //const WALKING_DEACCELERATION: f32 = 18.0;

    pub fn new(collision_world: &mut CollisionWorld, light_engine: &mut LightEngine) -> Self {
        let pos = Vector2::new(20.0, 20.0);
        Player {
            collider: collision_world.spawn_collider(
                RigidBodyArgs {
                    dynamic: true,
                    pos,
                    vel: Vector2::zero(),
                    user_data: 0,
                },
                ColliderArgs::default(),
                ShapeArgs::Ball { radius: 1.0 },
            ),
            health: 100.0,
            angle: 0.0,
            time_since_shot: 0.0,
            inventory_open: false,
            player_light: light_engine
                .spawn_light(Light::Radial {
                    pos,
                    color: Vector4::new(1.0, 1.0, 1.0, 0.0),
                    radius: 15.0,
                })
                .unwrap(),
        }
    }

    pub fn aim_at(&mut self, world_pos: Vector2, collision_world: &mut CollisionWorld) {
        self.angle = self
            .collider
            .get_pos(collision_world)
            .angle_to(world_pos)
            .to_degrees()
            - 90.0;
    }

    pub fn handle_controls(
        &mut self,
        rl: &RaylibHandle,
        camera: &Camera2D,
        collision_world: &mut CollisionWorld,
    ) {
        let mut movement_vector = Vector2::new(0.0, 0.0);
        if !self.inventory_open {
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
            self.aim_at(camera.to_world(rl.get_mouse_position()), collision_world);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_I) {
            self.inventory_open = !self.inventory_open;
        }
        self.handle_movement(rl, collision_world, &mut movement_vector);
    }

    pub fn handle_movement(
        &mut self,
        rl: &RaylibHandle,
        collision_world: &mut CollisionWorld,
        movement_vector: &mut Vector2,
    ) {
        let player_speed = Self::WALKING_ACCELERATION * self.collider.get_mass(collision_world);
        let player_acceleration = player_speed * rl.get_frame_time();
        let player_max_speed = match rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
            false => Self::WALKING_SPEED,
            true => Self::SPRINTING_SPEED,
        };
        let player_drag = player_speed / player_max_speed * rl.get_frame_time();
        let drag_vector = -self.collider.get_linvel(collision_world);
        self.collider
            .apply_impulse(drag_vector * player_drag, collision_world);
        movement_vector.normalize();
        self.collider
            .apply_impulse(*movement_vector * player_acceleration, collision_world);
    }

    pub fn update_player_light(
        &mut self,
        light_engine: &mut LightEngine,
        collision_world: &mut CollisionWorld,
    ) {
        light_engine
            .get_mut_light(&self.player_light)
            .set_pos(self.collider.get_pos(collision_world))
            .set_color(Vector4::new(1.0, 1.0, 1.0, 0.15));
    }

    pub fn handle_shooting(
        &mut self,
        rl: &mut RaylibHandle,
        collision_world: &mut CollisionWorld,
        bullets: &mut Vec<WorldColliderHandle>,
        aimed_at: Vector2,
    ) {
        let accuracy = 50.0
            / (self.collider.get_linvel(collision_world).length() / Self::WALKING_SPEED * 2.0)
                .max(1.0);
        let bullet_speed = 160.0;
        let max_angle = std::f32::consts::PI / 2.0 / accuracy;
        let random_accuracy_angle = rand::thread_rng().gen_range(-max_angle..max_angle);
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
            && self.time_since_shot > 0.1
            && !self.inventory_open
        {
            self.time_since_shot = 0.0;
            let d = (aimed_at - self.collider.get_pos(collision_world)).normalized();
            let bullet_radius = 0.1;
            bullets.push(collision_world.spawn_collider(
                RigidBodyArgs {
                    dynamic: true,
                    pos: self.collider.get_pos(collision_world) + d * 2.0,
                    vel: d.rotated(random_accuracy_angle) * bullet_speed,
                    user_data: ColliderUserData::BULLET,
                },
                ColliderArgs {
                    density: 1.5,
                    restitution: 0.1,
                    friction: 0.7,
                    user_data: ColliderUserData::BULLET,
                    sensor: false,
                },
                ShapeArgs::Ball {
                    radius: bullet_radius,
                },
            ));
        } else {
            self.time_since_shot += rl.get_frame_time();
        }
    }

    pub fn handle_spawning_dunmmies(
        &self,
        rl: &RaylibHandle,
        camera: &Camera2D,
        collision_world: &mut CollisionWorld,
        game_world: &mut GameWorld,
        light_engine: &mut LightEngine,
    ) {
        let mouse_pos = rl.get_mouse_position();
        if rl.is_key_pressed(KeyboardKey::KEY_G) {
            game_world.dummies.push({
                let dummy = Player::new(collision_world, light_engine);
                dummy
                    .collider
                    .set_pos(camera.to_world(mouse_pos), collision_world);
                dummy
            })
        }
    }

    pub fn render(
        &self,
        d: &mut RaylibDrawHandle,
        camera: &Camera2D,
        collision_world: &mut CollisionWorld,
        assets: &Assets,
        thread: &RaylibThread,
        target: &mut RenderTexture2D,
    ) {
        let player_texture = assets.get_texture("rifle.png");
        let player_pos = self.collider.get_pos(collision_world);
        let mut d = d.begin_texture_mode(thread, target);
        let scale = 0.1;
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
                player_texture.width() as f32 * scale,
                player_texture.width() as f32 * scale,
            )),
            Vector2::new(
                player_texture.width() as f32 * scale / 2.0 * camera.zoom,
                player_texture.height() as f32 * scale / 2.0 * camera.zoom,
            ),
            self.angle,
            Color::WHITE,
        );
        let font_size = 1.0;
        d.draw_text(
            &self.health.to_string(),
            camera.to_screen_x(player_pos.x - font_size / 2.0) as i32,
            camera.to_screen_y(player_pos.y - font_size / 2.0) as i32,
            (1.0 * camera.zoom) as i32,
            Color::WHITE,
        );
    }

    pub fn get_corpse(&self, collision_world: &mut CollisionWorld) -> Corpse {
        Corpse {
            pos: self.collider.get_pos(collision_world),
            animation_stage: 1,
            time_elapsed: 0.0,
            angle: self.angle,
        }
    }
}
