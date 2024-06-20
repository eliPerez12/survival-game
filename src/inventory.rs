use raylib::prelude::*;
use std::collections::HashMap;

use crate::{assets::Assets, Player};

#[derive(Clone, Debug)]
pub struct InventoryItem {
    pub rotated: bool,
    pub size: (u32, u32),
    pub item: Item,
}

#[derive(Clone, Debug)]
pub enum Item {
    Rifle,
    Pistol,
    MedKit,
}
impl Item {
    pub fn as_inventory_item(self, rotated: bool) -> InventoryItem {
        InventoryItem {
            rotated,
            size: self.get_inventory_size(),
            item: self
        }
    }

    pub fn get_inventory_size(&self) -> (u32, u32) {
        match self {
            Item::Rifle => (4,2),
            Item::Pistol => (2,1),
            Item::MedKit => (2,2)
        }
    }

    pub fn get_asset_name(&self) -> String {
        match self {
            Item::Rifle => "417.png".to_string(),
            Item::Pistol => "pistol.png".to_string(),
            Item::MedKit => "medkit.png".to_string(),
        }
    }
}
pub struct Inventory {
    pub items: HashMap<(i32, i32), InventoryItem>,
    pub selected_item: Option<((i32, i32), InventoryItem, Vector2)>,
}

