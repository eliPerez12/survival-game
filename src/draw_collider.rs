use crate::{traits::*, RapierCollisionWorld, WorldColliderHandle};
use rapier2d::na::{Isometry, Isometry2, UnitComplex};
use rapier2d::prelude::*;
use raylib::prelude::*;

pub type IsometryShape<'a> = (Isometry<f32, UnitComplex<f32>, 2>, &'a dyn Shape);

pub fn draw_cuboid(
    isometry_shape: IsometryShape,
    color: Color,
    d: &mut RaylibDrawHandle,
    camera: &Camera2D,
) {
    let collider = isometry_shape.1.as_cuboid().unwrap();
    let half_extents = collider.half_extents.to_raylib_vector2();
    let pos = isometry_shape.0.translation.vector.to_raylib_vector2();
    let angle = isometry_shape.0.rotation.angle().to_degrees();
    d.draw_rectangle_pro(
        Rectangle {
            x: camera.to_screen_x(pos.x),
            y: camera.to_screen_y(pos.y),
            width: half_extents.x * 2.0 * camera.zoom,
            height: half_extents.y * 2.0 * camera.zoom,
        },
        half_extents * camera.zoom,
        angle,
        color,
    );
}

pub fn draw_ball(
    isometry_shape: IsometryShape,
    color: Color,
    d: &mut RaylibDrawHandle,
    camera: &Camera2D,
) {
    let pos = isometry_shape.0.translation.vector.to_raylib_vector2();
    let r = isometry_shape.1.as_ball().unwrap().radius;
    d.draw_circle_v(camera.to_screen(pos), r * camera.zoom, color);
}

pub fn draw_triangle(
    isometry_shape: IsometryShape,
    color: Color,
    d: &mut RaylibDrawHandle,
    camera: &Camera2D,
) {
    let pos = isometry_shape.0.translation.vector.to_raylib_vector2();
    let shape = isometry_shape.1.as_triangle().unwrap();
    let angle = isometry_shape.0.rotation.angle();
    let points = (
        (shape.a.coords.to_raylib_vector2().rotated(angle) + pos),
        (shape.c.coords.to_raylib_vector2().rotated(angle) + pos),
        (shape.b.coords.to_raylib_vector2().rotated(angle) + pos),
    );

    d.draw_triangle(
        camera.to_screen(points.0),
        camera.to_screen(points.1),
        camera.to_screen(points.2),
        color,
    );
}

pub fn isometry_shape(pos: Vector2, angle: f32, shape: &dyn Shape) -> IsometryShape {
    (
        Isometry2::new(nalgebra::Vector2::from_raylib_vector2(pos), angle),
        shape,
    )
}

pub fn draw_compound(
    collider: &WorldColliderHandle,
    color: Color,
    d: &mut RaylibDrawHandle,
    camera: &Camera2D,
    collision_world: &RapierCollisionWorld,
) {
    let handles = collider.get_handles();
    let (rigid_body, collider) = (
        &collision_world.rigid_body_set[handles.0],
        &collision_world.collider_set[handles.1],
    );
    let rigid_body_angle = rigid_body.rotation().angle();
    let compound_pos = rigid_body.position();
    let shapes = collider.shape().as_compound().unwrap().shapes();
    for (isometry, shape) in shapes {
        let relative_pos = isometry.translation.vector.to_raylib_vector2();
        let angle = isometry.rotation.angle();
        let final_pos = compound_pos.translation.vector.to_raylib_vector2()
            + relative_pos.rotated(angle + compound_pos.rotation.angle());

        let shape_type = shape.0.shape_type();
        match shape_type {
            ShapeType::Ball => {
                draw_ball(
                    isometry_shape(final_pos, rigid_body_angle, &*shape.0),
                    color,
                    d,
                    camera,
                );
            }
            ShapeType::Cuboid => {
                draw_cuboid(
                    isometry_shape(final_pos, rigid_body_angle, &*shape.0),
                    color,
                    d,
                    camera,
                );
            }
            ShapeType::Triangle => draw_triangle(
                isometry_shape(final_pos, rigid_body_angle, &*shape.0),
                color,
                d,
                camera,
            ),
            _ => unimplemented!(),
        }
    }
}
