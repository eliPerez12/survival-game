use rapier2d::{
    crossbeam::{self, channel::Receiver},
    prelude::*,
};
use raylib::prelude::*;

// Adding additional methods to raylib camera2d
pub trait ImprovedCamera {
    fn to_screen(&self, world_pos: Vector2) -> Vector2;
    fn to_screen_x(&self, world_pos_x: f32) -> f32;
    fn to_screen_y(&self, world_pos_y: f32) -> f32;
    fn to_screen_rect(&self, rect: &Rectangle) -> Rectangle;
    fn to_world(&self, screen_pos: Vector2) -> Vector2;
    fn track(&mut self, pos: Vector2, screen_size: Vector2);
    fn get_world_pos(&self, offset: Vector2, screen_size: Vector2) -> Vector2;
    fn get_screen_offset(&self, world_pos: Vector2, screen_size: Vector2) -> Vector2;
}

impl ImprovedCamera for Camera2D {
    fn to_screen(&self, world_pos: Vector2) -> Vector2 {
        (world_pos + self.offset) * self.zoom
    }

    fn to_screen_x(&self, world_pos_x: f32) -> f32 {
        (world_pos_x + self.offset.x) * self.zoom
    }

    fn to_screen_y(&self, world_pos_y: f32) -> f32 {
        (world_pos_y + self.offset.y) * self.zoom
    }

    fn to_screen_rect(&self, rect: &Rectangle) -> Rectangle {
        Rectangle {
            x: (rect.x + self.offset.x) * self.zoom,
            y: (rect.y + self.offset.y) * self.zoom,
            width: rect.width * self.zoom,
            height: rect.height * self.zoom,
        }
    }

    fn to_world(&self, screen_pos: Vector2) -> Vector2 {
        (screen_pos / self.zoom) - self.offset
    }

    fn track(&mut self, target_world_pos: Vector2, screen_size: Vector2) {
        self.offset = self.get_screen_offset(target_world_pos, screen_size);
    }

    fn get_world_pos(&self, offset: Vector2, screen_size: Vector2) -> Vector2 {
        -offset + screen_size / (2.0 * self.zoom)
    }

    fn get_screen_offset(&self, world_pos: Vector2, screen_size: Vector2) -> Vector2 {
        -world_pos + screen_size / 2.0 / self.zoom
    }
}


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

enum Collider {
    Cuboid {
        rigid_body_handle: RigidBodyHandle,
        collider_handle: ColliderHandle,
    },
    Ball {
        rigid_body_handle: RigidBodyHandle,
        collider_handle: ColliderHandle,
    }
}

impl Collider {
    pub fn get_handles(&self) -> (RigidBodyHandle, ColliderHandle) {
        match *self {
            Collider::Cuboid { rigid_body_handle, collider_handle } => (rigid_body_handle, collider_handle),
            Collider::Ball { rigid_body_handle, collider_handle } => (rigid_body_handle, collider_handle),
        }
    }

    pub fn set_vel(&mut self, vel: Vector2, collision_world: &mut CollisionWorld) {
        let rigid_body = &mut collision_world.rigid_body_set[self.get_handles().0];
        rigid_body.set_linvel(rapier2d::na::Vector2::from_raylib_vector2(vel), true);
    }

    pub fn add_vel(&mut self, vel: Vector2, collision_world: &mut CollisionWorld) {
        let rigid_body = &mut collision_world.rigid_body_set[self.get_handles().0];
        rigid_body.apply_impulse(rapier2d::na::Vector2::from_raylib_vector2(vel), true);
    }

    pub fn get_pos(&self, collision_world: &mut CollisionWorld) -> Vector2 {
        let rigid_body = &collision_world.rigid_body_set[self.get_handles().0];
        rigid_body.position().translation.vector.to_raylib_vector2()
    }

    pub fn get_vel(&self, collision_world: &mut CollisionWorld) -> Vector2 {
        let rigid_body = &collision_world.rigid_body_set[self.get_handles().0];
        rigid_body.linvel().to_raylib_vector2()
    }
}


