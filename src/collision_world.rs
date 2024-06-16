use crate::world_collider::WorldColliderHandle;
use rapier2d::prelude::*;
use raylib::prelude::*;

use crate::rapier_world::*;
use crate::traits::*;

#[derive(Default)]
pub struct CollisionWorld {
    pub rapier: RapierCollisionWorld, //TODO: Make private
}

pub struct ColliderUserData;

impl ColliderUserData {
    pub const BULLET: u128 = 2;
}


pub struct RigidBodyArgs {
    pub dynamic: bool,
    pub pos: Vector2,
    pub vel: Vector2,
    pub user_data: u128,
}

impl RigidBodyArgs {
    fn build_rigid_body(&self) -> RigidBody {
        match self.dynamic {
            false => RigidBodyBuilder::fixed(),
            true => RigidBodyBuilder::dynamic(),
        }
        .translation(rapier2d::na::Vector2::from_raylib_vector2(self.pos))
        .linvel(rapier2d::na::Vector2::from_raylib_vector2(self.vel))
        .user_data(self.user_data)
        .build()
    }
}

pub enum ShapeArgs {
    Cuboid { half_extents: Vector2 },
    Ball { radius: f32 },
    Triangle { points: (Vector2, Vector2, Vector2) },
}

type RelitiveShapeArgs = (Vector2, ShapeArgs);

pub struct ColliderArgs {
    pub density: f32,
    pub restitution: f32,
    pub friction: f32,
}

impl ColliderArgs {
    fn build_collider(&self, shape_args: &ShapeArgs) -> Collider {
        match shape_args {
            ShapeArgs::Cuboid { half_extents } => {
                ColliderBuilder::cuboid(half_extents.x, half_extents.y)
            }
            ShapeArgs::Ball { radius } => ColliderBuilder::ball(*radius),
            ShapeArgs::Triangle { points } => ColliderBuilder::triangle(
                rapier2d::na::Vector2::from_raylib_vector2(points.0).into(),
                rapier2d::na::Vector2::from_raylib_vector2(points.1).into(),
                rapier2d::na::Vector2::from_raylib_vector2(points.2).into(),
            ),
        }
        .restitution(self.restitution)
        .density(self.density)
        .friction(self.friction)
        .active_events(ActiveEvents::COLLISION_EVENTS)
        .build()
    }

    fn build_compound(&self, shape_args: Vec<RelitiveShapeArgs>) -> Collider {
        let shapes = {
            shape_args
                .iter()
                .map(|(rigid_body_args, shape_args)| {
                    let isometry = nalgebra::Isometry2::new(
                        rapier2d::na::Vector2::from_raylib_vector2(*rigid_body_args),
                        0.0,
                    );
                    let shape = match shape_args {
                        ShapeArgs::Cuboid { half_extents } => SharedShape::new(Cuboid::new(
                            nalgebra::Vector2::new(half_extents.x, half_extents.y),
                        )),
                        ShapeArgs::Ball { radius } => SharedShape::new(Ball::new(*radius)),
                        ShapeArgs::Triangle { points } => SharedShape::new(Triangle::new(
                            nalgebra::Vector2::new(points.0.x, points.0.y).into(),
                            nalgebra::Vector2::new(points.1.x, points.1.y).into(),
                            nalgebra::Vector2::new(points.2.x, points.2.y).into(),
                        )),
                    };
                    (isometry, shape)
                })
                .collect::<Vec<(Isometry<Real>, SharedShape)>>()
        };
        ColliderBuilder::compound(shapes)
            .restitution(self.restitution)
            .density(self.density)
            .friction(self.friction)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build()
    }
}

impl Default for ColliderArgs {
    fn default() -> Self {
        Self {
            density: 1.0,
            restitution: 0.7,
            friction: 0.5,
        }
    }
}

impl CollisionWorld {
    pub fn spawn_collider(
        &mut self,
        rigid_body_args: RigidBodyArgs,
        collider_args: ColliderArgs,
        shape_args: ShapeArgs,
    ) -> WorldColliderHandle {
        let rigid_body = rigid_body_args.build_rigid_body();
        let collider = collider_args.build_collider(&shape_args);
        let rigid_body_handle = self.rapier.rigid_body_set.insert(rigid_body);
        let collider_handle = self.rapier.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rapier.rigid_body_set,
        );
        WorldColliderHandle {
            rigid_body_handle,
            collider_handle,
        }
    }

    pub fn delete_collider(&mut self, collider: WorldColliderHandle) {
        self.rapier.rigid_body_set.remove(
            collider.rigid_body_handle,
            &mut self.rapier.island_manager,
            &mut self.rapier.collider_set,
            &mut self.rapier.impulse_joint_set,
            &mut self.rapier.multibody_joint_set,
            true,
        );
    }

    pub fn spawn_compound(
        &mut self,
        rigid_body_args: RigidBodyArgs,
        collider_args: ColliderArgs,
        shape_args: Vec<RelitiveShapeArgs>,
    ) -> WorldColliderHandle {
        let rigid_body = rigid_body_args.build_rigid_body();
        let collider = collider_args.build_compound(shape_args);
        let rigid_body_handle = self.rapier.rigid_body_set.insert(rigid_body);
        let collider_handle = self.rapier.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rapier.rigid_body_set,
        );
        WorldColliderHandle {
            rigid_body_handle,
            collider_handle,
        }
    }
}

impl CollisionWorld {
    const FIXED_TIME_STEP: f32 = 1.0 / 100.0;
    const MAX_FRAME_TIME: f32 = 0.25; // To prevent spiral of death in case of a long frame

    pub fn step(&mut self, rl: &RaylibHandle) {
        // Get the elapsed time for the current frame
        let frame_time = rl.get_frame_time().min(Self::MAX_FRAME_TIME);

        // Accumulate the elapsed time
        self.rapier.accumulated_time += frame_time;

        // Perform physics updates in fixed time steps
        while self.rapier.accumulated_time >= Self::FIXED_TIME_STEP {
            self.rapier.integration_parameters.dt = Self::FIXED_TIME_STEP;
            self.rapier.step();
            self.rapier.accumulated_time -= Self::FIXED_TIME_STEP;
        }
    }
}
