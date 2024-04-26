// Adding additional methods to raylib camera2d
use raylib::prelude::*;

pub trait ImprovedCamera {
    fn to_screen(&self, world_pos: Vector2) -> Vector2;
    fn to_screen_x(&self, world_pos_x: f32) -> f32;
    fn to_screen_y(&self, world_pos_y: f32) -> f32;
    fn to_screen_rect(&self, rect: &Rectangle) -> Rectangle;
    fn to_world(&self, screen_pos: Vector2) -> Vector2;
    fn track(&mut self, pos: Vector2, screen_size: Vector2);
    fn get_world_pos(&self, offset: Vector2, screen_size: Vector2) -> Vector2;
    fn get_screen_offset(&self, world_pos: Vector2, screen_size: Vector2) -> Vector2;

    fn handle_camera_controls(&mut self, rl: &RaylibHandle);
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

    fn handle_camera_controls(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.offset.y += 10.0 * rl.get_frame_time();
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.offset.y -= 10.0 * rl.get_frame_time();
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.offset.x += 10.0 * rl.get_frame_time();
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.offset.x -= 10.0 * rl.get_frame_time();
        }

        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        let mouse_wheel_move = rl.get_mouse_wheel_move();

        if mouse_wheel_move != 0.0 {
            let old_world_pos = self.get_world_pos(self.offset, screen_size);
            self.zoom *= 1.0 + rl.get_mouse_wheel_move() / 100.0;
            self.track(old_world_pos, screen_size);
        }
    }
}

pub trait RaylibVector2 {
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
