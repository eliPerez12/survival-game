use crate::{collision_world::*, world_collider::WorldColliderHandle, Player, traits::*};
use rand::Rng;
use raylib::prelude::*;


pub struct GameWorld {
    pub bullets: Vec<WorldColliderHandle>,
    pub dummies: Vec<Player>,
}

impl GameWorld {
    pub fn new() -> Self {
        GameWorld {
            bullets: vec![],
            dummies: vec![],
        }
    }

    pub fn apply_damage_dummies(&mut self, rl: &mut RaylibHandle, collision_world: &mut CollisionWorld) {
        for dummy in &mut self.dummies {
            dummy.apply_collision_damage(collision_world, &mut self.bullets);
            dummy.handle_movement(rl, collision_world, &mut Vector2::zero());
        }
    }

    pub fn handle_bullet_physics(&mut self, rl: &RaylibHandle, collision_world: &mut CollisionWorld) {
        for bullet in &mut self.bullets {
            let bullet_speed = bullet.get_vel(collision_world);
            let bullet_inerta = bullet_speed * bullet.get_mass(collision_world);
            let drag = 1.1;
            bullet.apply_impulse(
                -bullet_inerta / drag * rl.get_frame_time(),
                collision_world,
            )
        }

        self.bullets.retain(|bullet_handle| {
            if bullet_handle.get_vel(collision_world).length() < 3.5 {
                collision_world.delete_collider(bullet_handle.clone());
                false
            } else {
                true
            }
        });
    }

    pub fn render_bullets(&mut self, d: &mut RaylibDrawHandle, camera: &Camera2D, collision_world: &mut CollisionWorld) {
        let camera_world_rect = camera.to_world_rect(&Rectangle::new(
            0.0,
            0.0,
            d.get_screen_width() as f32 * 1.1,
            d.get_screen_height() as f32 * 1.1,
        ));
        for bullet in &self.bullets {
            let bounding_sphere = bullet.get_bounding_sphere(collision_world);
            if camera_world_rect.check_collision_circle_rec(
                bounding_sphere.center().coords.to_raylib_vector2(),
                bounding_sphere.radius,
            ) {
                bullet.draw(collision_world, camera, d);
            }
        }
    }

    pub fn render_dummies(&self, player: &Player, camera: &Camera2D, collision_world: &mut CollisionWorld, d: &mut RaylibDrawHandle, player_texture: &Texture2D) {
        let player_screen_pos = camera.to_screen(player.collider.get_pos(collision_world));
        let camera_world_rect = camera.to_world_rect(&Rectangle::new(
            0.0,
            0.0,
            d.get_screen_width() as f32 * 1.25,
            d.get_screen_height() as f32 * 1.25,
        ));
        for dummy in &self.dummies {
            let bounding_sphere = dummy.collider.get_bounding_sphere(collision_world);
            let player_bounding_sphere = player.collider.get_bounding_sphere(collision_world);
            let player_pos = player.collider.get_pos(collision_world);
            let dummy_pos = dummy.collider.get_pos(collision_world);
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
                        d,
                        camera,
                        collision_world,
                        player_texture,
                        player_screen_pos,
                    );
                }
            }
        }
    }
}



pub fn spawn_debug_colldier_world(
    debug_colliders: &mut Vec<WorldColliderHandle>,
    collision_world: &mut CollisionWorld,
) {
    for _ in 0..100 {
        let size_x = rand::thread_rng().gen_range(2.0..5.0);
        let size_y = rand::thread_rng().gen_range(2.0..5.0);
        let pos_x = rand::thread_rng().gen_range(-50.0..50.0);
        let pos_y = rand::thread_rng().gen_range(-50.0..50.0);
        debug_colliders.push(collision_world.spawn_collider(
            RigidBodyArgs {
                dynamic: false,
                pos: Vector2::new(pos_x, pos_y),
                vel: Vector2::zero(),
                user_data: ColliderUserData::WALL,
            },
            ColliderArgs {
                density: 1.0,
                restitution: 0.5,
                friction: 0.7,
                user_data: ColliderUserData::WALL,
            },
            ShapeArgs::Cuboid {
                half_extents: Vector2::new(size_x, size_y),
            },
        ));
    }
}
