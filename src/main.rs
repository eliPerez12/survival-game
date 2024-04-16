use ncollide2d::na::Vector2 as Vec2;
use ncollide2d::na::Isometry2;
use ncollide2d::query::RayCast;
use ncollide2d::shape::{Cuboid, ShapeHandle};
use raylib::prelude::*;

trait RaylibVector2 {
    fn to_raylib_vector2(&self) -> Vector2;
    fn from_raylib_vector2(vector: Vector2) -> Self;
}

impl RaylibVector2 for Vec2<f32> {
    fn to_raylib_vector2(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
    fn from_raylib_vector2(vector: Vector2) -> Self {
        Self::new(vector.x, vector.y)
    }
}

struct Collider {
    shape: ShapeHandle<f32>,
    isometry: Isometry2<f32>,
}

impl Collider {
    pub fn new(pos: Vector2, size: Vector2) -> Self {
        Collider {
            shape: ShapeHandle::new(Cuboid::new(Vec2::from_raylib_vector2(size/2.0))),
            isometry: Isometry2::new(Vec2::from_raylib_vector2(pos + size/2.0), -std::f32::consts::PI/2.0),
        }
    }
    
    pub fn intersects_ray(&self, ray: &Ray) -> Option<Vector2> {
        let intersection = self.shape.toi_and_normal_with_ray(
            &self.isometry,
            &ray.ncollide_ray,
            ray.length,
            true
        );
        intersection.map(|intersection|  ray.ncollide_ray.point_at(intersection.toi).coords.to_raylib_vector2())
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let top_left = self.get_top_left();
        let size = self.get_size();
        let rectangle = Rectangle {
            x: top_left.x,
            y: top_left.y,
            width: size.x,
            height: size.y
        };
        d.draw_rectangle_pro(rectangle, Vector2::zero(), self.get_rotation().to_degrees(), Color::BLUE);
    }

    pub fn get_top_left(&self) -> Vector2 {
        self.get_center() - self.shape.local_aabb().half_extents().to_raylib_vector2()
    }

    pub fn get_center(&self) -> Vector2 {
        self.isometry.translation.vector.to_raylib_vector2()
    }

    pub fn get_size(&self) -> Vector2 {
        self.shape.local_aabb().extents().to_raylib_vector2()
    }

    pub fn get_rotation(&self) -> f32 {
        self.isometry.rotation.re
    }

    pub fn set_pos(&mut self, pos: Vector2) {
        self.isometry.translation.vector = Vec2::from_raylib_vector2(pos)
    }

    // pub fn get_center(&self) -> Vector2 {
    //     self.isometry.translation.vector.to_raylib_vector2()
    // }
}

struct Ray {
    ncollide_ray: ncollide2d::query::Ray<f32>,
    length: f32,
}

impl Ray {
    pub fn new(origin: Vector2, direction: Vector2, length: f32) -> Self {
        Ray {
            ncollide_ray: ncollide2d::query::Ray::new(
                Vec2::from_raylib_vector2(origin).into(),
                Vec2::from_raylib_vector2(direction),
            ),
            length,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_line_ex(
            self.ncollide_ray.origin.coords.to_raylib_vector2(),
            self.ncollide_ray.origin.coords.to_raylib_vector2() + self.ncollide_ray.dir.to_raylib_vector2() * self.length,
            5.0,
            Color::RED,
        );
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Ray and Cuboid")
        .build();

    let mut cuboid_collider = Collider::new(
        Vector2::new(0.0, 0.0),
        Vector2::new(50.0, 100.0)
    );

    let ray = Ray::new(Vector2::new(50.0, 50.0), Vector2::new(1.0, 1.0), 1_000.0);
   
    while !rl.window_should_close() {

        cuboid_collider.set_pos(rl.get_mouse_position());

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        cuboid_collider.draw(&mut d);
        ray.draw(&mut d);
        if let Some(collision) = cuboid_collider.intersects_ray(&ray) {
            d.draw_circle_v(collision, 10.0, Color::GREEN)
        }

        d.draw_text(&format!("{:?}", d.get_mouse_position()),0,0,20,Color::WHITE);

    }
}