impl InventoryItem {
    fn get_rotation_offset(&self, scale: f32) -> f32 {
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
    pub const SIZE: (i32, i32) = (8, 4);

    pub fn get_item_rect(
        &self,
        grid_pos: &(i32, i32),
        scale: f32,
        inventory_top_left: Vector2,
    ) -> Rectangle {
        let item = &self.items[grid_pos];
        let rotation_offset = if item.rotated {
            item.get_rotation_offset(scale)
        } else {
            0.0
        };
        let (x, y) = (
            inventory_top_left.x + (grid_pos.0 as f32 * Self::GRID_SPACING * scale) + (4.0 * scale) + rotation_offset,
            inventory_top_left.y + (grid_pos.1 as f32 * Self::GRID_SPACING * scale) + (4.0 * scale),
        );
        Rectangle {
            x,
            y,
            width: (item.size.0 as f32 * Self::GRID_SPACING - 1.0) * scale + 0.1,
            height: (item.size.1 as f32 * Self::GRID_SPACING - 1.0) * scale,
        }
    }

    pub fn get_hovered_rect(
        &self,
        scale: f32,
        inventory_top_left: Vector2,
        mouse_pos: Vector2,
    ) -> Option<(&(i32, i32), &InventoryItem)> {
        for (grid_pos, item) in &self.items {
            let mut rect = self.get_item_rect(grid_pos, scale, inventory_top_left);
            if item.rotated {
                rect = item.rotate_back(&rect, scale);
            }
            if rect.check_collision_point_rec(mouse_pos) {
                return Some((grid_pos, item));
            }
        }
        None
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle, player: &Player, assets: &Assets) {
        if player.inventory_open {
            let screen_size = Vector2::new(d.get_screen_width() as f32, d.get_screen_height() as f32);
            let texture = assets.get_texture("inventory.png");
            let texture_size = Vector2::new(texture.width() as f32, texture.height() as f32);
            let scale = 7.0;
            let inventory_top_left = Vector2::new(screen_size.x / 2.0, screen_size.y / 2.0)
                - Vector2::new(texture_size.x / 2.0 * scale, texture_size.y / 2.0 * scale);
            
            self.draw_background(d, screen_size);
            self.draw_inventory_slots(d, texture, texture_size, scale, inventory_top_left);

            self.draw_items(d, assets, scale, inventory_top_left);
            self.draw_selected_item(d, scale, inventory_top_left);
            
            let offset = self.handle_item_selection(d, scale, inventory_top_left, assets);
            if let Some(item) = &mut self.selected_item {
                item.2 = offset;
            }
        }
    }

    fn draw_background(&self, d: &mut RaylibDrawHandle, screen_size: Vector2) {
        d.draw_rectangle(0, 0, screen_size.x as i32, screen_size.y as i32, Color::new(0, 0, 0, 100));
    }

    fn draw_inventory_slots(
        &self,
        d: &mut RaylibDrawHandle,
        texture: &Texture2D,
        texture_size: Vector2,
        scale: f32,
        inventory_top_left: Vector2,
    ) {
        d.draw_texture_pro(
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
    }

    fn draw_items(
        &self,
        d: &mut RaylibDrawHandle,
        assets: &Assets,
        scale: f32,
        inventory_top_left: Vector2,
    ) {
        
        for (grid_pos, item) in &self.items {
            let texture = assets.get_texture(&item.item.get_asset_name());
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
    }

    fn draw_selected_item(
        &self,
        d: &mut RaylibDrawHandle,
        scale: f32,
        inventory_top_left: Vector2,
    ) {
        if let Some(item) = &self.selected_item {
            let mut dest_rect = self.get_item_rect(&item.0, scale, inventory_top_left);
            let item = self.items.get(&item.0).unwrap();
            let rotation = if item.rotated { 90.0 } else { 0.0 };
            dest_rect.width += 0.01; // correcting for rounding
            d.draw_rectangle_pro(dest_rect, Vector2::zero(), rotation, Color::new(255, 255, 255, 140));
        }
    }

    fn handle_item_selection(
        &mut self,
        d: &mut RaylibDrawHandle,
        scale: f32,
        inventory_top_left: Vector2,
        assets: &Assets
    ) -> Vector2 {
        let mut new_mouse_offset = Vector2::zero();
        if d.is_key_pressed(KeyboardKey::KEY_R) {
            if let Some(item) = &mut self.selected_item {
                if item.1.size.0 != item.1.size.1 {
                    let selected_item = &mut item.1;
                    selected_item.rotated = !selected_item.rotated;
                }
            }
        }
        if self.selected_item.is_none() {
            if let Some((grid_pos, item)) = self.get_hovered_rect(scale, inventory_top_left, d.get_mouse_position()) {
                let mut rect = self.get_item_rect(grid_pos, scale, inventory_top_left);
                if item.rotated {
                    rect = item.rotate_back(&rect, scale);
                    rect.width += 0.01; // correcting for rounding
                    rect.x += 0.01;
                } else {
                    rect.width += 0.01; // correcting for rounding
                }
                d.draw_rectangle_rec(rect, Color::new(255, 255, 255, 80));
                if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                    self.selected_item = Some((*grid_pos, item.clone(), Vector2::zero()));
                }
            }
        } else if !d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            self.place_selected_item(scale);
        } else if let Some((grid_pos, selected_item, mouse_offset)) = &self.selected_item {
            new_mouse_offset += *mouse_offset + d.get_mouse_delta();
            self.draw_moving_item(d, grid_pos, selected_item, scale, inventory_top_left, new_mouse_offset, assets);
        }
        new_mouse_offset
    }

    fn place_selected_item(
        &mut self,
        scale: f32,
    ) {
        if let Some((grid_pos, selected_item, offset)) = &self.selected_item {
            let new_grid = (
                (grid_pos.0 as f32 + (offset.x / Self::GRID_SPACING / scale)).round() as i32,
                (grid_pos.1 as f32 + (offset.y / Self::GRID_SPACING / scale)).round() as i32
            );
            let item_size = if selected_item.rotated {
                (selected_item.size.1, selected_item.size.0)
            } else {
                (selected_item.size.0, selected_item.size.1)
            };
            if (new_grid.0 + item_size.0 as i32 - 1) < Self::SIZE.0
                && (new_grid.1 + item_size.1 as i32 - 1) < Self::SIZE.1
                && new_grid.0 >= 0
                && new_grid.1 >= 0
            {
                self.items.remove(grid_pos);
                self.items.insert(new_grid, selected_item.clone());
            }
        }
        self.selected_item = None;
    }

    fn draw_moving_item(
        &self,
        d: &mut RaylibDrawHandle,
        grid_pos: &(i32, i32),
        selected_item: &InventoryItem,
        scale: f32,
        inventory_top_left: Vector2,
        new_mouse_offset: Vector2,
        assets: &Assets,
    ) {
        let rect = self.get_item_rect(grid_pos, scale, inventory_top_left);
        let item = self.items.get(grid_pos).unwrap();
        let rotation_offset = if selected_item.rotated != item.rotated {
            if selected_item.rotated {
                selected_item.get_rotation_offset(scale)
            } else {
                -selected_item.get_rotation_offset(scale)
            }
        } else {
            0.0
        };
        let offset_rect = Rectangle {
            x: rect.x + rotation_offset + new_mouse_offset.x,
            y: rect.y + new_mouse_offset.y,
            width: rect.width + 0.01, // Correcting for rounding
            height: rect.height,
        };
        let texture = assets.get_texture(&item.item.get_asset_name());
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
            if selected_item.rotated { 90.0 } else { 0.0 },
            Color::new(255, 255, 255, 255),
        );
    }
}

