use std::collections::HashMap;

use rapier2d::na::SimdComplexField;
use raylib::prelude::*;

use crate::{assets::Assets, Player};

#[derive(Clone)]
pub struct InventoryItem {
    pub rotated: bool,
    pub size: (u32, u32),
}

pub struct Inventory {
    pub items: HashMap<(i32, i32), InventoryItem>,
    pub selected_item: Option<((i32, i32), InventoryItem, Vector2)>,
}

impl InventoryItem {
    fn get_rotation_offset(&self, scale: f32 )-> f32 {
        (self.size.0 as f32 * Inventory::GRID_SPACING - 1.0) / 2.0 * scale - 2.5
    }

    fn rotate_back(&self, rect: &Rectangle, scale: f32) -> Rectangle {
        Rectangle {
            x: rect.x - self.get_rotation_offset(scale),
            y: rect.y,
            width: rect.height,
            height: rect.width,
        }
    }
}


impl Inventory {
    pub const GRID_SPACING: f32 = 17.0;
    pub fn get_item_rect(&self, grid_pos: &(i32, i32), scale: f32, inventory_top_left: Vector2) -> Rectangle {
        let item = &self.items[grid_pos];
        let rotation_offset = if item.rotated {
            item.get_rotation_offset(scale)
        } else {
            0.0
        };
        let (x, y) = (inventory_top_left.x
            + (grid_pos.0 as f32 * Self::GRID_SPACING * scale)
            + (4.0 * scale)
            + rotation_offset,
            inventory_top_left.y
            + (grid_pos.1 as f32 * Self::GRID_SPACING * scale)
            + (4.0 * scale));
        Rectangle {
            x,
            y,
            width: (item.size.0 as f32 * Self::GRID_SPACING - 1.0) * scale + 0.1,
            height: (item.size.1 as f32 * Self::GRID_SPACING - 1.0) * scale,
        }
    }

    pub fn get_hovered_rect(&self, scale: f32, inventory_top_left: Vector2, mouse_pos: Vector2) -> Option<(&(i32, i32), &InventoryItem)> {
        for (grid_pos, item) in &self.items  {
            let mut rect = self.get_item_rect(grid_pos, scale, inventory_top_left);
            if item.rotated {
                rect = item.rotate_back(&rect, scale);
            }
            if rect.check_collision_point_rec(mouse_pos) {
                return Some((grid_pos, item))
            }
        }
        None
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle, player: &Player, assets: &Assets) {
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
                Color::new(255, 255, 255, 235),
            );
            let texture = assets.get_texture("417.png"); // Drawing items
            for (grid_pos, item) in &self.items {
                let dest_rect = self.get_item_rect(grid_pos, scale, inventory_top_left);
                d.draw_texture_pro(
                    texture,
                    Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: texture.width() as f32,
                        height: texture.height() as f32,
                    },
                    dest_rect,
                    Vector2::new(0.0, 0.0),
                    if item.rotated { 90.0 } else { 0.0 },
                    Color::new(255, 255, 255, 255),
                );
            }
                let mut new_mouse_offset = Vector2::zero();
                if d.is_key_pressed(KeyboardKey::KEY_R) {
                    if let Some(item) = &mut self.selected_item{
                        item.1.rotated = !item.1.rotated
                    }
                }
                if self.selected_item.is_none() {
                    // Player selects item
                    if let Some((grid_pos, item)) = self.get_hovered_rect(scale, inventory_top_left, d.get_mouse_position()) {
                        if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                            let mut rect = self.get_item_rect(grid_pos, scale, inventory_top_left);
                            if item.rotated {
                                rect = item.rotate_back(&rect, scale);
                            };
                            d.draw_rectangle_rec(rect, Color::WHITE);
                            self.selected_item = Some((*grid_pos, item.clone(), Vector2::zero()));
                        }
                    }
                    // Player stoped selecting item
                } else if !d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                    if let Some((grid_pos, _item, offset)) = &self.selected_item {
                        let new_grid = ((grid_pos.0 as f32 + (offset.x/ 17.0 / scale)).round() as i32, (grid_pos.1 as f32 + (offset.y/17.0 / scale)).round() as i32);
                        let item = self.items.remove(grid_pos).unwrap();
                        self.items.insert(new_grid, item);
                        self.selected_item = None;
                    }
                    // Player still selecting item
                } else if let Some((grid_pos, item, mouse_offset)) = &self.selected_item {

                    new_mouse_offset += *mouse_offset + d.get_mouse_delta();
                    let rect = self.get_item_rect(grid_pos, scale, inventory_top_left);
                    let mut new_rect = rect;
                    if item.rotated {
                        new_rect = item.rotate_back(&rect, scale);
                    }; 
                    d.draw_rectangle_rec(new_rect, Color::new(255,255,255,140));
                    let offset_rect = Rectangle {
                        x: rect.x + new_mouse_offset.x,
                        y: rect.y + new_mouse_offset.y,
                        width: rect.width,
                        height: rect.height,
                    };
                    d.draw_texture_pro(
                        texture,
                        Rectangle {
                            x: 0.0,
                            y: 0.0,
                            width: texture.width() as f32,
                            height: texture.height() as f32,
                        },
                        offset_rect,
                        Vector2::new(0.0, 0.0),
                        if item.rotated { 90.0 } else { 0.0 },
                        Color::new(255, 255, 255, 255),
                    );
                    // d.draw_rectangle_rec(offset_rect, Color::BLUE);
                }
                if let Some(item) = &mut self.selected_item{
                    item.2 = new_mouse_offset;
                }

            }

    }
    
}
