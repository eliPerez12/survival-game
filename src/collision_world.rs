use crate::traits::*;
use rapier2d::{
    crossbeam::{self, channel::Receiver},
    geometry::SharedShape,
    prelude::*,
};
use raylib::prelude::*;

pub const GRAVITY: f32 = 9.81;

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

    pub fn get_pos(&self, collision_world: &mut RapierCollisionWorld) -> Vector2 {
        let rigid_body = &collision_world.rigid_body_set[self.get_handles().0];
        rigid_body.position().translation.vector.to_raylib_vector2()
    }

    pub fn get_vel(&self, collision_world: &mut RapierCollisionWorld) -> Vector2 {
        let rigid_body = &collision_world.rigid_body_set[self.get_handles().0];
        rigid_body.linvel().to_raylib_vector2()
    }

    pub fn get_mass(&self, collision_world: &mut RapierCollisionWorld) -> f32 {
        let collider = &collision_world.collider_set[self.get_handles().1];
        collider.mass()
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

impl WorldCollider {
    pub fn new_compound(
        pos: Vector2,
        vel: Vector2,
        shapes: Vec<(
            nalgebra::Isometry<f32, nalgebra::Unit<nalgebra::Complex<f32>>, 2>,
            SharedShape,
        )>,
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
            WorldCollider::Compound {
                rigid_body_handle,
                collider_handle,
            } => {}
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
        let shapes = collider.shape().as_compound().unwrap().shapes();
        for (isometry, shape) in shapes {
            let shape_type = shape.0.shape_type();
            match shape_type {
                ShapeType::Ball => (),
                ShapeType::Cuboid => (),
                _ => unimplemented!()
            }
        }
    }
}

// TODO: Make all fields private
pub struct RapierCollisionWorld {
    physics_pipeline: PhysicsPipeline,
    gravity: Vector<Real>,
    pub integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    event_handler: ChannelEventCollector,
    collision_recv: Receiver<CollisionEvent>,
    contact_force_recv: Receiver<ContactForceEvent>,
}

impl Default for RapierCollisionWorld {
    fn default() -> Self {
        // Initialize the event collector.
        let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();
        Self {
            gravity: vector![0.0, GRAVITY],
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            query_pipeline: QueryPipeline::new(),
            event_handler: { ChannelEventCollector::new(collision_send, contact_force_send) },
            collision_recv,
            contact_force_recv,
        }
    }
}

impl RapierCollisionWorld {
    pub fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &self.event_handler,
        );
    }

    pub fn get_collisions(&self) -> (Vec<CollisionEvent>, Vec<ContactForceEvent>) {
        let mut collisions = vec![];
        let mut contacts = vec![];
        while let Ok(collision_event) = self.collision_recv.try_recv() {
            // Handle the collision event.
            collisions.push(collision_event);
        }

        while let Ok(contact_force_event) = self.contact_force_recv.try_recv() {
            // Handle the collision event.
            contacts.push(contact_force_event);
        }

        (collisions, contacts)
    }

    pub fn get_collider(collider_handle: ColliderHandle) {}
}
