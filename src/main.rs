use rapier2d::{
    crossbeam::{self, channel::Receiver},
    prelude::*,
};
use raylib::prelude::*;

trait RaylibVector2 {
    fn to_raylib_vector2(&self) -> Vector2;
    fn from_raylib_vector2(vector: Vector2) -> Self;
}

impl RaylibVector2 for rapier2d::na::Vector2<f32> {
    fn to_raylib_vector2(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
    fn from_raylib_vector2(vector: Vector2) -> Self {
        Self::new(vector.x, vector.y)
    }
}

struct Cuboid {
    rigid_body_handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
}

impl Cuboid {
    pub fn new(
        pos: Vector2,
        half_extents: Vector2,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> Self {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(rapier2d::na::Vector2::from_raylib_vector2(pos))
            .build();
        let collider = ColliderBuilder::cuboid(half_extents.x, half_extents.y)
            .restitution(0.7)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let rigid_body_handle = rigid_body_set.insert(rigid_body);
        let collider_handle =
            collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);
        Self {
            rigid_body_handle,
            collider_handle,
        }
    }
}

struct Ball {
    rigid_body_handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
}

impl Ball {
    pub fn new(
        pos: Vector2,
        radius: f32,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> Self {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(rapier2d::na::Vector2::from_raylib_vector2(pos))
            .build();
        let collider = ColliderBuilder::ball(radius)
            .restitution(0.7)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let rigid_body_handle = rigid_body_set.insert(rigid_body);
        let collider_handle =
            collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);
        Self {
            rigid_body_handle,
            collider_handle,
        }
    }
}

pub struct CollisionWorld {
    physics_pipeline: PhysicsPipeline,
    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    event_handler: ChannelEventCollector,
    collision_recv: Receiver<CollisionEvent>,
    contact_force_recv: Receiver<ContactForceEvent>,
}

impl Default for CollisionWorld {
    fn default() -> Self {
        // Initialize the event collector.
        let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();
        Self {
            gravity: vector![0.0, 9.81],
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

impl CollisionWorld {
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
            println!("Received collision event: {:?}", collision_event);
        }

        while let Ok(contact_force_event) = self.contact_force_recv.try_recv() {
            // Handle the collision event.
            contacts.push(contact_force_event);
            println!("Received contact event: {:?}", contact_force_event);
        }

        (collisions, contacts)
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1080, 720)
        .title("Physics")
        .vsync()
        .build();

    let mut collision_world = CollisionWorld::default();
    /* Create the ground. */
    let ground_collider = ColliderBuilder::cuboid(1000.0, 0.1).build();
    let ground_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![0.0, 720.0])
        .build();
    let ground_body_handle = collision_world.rigid_body_set.insert(ground_rigid_body);
    collision_world.collider_set.insert_with_parent(
        ground_collider,
        ground_body_handle,
        &mut collision_world.rigid_body_set,
    );

    /* Create the bouncing ball. */
    let mut balls = vec![];
    let mut cuboids = vec![];

    for x in 0..10 {
        for y in 0..10 {
            if rl.get_random_value::<i32>(0..2) == 1 {
                balls.push(Ball::new(
                    Vector2::new(x as f32 * 21.0, y as f32 * 21.0),
                    10.0,
                    &mut collision_world.rigid_body_set,
                    &mut collision_world.collider_set,
                ));
            } else {
                cuboids.push(Cuboid::new(
                    Vector2::new(x as f32 * 21.0, y as f32 * 21.0),
                    Vector2::new(10.0, 10.0),
                    &mut collision_world.rigid_body_set,
                    &mut collision_world.collider_set,
                ));
            }
            
        }
    }
    /* Create other structures necessary for the simulation. */

    /* Run the game loop, stepping the simulation once per frame. */
    while !rl.window_should_close() {
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);

        collision_world.integration_parameters.dt = rl.get_frame_time();
        collision_world.step();
        let collisions = collision_world.get_collisions();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        for ball in &balls {
            let pos = collision_world.rigid_body_set[ball.rigid_body_handle]
                .translation()
                .to_raylib_vector2();
            let r = collision_world.collider_set[ball.collider_handle]
                .shape()
                .as_ball()
                .unwrap()
                .radius;
            d.draw_circle_v(pos, r, Color::BLUE);
        }

        for cuboid in &cuboids {
            let rigid_body = &collision_world.rigid_body_set[cuboid.rigid_body_handle];
            let collider = collision_world.collider_set[cuboid.collider_handle]
                .shape()
                .as_cuboid()
                .unwrap();
            let half_extents = collider.half_extents.to_raylib_vector2();
                
            let pos = rigid_body.translation().to_raylib_vector2();
            d.draw_rectangle_pro(
                Rectangle {
                    x: pos.x,
                    y: pos.y, 
                    width: half_extents.x * 2.0,
                    height: half_extents.y * 2.0,
                },
                half_extents,
                rigid_body.rotation().angle().to_degrees(),
                Color::RED
            );
        }

        for collision in &collisions.0 {
        }
    }
}
