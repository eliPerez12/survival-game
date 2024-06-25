use crate::{
    collision_world::*, lighting::LightEngine, lighting_renderer::LightingRenderer, traits::*,
    world_collider::WorldColliderHandle, Assets, Corpse, Player,
};
use rand::Rng;
use raylib::prelude::*;

pub struct GameWorld {
    pub bullets: Vec<WorldColliderHandle>,
    pub dummies: Vec<Player>,
    pub corpses: Vec<Corpse>,
}

impl GameWorld {
    pub fn new() -> Self {
        GameWorld {
            bullets: vec![],
            dummies: vec![],
            corpses: vec![],
        }
    }
    //TODO: Fix too many args
    #[allow(clippy::too_many_arguments)]
    pub fn render_entities(
        &self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        lighting_renderer: &mut LightingRenderer,
        collision_world: &mut CollisionWorld,
        camera: &Camera2D,
        assets: &Assets,
        player: &Player,
    ) {
        self.render_bullets(
            d,
            thread,
            camera,
            collision_world,
            &mut lighting_renderer.target,
        );
        self.render_corpses(camera, d, assets, thread, &mut lighting_renderer.target);
        self.render_dummies(
            player,
            camera,
            collision_world,
            d,
            assets,
            thread,
            &mut lighting_renderer.target,
        );
    }

    pub fn handle_corpses(&mut self, rl: &RaylibHandle) {
        for corpse in &mut self.corpses {
            corpse.update_animation(rl);
        }
    }

    pub fn handle_bullet_physics(
        &mut self,
        rl: &RaylibHandle,
        collision_world: &mut CollisionWorld,
    ) {
        let drag_amount = 25.0;
        for bullet in &mut self.bullets {
            let drag_dir = -bullet.get_linvel(collision_world).normalized();
            let drag_vector = drag_dir * drag_amount * rl.get_frame_time();
            bullet.add_linvel(drag_vector, collision_world)
        }

        self.bullets.retain(|bullet_handle| {
            if bullet_handle.get_linvel(collision_world).length()
                < drag_amount * rl.get_frame_time()
            {
                collision_world.delete_collider(bullet_handle.clone());
                false
            } else {
                true
            }
        });
    }

    fn render_bullets(
        &self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        camera: &Camera2D,
        collision_world: &mut CollisionWorld,
        target: &mut RenderTexture2D,
    ) {
        let camera_world_rect = camera.get_visible_rect(Vector2::new(
            d.get_screen_width() as f32,
            d.get_screen_height() as f32,
        ));
        for bullet in &self.bullets {
            let bounding_sphere = bullet.get_bounding_sphere(collision_world);
            if camera_world_rect.check_collision_circle_rec(
                bounding_sphere.center().coords.to_raylib_vector2(),
                bounding_sphere.radius,
            ) {
                bullet.draw(collision_world, camera, d, thread, target);
            }
        }
    }

    fn render_corpses(
        &self,
        camera: &Camera2D,
        d: &mut RaylibDrawHandle,
        assets: &Assets,
        thread: &RaylibThread,
        target: &mut RenderTexture2D,
    ) {
        let camera_world_rect = camera.get_visible_rect(Vector2::new(
            d.get_screen_width() as f32,
            d.get_screen_height() as f32,
        ));
        for corpse in &self.corpses {
            if camera_world_rect.check_collision_point_rec(corpse.pos) {
                corpse.render(d, assets, camera, thread, target)
            }
        }
    }
    //TODO: Fix too many args
    #[allow(clippy::too_many_arguments)]
    fn render_dummies(
        &self,
        player: &Player,
        camera: &Camera2D,
        collision_world: &mut CollisionWorld,
        d: &mut RaylibDrawHandle,
        assets: &Assets,
        thread: &RaylibThread,
        target: &mut RenderTexture2D,
    ) {
        let camera_world_rect = camera.get_visible_rect(Vector2::new(
            d.get_screen_width() as f32,
            d.get_screen_height() as f32,
        ));
        for dummy in &self.dummies {
            for i in 0..4 {
                // check 4 diffrent points on dummy
                let offset = match i {
                    0 => Vector2::new(0.0, 1.0),
                    1 => Vector2::new(1.0, 0.0),
                    2 => Vector2::new(0.0, -1.0),
                    3 => Vector2::new(-1.0, 0.0),
                    _ => unreachable!(),
                };
                let bounding_sphere = dummy.collider.get_bounding_sphere(collision_world);
                let player_bounding_sphere = player.collider.get_bounding_sphere(collision_world);
                let player_pos = player.collider.get_pos(collision_world);
                let dummy_pos = dummy.collider.get_pos(collision_world) + offset;
                let dx = dummy_pos - player_pos;
                let dn = dx.normalized();

                let ray_origin = player_pos + dn * 2.0 * player_bounding_sphere.radius;
                let ray = &rapier2d::geometry::Ray::new(
                    rapier2d::na::Vector2::new(ray_origin.x, ray_origin.y).into(),
                    rapier2d::na::Vector2::new(dn.x, dn.y),
                );
                let ray_length =
                    dx.length() - (bounding_sphere.radius + player_bounding_sphere.radius);
                fn predicate(
                    _handle: rapier2d::geometry::ColliderHandle,
                    collider: &rapier2d::geometry::Collider,
                ) -> bool {
                    collider.shape().as_cuboid().is_some()
                }
                let intersection = collision_world
                    .rapier
                    .query_pipeline
                    .cast_ray_and_get_normal(
                        &collision_world.rapier.rigid_body_set,
                        &collision_world.rapier.collider_set,
                        ray,
                        ray_length,
                        true,
                        rapier2d::pipeline::QueryFilter {
                            exclude_rigid_body: Some(dummy.collider.rigid_body_handle),
                            predicate: Some(&predicate),
                            ..Default::default()
                        },
                    );
                if let Some(_intersection) = intersection {
                } else if camera_world_rect.check_collision_circle_rec(
                    bounding_sphere.center().coords.to_raylib_vector2(),
                    bounding_sphere.radius,
                ) {
                    dummy.render(d, camera, collision_world, assets, thread, target);
                }
            }
        }
    }
}

pub fn spawn_debug_colldier_world(
    debug_colliders: &mut Vec<WorldColliderHandle>,
    collision_world: &mut CollisionWorld,
) {
    for _ in 0..10 {
        let size_x = rand::thread_rng().gen_range(1.0..6.4);
        let size_y = rand::thread_rng().gen_range(1.0..6.4);
        let pos_x = rand::thread_rng().gen_range(0.0..16.0 * 6.4);
        let pos_y = rand::thread_rng().gen_range(0.0..16.0 * 6.4);
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
                friction: 0.5,
                user_data: ColliderUserData::WALL,
                sensor: false,
            },
            ShapeArgs::Cuboid {
                half_extents: Vector2::new(size_x, size_y),
            },
        ));
    }
}
