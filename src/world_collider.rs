#![allow(dead_code)]

use rapier2d::{geometry::SharedShape, prelude::*};
use raylib::prelude::*;

use crate::draw_collider::*;
use crate::traits::*;
use crate::CollisionWorld;
use crate::RapierCollisionWorld;

pub enum WorldCollider {
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

impl WorldCollider {
    pub fn get_handles(&self) -> (RigidBodyHandle, ColliderHandle) {
        match *self {
            WorldCollider::Cuboid {
                rigid_body_handle,
                collider_handle,
            } => (rigid_body_handle, collider_handle),
            WorldCollider::Ball {
                rigid_body_handle,
                collider_handle,
            } => (rigid_body_handle, collider_handle),
            WorldCollider::Triangle {
                rigid_body_handle,
                collider_handle,
            } => (rigid_body_handle, collider_handle),
            WorldCollider::Compound {
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

impl WorldCollider {
    pub fn new_cuboid(
        pos: Vector2,
        vel: Vector2,
        half_extents: Vector2,
        fixed: bool,
        collision_world: &mut CollisionWorld,
    ) -> Self {
        let rapier = &mut collision_world.rapier;
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
        let rigid_body_handle = rapier.rigid_body_set.insert(rigid_body);
        let collider_handle = rapier.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut rapier.rigid_body_set,
        );
        WorldCollider::Cuboid {
            rigid_body_handle,
            collider_handle,
        }
    }
}

impl WorldCollider {
    pub fn new_ball(
        pos: Vector2,
        vel: Vector2,
        radius: f32,
        fixed: bool,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> Self {
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
        let rigid_body_handle = rigid_body_set.insert(rigid_body);
        let collider_handle =
            collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);
        WorldCollider::Ball {
            rigid_body_handle,
            collider_handle,
        }
    }
}

impl WorldCollider {
    pub fn new_triangle(
        pos: Vector2,
        vel: Vector2,
        points: (Vector2, Vector2, Vector2),
        fixed: bool,
        collision_world: &mut CollisionWorld,
    ) -> WorldCollider {
        let rapier = &mut collision_world.rapier;
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
        let rigid_body_handle = rapier.rigid_body_set.insert(rigid_body);
        let collider_handle = rapier.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut rapier.rigid_body_set,
        );
        WorldCollider::Triangle {
            rigid_body_handle,
            collider_handle,
        }
    }
}

type Shapes = Vec<(
    nalgebra::Isometry<f32, nalgebra::Unit<nalgebra::Complex<f32>>, 2>,
    SharedShape,
)>;

impl WorldCollider {
    pub fn new_compound(
        pos: Vector2,
        vel: Vector2,
        shapes: Shapes,
        fixed: bool,
        collision_world: &mut CollisionWorld,
    ) -> Self {
        let rapier = &mut collision_world.rapier;
        let rigid_body = match fixed {
            true => RigidBodyBuilder::fixed(),
            false => RigidBodyBuilder::dynamic(),
        }
        .translation(rapier2d::na::Vector2::from_raylib_vector2(pos))
        .linvel(rapier2d::na::Vector2::from_raylib_vector2(vel))
        .build();
        let collider = ColliderBuilder::compound(shapes);
        let rigid_body_handle = rapier.rigid_body_set.insert(rigid_body);
        let collider_handle = rapier.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut rapier.rigid_body_set,
        );
        WorldCollider::Compound {
            rigid_body_handle,
            collider_handle,
        }
    }
}

impl WorldCollider {
    pub fn draw(
        &self,
        collision_world: &CollisionWorld,
        camera: Camera2D,
        d: &mut RaylibDrawHandle,
    ) {
        match self {
            WorldCollider::Cuboid { .. } => draw_cuboid(
                self.get_isometry_shape(collision_world),
                Color::BLUE,
                d,
                &camera,
            ),
            WorldCollider::Ball { .. } => draw_ball(
                self.get_isometry_shape(collision_world),
                Color::RED,
                d,
                &camera,
            ),
            WorldCollider::Triangle { .. } => draw_triangle(
                self.get_isometry_shape(collision_world),
                Color::YELLOW,
                d,
                &camera,
            ),
            WorldCollider::Compound { .. } => {
                draw_compound(self, Color::WHITE, d, &camera, &collision_world.rapier)
            }
        }
    }
}
