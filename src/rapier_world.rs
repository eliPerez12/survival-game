#![allow(dead_code)]

use rapier2d::{
    crossbeam::{self, channel::Receiver},
    prelude::*,
};

pub const GRAVITY: f32 = 0.0;

pub struct RapierCollisionWorld {
    pub physics_pipeline: PhysicsPipeline,
    pub gravity: Vector<Real>,
    pub integration_parameters: IntegrationParameters,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    pub event_handler: ChannelEventCollector,
    pub collision_recv: Receiver<CollisionEvent>,
    pub contact_force_recv: Receiver<ContactForceEvent>,
    pub accumulated_time: f32,
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
            accumulated_time: 0.0,
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
}
