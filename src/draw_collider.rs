use crate::traits::*;
use rapier2d::na::{Isometry, UnitComplex};
use rapier2d::prelude::*;
use raylib::prelude::*;

pub type IsometryShape<'a> = (Isometry<f32, UnitComplex<f32>, 2>, &'a dyn Shape);

pub fn draw_shape(
    isometry_shape: IsometryShape,
    color: Color,
    d: &mut RaylibDrawHandle,
    camera: &Camera2D,
) {
    let pos = isometry_shape.0.translation.vector.to_raylib_vector2();
    let angle = isometry_shape.0.rotation.angle().to_degrees();
    if let Some(collider) = isometry_shape.1.as_cuboid() {
        let half_extents = collider.half_extents.to_raylib_vector2();
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
    } else if let Some(collider) = isometry_shape.1.as_ball() {
        d.draw_circle_v(camera.to_screen(pos), collider.radius * camera.zoom, color);
    } else if let Some(collider) = isometry_shape.1.as_triangle() {
        let points = (
            (collider.a.coords.to_raylib_vector2().rotated(angle) + pos),
            (collider.c.coords.to_raylib_vector2().rotated(angle) + pos),
            (collider.b.coords.to_raylib_vector2().rotated(angle) + pos),
        );
        d.draw_triangle(
            camera.to_screen(points.0),
            camera.to_screen(points.1),
            camera.to_screen(points.2),
            color,
        );
    } else if let Some(collider) = isometry_shape.1.as_compound() {
        for (mut isometery, shape) in collider.shapes() {
            isometery.translation.vector += isometry_shape.0.translation.vector;
            draw_shape((isometery, &*shape.0), color, d, camera);
        }
    }
}