impl Collider {
    pub fn new_cuboid(
        pos: Vector2,
        vel: Vector2,
        half_extents: Vector2,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> Self {
        let rigid_body = RigidBodyBuilder::dynamic()
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
        Collider::Cuboid {
            rigid_body_handle,
            collider_handle,
        }
    }
}


impl Collider {
    pub fn new_ball(
        pos: Vector2,
        vel: Vector2,
        radius: f32,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> Self {
        let rigid_body = RigidBodyBuilder::dynamic()
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
        Collider::Ball {
            rigid_body_handle,
            collider_handle,
        }
    }
}

impl Collider {
    pub fn draw(&self, collision_world: &CollisionWorld, camera: Camera2D,d: &mut RaylibDrawHandle) {
        match self {
            Collider::Cuboid { rigid_body_handle, collider_handle } => {
                let rigid_body = &collision_world.rigid_body_set[*rigid_body_handle];
                let collider = collision_world.collider_set[*collider_handle]
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
                    Color::RED
                );
            },
            Collider::Ball { rigid_body_handle, collider_handle } => {
                let pos = collision_world.rigid_body_set[*rigid_body_handle]
                .translation()
                .to_raylib_vector2();
                let r = collision_world.collider_set[*collider_handle]
                .shape()
                .as_ball()
                .unwrap()
                .radius;
            d.draw_circle_v(camera.to_screen(pos), r * camera.zoom, Color::BLUE);
            },
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
        }

        while let Ok(contact_force_event) = self.contact_force_recv.try_recv() {
            // Handle the collision event.
            contacts.push(contact_force_event);
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
    let mut camera = Camera2D {
        zoom: 25.0,
        ..Default::default()
    };
    let mut collision_world = CollisionWorld::default();
    let ground_collider = ColliderBuilder::cuboid(1000.0, 0.1).build();
    let ground_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![0.0, 20.0])
        .build();
    let ground_body_handle = collision_world.rigid_body_set.insert(ground_rigid_body);
    collision_world.collider_set.insert_with_parent(
        ground_collider,
        ground_body_handle,
        &mut collision_world.rigid_body_set,
    );


    /* Create the bouncing ball. */
    let mut colliders = vec![];

    for x in 0..10 {
        for y in 0..10 {
            if rl.get_random_value::<i32>(0..2) == 1 {
                colliders.push(Collider::new_ball(
                    Vector2::new(x as f32, y as f32),
                    Vector2::zero(),
                    0.5,
                    &mut collision_world.rigid_body_set,
                    &mut collision_world.collider_set,
                ));
            } else {
                colliders.push(Collider::new_cuboid(
                    Vector2::new(x as f32, y as f32),
                    Vector2::zero(),
                    Vector2::new(0.5, 0.5),
                    &mut collision_world.rigid_body_set,
                    &mut collision_world.collider_set,
                ));
            }
            
        }
    }

    let mut player = Collider::new_cuboid(
        Vector2::new(20.0, 0.0),
        Vector2::zero(),
        Vector2::new(2.0, 2.0),
        &mut collision_world.rigid_body_set,
        &mut collision_world.collider_set
    );

    while !rl.window_should_close() {
        /*
         * Update
         */

        
        if rl.is_key_down(KeyboardKey::KEY_W) {
            camera.offset.y += 10.0 * rl.get_frame_time();
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            camera.offset.y -= 10.0 * rl.get_frame_time();
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            camera.offset.x += 10.0 * rl.get_frame_time();
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            camera.offset.x -= 10.0 * rl.get_frame_time();
        }
        collision_world.integration_parameters.dt = rl.get_frame_time();
        collision_world.step();
        let player_pos = player.get_pos(&mut collision_world);
        let mouse_pos = camera.to_world(rl.get_mouse_position());
        let d = mouse_pos - player_pos;
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            player.add_vel(dbg!(d.normalized() * 4.0), &mut collision_world);
        }
        let collisions = collision_world.get_collisions();

        /*
         * Drawing
         */
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        for collider in &colliders {
            collider.draw(&collision_world, camera, &mut d);
        }

        player.draw(&collision_world, camera, &mut d);
    }
}
