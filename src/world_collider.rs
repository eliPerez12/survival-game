#![allow(dead_code)]

use rapier2d::parry::bounding_volume::BoundingSphere;
use rapier2d::prelude::*;
use raylib::prelude::*;

use crate::draw_collider::*;
use crate::traits::*;
use crate::CollisionWorld;
use crate::RapierCollisionWorld;
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct WorldColliderHandle {
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

impl WorldColliderHandle {
    // Add linear velocity to collider
    pub fn add_linvel(&self, vel: Vector2, collision_world: &mut CollisionWorld) {
        let rigid_body = &mut collision_world.rapier.rigid_body_set[self.rigid_body_handle];
        rigid_body.apply_impulse(rapier2d::na::Vector2::from_raylib_vector2(vel), true)
    }

    // Add angular velocity to collider
    pub fn add_angvel(&self, torque: f32, collision_world: &mut CollisionWorld) {
        let rigid_body = &mut collision_world.rapier.rigid_body_set[self.rigid_body_handle];
        rigid_body.apply_torque_impulse(torque, false);
    }

    // Set angular velocity to collider
    pub fn set_angvel(&self, angvel: f32, collision_world: &mut CollisionWorld) {
        let rigid_body = &mut collision_world.rapier.rigid_body_set[self.rigid_body_handle];
        rigid_body.set_angvel(angvel, true);
    }

    pub fn set_linvel(&self, linvel: Vector2, collision_world: &mut CollisionWorld) {
        let rigid_body = &mut collision_world.rapier.rigid_body_set[self.rigid_body_handle];
        rigid_body.set_linvel(nalgebra::Vector2::from_raylib_vector2(linvel), true);
    }

    pub fn get_linvel(&self, collision_world: &CollisionWorld) -> Vector2 {
        let rigid_body = &collision_world.rapier.rigid_body_set[self.rigid_body_handle];
        rigid_body.linvel().to_raylib_vector2()
    }

    pub fn get_angvel(&self, collision_world: &CollisionWorld) -> f32 {
        let rigid_body = &collision_world.rapier.rigid_body_set[self.rigid_body_handle];
        rigid_body.angvel()
    }

    pub fn get_pos(&self, collision_world: &CollisionWorld) -> Vector2 {
        let rigid_body = &collision_world.rapier.rigid_body_set[self.rigid_body_handle];
        rigid_body.position().translation.vector.to_raylib_vector2()
    }

    pub fn get_vel(&self, collision_world: &CollisionWorld) -> Vector2 {
        let rigid_body = &collision_world.rapier.rigid_body_set[self.rigid_body_handle];
        rigid_body.linvel().to_raylib_vector2()
    }

    pub fn get_center_of_mass(&self, collision_world: &CollisionWorld) -> Vector2 {
        let rigid_body = &collision_world.rapier.rigid_body_set[self.rigid_body_handle];
        rigid_body.center_of_mass().coords.to_raylib_vector2()
    }

    pub fn get_mass(&self, collision_world: &CollisionWorld) -> f32 {
        collision_world.rapier.collider_set[self.collider_handle].mass()
    }

    pub fn get_bounding_sphere(&self, collision_world: &CollisionWorld) -> BoundingSphere {
        let pos = self.get_pos(collision_world);
        collision_world.rapier.collider_set[self.collider_handle]
            .shape()
            .compute_bounding_sphere(&nalgebra::Vector2::from_raylib_vector2(pos).into())
    }

    pub fn get_angle(&self, collision_world: &CollisionWorld) -> f32 {
        collision_world.rapier.collider_set[self.collider_handle]
            .rotation()
            .angle()
    }

    pub fn get_isometry_shape<'a>(&'a self, collision_world: &'a CollisionWorld) -> IsometryShape {
        let isometry = collision_world.rapier.rigid_body_set[self.rigid_body_handle].position();
        let collider = &collision_world.rapier.collider_set[self.collider_handle];
        let shape = collider.shape();
        (*isometry, shape)
    }

    pub fn get_mut<'a>(
        &'a self,
        collision_world: &'a mut RapierCollisionWorld,
    ) -> (&mut RigidBody, &mut Collider) {
        (
            &mut collision_world.rigid_body_set[self.rigid_body_handle],
            &mut collision_world.collider_set[self.collider_handle],
        )
    }

    pub fn get<'a>(&'a self, collision_world: &'a RapierCollisionWorld) -> (&RigidBody, &Collider) {
        (
            &collision_world.rigid_body_set[self.rigid_body_handle],
            &collision_world.collider_set[self.collider_handle],
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
        let isometry_shape = self.get_isometry_shape(collision_world);
        draw_shape(isometry_shape, Color::WHITE, d, &camera);
    }
}
