use std::collections::HashMap;

use raylib::prelude::*;

use crate::{assets::Assets, Player};

pub struct Inventrory {
    pub grids: HashMap<(u32, u32), bool>
}

impl Inventrory {
    pub fn render(&self, d: &mut RaylibDrawHandle, player: &Player, assets: &Assets) {
        if player.inventory_open {
            let screen_size =
                Vector2::new(d.get_screen_width() as f32, d.get_screen_height() as f32);
            let texture = assets.get_texture("inventory.png");
            let texture_size = Vector2::new(texture.width() as f32, texture.height() as f32);
            let scale = 7.0;
            let inventory_top_left = Vector2::new(screen_size.x / 2.0, screen_size.y / 2.0)
                - Vector2::new(texture_size.x / 2.0 * scale, texture_size.y / 2.0 * scale);
            d.draw_rectangle( // Make background darker
                0,
                0,
                screen_size.x as i32,
                screen_size.y as i32,
                Color::new(0, 0, 0, 100),
            );
            d.draw_texture_pro( // Drawing inventory slots
                texture,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: texture_size.x,
                    height: texture_size.y,
                },
                Rectangle {
                    x: inventory_top_left.x,
                    y: inventory_top_left.y,
                    width: texture_size.x * scale,
                    height: texture_size.y * scale,
                },
                Vector2::zero(),
                0.0,
                Color::new(255, 255, 255, 220),
            );
            let texture = assets.get_texture("417.png"); // Drawing items
            for (grid_pos, rotated) in &self.grids {
                let rotation_offset = if *rotated {
                    texture.width() as f32 / 2.0 * scale - 2.5
                } else {
                    0.0
                };
                let (x, y) = (inventory_top_left.x
                    + (grid_pos.0 as f32 * 17.0 * scale)
                    + (4.0 * scale)
                    + rotation_offset,
                    inventory_top_left.y
                    + (grid_pos.1 as f32 * 17.0 * scale)
                    + (4.0 * scale));
                d.draw_texture_pro(
                    texture,
                    Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: texture.width() as f32,
                        height: texture.height() as f32,
                    },
                    Rectangle {
                        x,
                        y,
                        width: texture.width() as f32 * scale,
                        height: texture.height() as f32 * scale,
                    },
                    Vector2::new(0.0, 0.0),
                    if *rotated { 90.0 } else { 0.0 },
                    Color::new(255, 255, 255, 255),
                );
            }
            
        }
    }
    
}
