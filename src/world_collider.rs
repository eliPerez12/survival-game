#![allow(dead_code)]

use rapier2d::{
    geometry::SharedShape,
    prelude::*,
};
use raylib::prelude::*;

use crate::traits::*;
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
            WorldCollider::Compound {
                rigid_body_handle,
                collider_handle,
            } => (rigid_body_handle, collider_handle),
        }
    }

    pub fn add_vel(&mut self, vel: Vector2, collision_world: &mut RapierCollisionWorld) {
        let rigid_body = &mut collision_world.rigid_body_set[self.get_handles().0];
        rigid_body.apply_impulse(rapier2d::na::Vector2::from_raylib_vector2(vel), true);
    }

    pub fn get_pos(&self, collision_world: &RapierCollisionWorld) -> Vector2 {
        let rigid_body = &collision_world.rigid_body_set[self.get_handles().0];
        rigid_body.position().translation.vector.to_raylib_vector2()
    }

    pub fn get_vel(&self, collision_world: &RapierCollisionWorld) -> Vector2 {
        let rigid_body = &collision_world.rigid_body_set[self.get_handles().0];
        rigid_body.linvel().to_raylib_vector2()
    }

    pub fn get_mass(&self, collision_world: &RapierCollisionWorld) -> f32 {
        collision_world.collider_set[self.get_handles().1].mass()
    }

    pub fn get_angle(&self, collision_world: &RapierCollisionWorld) -> f32 {
        collision_world.collider_set[self.get_handles().1].rotation().angle()
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
        let collider = ColliderBuilder::cuboid(half_extents.x, half_extents.y)
            .restitution(0.7)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let rigid_body_handle = rigid_body_set.insert(rigid_body);
        let collider_handle =
            collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);
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
        let collider = ColliderBuilder::compound(shapes);
        let rigid_body_handle = rigid_body_set.insert(rigid_body);
        let collider_handle =
            collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);
        WorldCollider::Compound {
            rigid_body_handle,
            collider_handle,
        }
    }
}

impl WorldCollider {
    pub fn draw(
        &self,
        collision_world: &RapierCollisionWorld,
        camera: Camera2D,
        d: &mut RaylibDrawHandle,
    ) {
        match self {
            WorldCollider::Cuboid { .. } => self.draw_cuboid(d, &camera, collision_world),
            WorldCollider::Ball { .. } => self.draw_ball(d, &camera, collision_world),
            WorldCollider::Compound {..} => {self.draw_compound(d, &camera, collision_world)}
        }
    }

    fn draw_cuboid(
        &self,
        d: &mut RaylibDrawHandle,
        camera: &Camera2D,
        collision_world: &RapierCollisionWorld,
    ) {
        let handles = self.get_handles();
        let rigid_body = &collision_world.rigid_body_set[handles.0];
        let collider = collision_world.collider_set[handles.1]
            .shape()
            .as_cuboid()
            .unwrap();
        let half_extents = collider.half_extents.to_raylib_vector2();
        let pos = rigid_body.translation().to_raylib_vector2();
        d.draw_rectangle_pro(
            Rectangle {
                x: camera.to_screen_x(pos.x),
                y: camera.to_screen_y(pos.y),
                width: half_extents.x * 2.0 * camera.zoom,
                height: half_extents.y * 2.0 * camera.zoom,
            },
            half_extents * camera.zoom,
            rigid_body.rotation().angle().to_degrees(),
            Color::RED,
        );
    }

    fn draw_ball(
        &self,
        d: &mut RaylibDrawHandle,
        camera: &Camera2D,
        collision_world: &RapierCollisionWorld,
    ) {
        let handles = self.get_handles();
        let pos = collision_world.rigid_body_set[handles.0]
            .translation()
            .to_raylib_vector2();
        let r = collision_world.collider_set[handles.1]
            .shape()
            .as_ball()
            .unwrap()
            .radius;
        d.draw_circle_v(camera.to_screen(pos), r * camera.zoom, Color::BLUE);
    }

    fn draw_compound(
        &self,
        d: &mut RaylibDrawHandle,
        camera: &Camera2D,
        collision_world: &RapierCollisionWorld,
    ) {
        let handles = self.get_handles();
        let (rigid_body, collider) = (
            &collision_world.rigid_body_set[handles.0],
            &collision_world.collider_set[handles.1],
        );
        let compound_pos = rigid_body.position();
        let shapes = collider.shape().as_compound().unwrap().shapes();
        for (isometry, shape) in shapes {
            let relative_pos = isometry.translation.vector.to_raylib_vector2();
            let angle = isometry.rotation.angle();
            let final_pos = compound_pos.translation.vector.to_raylib_vector2() + relative_pos.rotated(angle + compound_pos.rotation.angle());

            let shape_type = shape.0.shape_type();
            match shape_type {
                ShapeType::Ball => {
                    let r = shape.as_ball().unwrap().radius;
                    d.draw_circle_v(
                        camera.to_screen(final_pos),
                        r * camera.zoom,
                        Color::BLUE,
                    );
                }
                ShapeType::Cuboid => {
                    let half_extents = shape.as_cuboid().unwrap().half_extents.to_raylib_vector2();
                    d.draw_rectangle_pro(
                    Rectangle {
                        x: camera.to_screen_x(final_pos.x),
                        y: camera.to_screen_y(final_pos.y),
                        width: half_extents.x * 2.0 * camera.zoom,
                        height: half_extents.y * 2.0 * camera.zoom,
                    },
                    half_extents * camera.zoom,
                    rigid_body.rotation().angle().to_degrees(),
                    Color::RED,
                );},
                _ => unimplemented!(),
            }
        }
    }
}