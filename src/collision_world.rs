use crate::world_collider::WorldColliderHandle;
use rapier2d::prelude::*;
use raylib::prelude::*;

use crate::rapier_world::*;
use crate::traits::*;
#[derive(Default)]
pub struct CollisionWorld {
    pub rapier: RapierCollisionWorld, //TODO: Make private
    colliders: Vec<WorldColliderHandle>,
}

pub type Shapes = Vec<(
    nalgebra::Isometry<f32, nalgebra::Unit<nalgebra::Complex<f32>>, 2>,
    SharedShape,
)>;

impl CollisionWorld {
    pub fn new_cuboid(
        &mut self,
        pos: Vector2,
        vel: Vector2,
        half_extents: Vector2,
        fixed: bool,
    ) -> WorldColliderHandle {
        let rigid_body = match fixed {
            true => RigidBodyBuilder::fixed(),
            false => RigidBodyBuilder::dynamic(),
        }
        .translation(rapier2d::na::Vector2::from_raylib_vector2(pos))
        .linvel(rapier2d::na::Vector2::from_raylib_vector2(vel))
        .build();
        let collider = ColliderBuilder::cuboid(half_extents.x, half_extents.y)
            .restitution(0.7)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let rigid_body_handle = self.rapier.rigid_body_set.insert(rigid_body);
        let collider_handle = self.rapier.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rapier.rigid_body_set,
        );
        WorldColliderHandle::Cuboid {
            rigid_body_handle,
            collider_handle,
        }
    }

    pub fn new_ball(
        &mut self,
        pos: Vector2,
        vel: Vector2,
        radius: f32,
        fixed: bool,
    ) -> WorldColliderHandle {
        let rigid_body = match fixed {
            true => RigidBodyBuilder::fixed(),
            false => RigidBodyBuilder::dynamic(),
        }
        .translation(rapier2d::na::Vector2::from_raylib_vector2(pos))
        .linvel(rapier2d::na::Vector2::from_raylib_vector2(vel))
        .build();
        let collider = ColliderBuilder::ball(radius)
            .restitution(0.7)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let rigid_body_handle = self.rapier.rigid_body_set.insert(rigid_body);
        let collider_handle = self.rapier.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rapier.rigid_body_set,
        );
        WorldColliderHandle::Ball {
            rigid_body_handle,
            collider_handle,
        }
    }

    pub fn new_triangle(
        &mut self,
        pos: Vector2,
        vel: Vector2,
        points: (Vector2, Vector2, Vector2),
        fixed: bool,
    ) -> WorldColliderHandle {
        let rigid_body = match fixed {
            true => RigidBodyBuilder::fixed(),
            false => RigidBodyBuilder::dynamic(),
        }
        .translation(rapier2d::na::Vector2::from_raylib_vector2(pos))
        .linvel(rapier2d::na::Vector2::from_raylib_vector2(vel))
        .build();
        let collider = ColliderBuilder::triangle(
            rapier2d::na::Vector2::from_raylib_vector2(points.0).into(),
            rapier2d::na::Vector2::from_raylib_vector2(points.1).into(),
            rapier2d::na::Vector2::from_raylib_vector2(points.2).into(),
        )
        .restitution(0.7)
        .active_events(ActiveEvents::COLLISION_EVENTS)
        .build();
        let rigid_body_handle = self.rapier.rigid_body_set.insert(rigid_body);
        let collider_handle = self.rapier.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rapier.rigid_body_set,
        );
        WorldColliderHandle::Triangle {
            rigid_body_handle,
            collider_handle,
        }
    }

    pub fn new_compound(
        &mut self,
        pos: Vector2,
        vel: Vector2,
        shapes: Shapes,
        fixed: bool,
    ) -> WorldColliderHandle {
        let rigid_body = match fixed {
            true => RigidBodyBuilder::fixed(),
            false => RigidBodyBuilder::dynamic(),
        }
        .translation(rapier2d::na::Vector2::from_raylib_vector2(pos))
        .linvel(rapier2d::na::Vector2::from_raylib_vector2(vel))
        .build();
        let collider = ColliderBuilder::compound(shapes);
        let rigid_body_handle = self.rapier.rigid_body_set.insert(rigid_body);
        let collider_handle = self.rapier.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rapier.rigid_body_set,
        );
        WorldColliderHandle::Compound {
            rigid_body_handle,
            collider_handle,
        }
    }
}

impl CollisionWorld {
    const MIN_PHYSICS_ACCURACY: f32 = 1.0 / 60.0;

    pub fn step(&mut self, rl: &RaylibHandle) {
        self.rapier.integration_parameters.dt = rl.get_frame_time().min(Self::MIN_PHYSICS_ACCURACY);
        self.rapier.step();
    }
}
