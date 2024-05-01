#![allow(dead_code)]

use rapier2d::prelude::*;
use raylib::prelude::*;

use crate::draw_collider::*;
use crate::traits::*;
use crate::CollisionWorld;
use crate::RapierCollisionWorld;

pub enum WorldColliderHandle {
    Cuboid {
        rigid_body_handle: RigidBodyHandle,
        collider_handle: ColliderHandle,
    },
    Ball {
        rigid_body_handle: RigidBodyHandle,
        collider_handle: ColliderHandle,
    },
    Triangle {
        rigid_body_handle: RigidBodyHandle,
        collider_handle: ColliderHandle,
    },
    Compound {
        rigid_body_handle: RigidBodyHandle,
        collider_handle: ColliderHandle,
    },
}

impl WorldColliderHandle {
    pub fn get_handles(&self) -> (RigidBodyHandle, ColliderHandle) {
        match *self {
            WorldColliderHandle::Cuboid {
                rigid_body_handle,
                collider_handle,
            } => (rigid_body_handle, collider_handle),
            WorldColliderHandle::Ball {
                rigid_body_handle,
                collider_handle,
            } => (rigid_body_handle, collider_handle),
            WorldColliderHandle::Triangle {
                rigid_body_handle,
                collider_handle,
            } => (rigid_body_handle, collider_handle),
            WorldColliderHandle::Compound {
                rigid_body_handle,
                collider_handle,
            } => (rigid_body_handle, collider_handle),
        }
    }

    pub fn add_vel(&mut self, vel: Vector2, collision_world: &mut CollisionWorld) {
        let rigid_body = &mut collision_world.rapier.rigid_body_set[self.get_handles().0];
        rigid_body.apply_impulse(rapier2d::na::Vector2::from_raylib_vector2(vel), true);
    }

    pub fn get_pos(&self, collision_world: &CollisionWorld) -> Vector2 {
        let rigid_body = &collision_world.rapier.rigid_body_set[self.get_handles().0];
        rigid_body.position().translation.vector.to_raylib_vector2()
    }

    pub fn get_vel(&self, collision_world: &RapierCollisionWorld) -> Vector2 {
        let rigid_body = &collision_world.rigid_body_set[self.get_handles().0];
        rigid_body.linvel().to_raylib_vector2()
    }

    pub fn get_mass(&self, collision_world: &RapierCollisionWorld) -> f32 {
        collision_world.collider_set[self.get_handles().1].mass()
    }

    pub fn get_angle(&self, collision_world: &CollisionWorld) -> f32 {
        collision_world.rapier.collider_set[self.get_handles().1]
            .rotation()
            .angle()
    }

    pub fn get_isometry_shape<'a>(&'a self, collision_world: &'a CollisionWorld) -> IsometryShape {
        let (rigid_body_handle, collider_handle) = self.get_handles();
        let isometry = collision_world.rapier.rigid_body_set[rigid_body_handle].position();
        let collider = &collision_world.rapier.collider_set[collider_handle];
        let shape = collider.shape();
        (*isometry, shape)
    }

    pub fn get_mut<'a>(
        &'a self,
        collision_world: &'a mut RapierCollisionWorld,
    ) -> (&mut RigidBody, &mut Collider) {
        let handles = self.get_handles();
        (
            &mut collision_world.rigid_body_set[handles.0],
            &mut collision_world.collider_set[handles.1],
        )
    }

    pub fn get<'a>(&'a self, collision_world: &'a RapierCollisionWorld) -> (&RigidBody, &Collider) {
        let handles = self.get_handles();
        (
            &collision_world.rigid_body_set[handles.0],
            &collision_world.collider_set[handles.1],
        )
    }
}

impl WorldColliderHandle {
    pub fn draw(
        &self,
        collision_world: &CollisionWorld,
        camera: Camera2D,
        d: &mut RaylibDrawHandle,
    ) {
        match self {
            WorldColliderHandle::Cuboid { .. } => draw_cuboid(
                self.get_isometry_shape(collision_world),
                Color::BLUE,
                d,
                &camera,
            ),
            WorldColliderHandle::Ball { .. } => draw_ball(
                self.get_isometry_shape(collision_world),
                Color::RED,
                d,
                &camera,
            ),
            WorldColliderHandle::Triangle { .. } => draw_triangle(
                self.get_isometry_shape(collision_world),
                Color::YELLOW,
                d,
                &camera,
            ),
            WorldColliderHandle::Compound { .. } => {
                draw_compound(self, Color::WHITE, d, &camera, &collision_world.rapier)
            }
        }
    }
}
