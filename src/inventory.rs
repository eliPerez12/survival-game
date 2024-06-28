use raylib::prelude::*;
use crate::assets::Assets;

pub fn source_rect_of_texture(texture: &Texture2D) -> Rectangle {
    Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32)
}

const SLOT_PARTS: [(f32, f32, f32, f32); 9] = [
    (4.0, 4.0, 16.0, 16.0), // SLOT_INNER
    (4.0, 0.0, 16.0, 4.0),  // SLOT_UPPER
    (4.0, 20.0, 16.0, 4.0), // SLOT_LOWER
    (0.0, 4.0, 4.0, 16.0),  // SLOT_LEFT_BORDER
    (20.0, 4.0, 4.0, 16.0), // SLOT_RIGHT_BORDER
    (0.0, 0.0, 4.0, 4.0),   // SLOT_UPPER_LEFT_CORNER
    (20.0, 0.0, 4.0, 4.0),  // SLOT_UPPER_RIGHT_CORNER
    (20.0, 20.0, 4.0, 4.0), // SLOT_BOTTOM_RIGHT_CORNER
    (0.0, 20.0, 4.0, 4.0),  // SLOT_BOTTOM_LEFT_CORNER
];

pub struct Inventory {
    inventory_slots: Vec<(Vector2, (u32, u32))>,
    scale: f32,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            scale: 3.0,
            inventory_slots: vec![
                (Vector2::new(0.0, 0.0), (8, 16)),
                (Vector2::new(500.0, 0.0), (8, 16)),
            ]
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {

        }
    }


    pub fn render(&self, d: &mut RaylibDrawHandle, assets: &Assets) {
        let slot_texture = assets.get_texture("slot.png");
        let inventory_tint = Color::new(255, 255, 255, 220);
        let draw_part = |d: &mut RaylibDrawHandle, part: (f32, f32, f32, f32), dest: Rectangle| {
            d.draw_texture_pro(
                slot_texture,
                Rectangle::new(part.0, part.1, part.2, part.3),
                dest,
                Vector2::zero(),
                0.0,
                inventory_tint,
            );
        };

        
        for inventory in &self.inventory_slots {
            // Drawing inner slots
            for x in 0..inventory.1.0 {
                for y in 0..inventory.1.1 {
                    let dest = Rectangle::new(
                        x as f32 * 16.0 * self.scale + 4.0 * self.scale + inventory.0.x,
                        y as f32 * 16.0 * self.scale + 4.0 * self.scale + inventory.0.y,
                        16.0 * self.scale,
                        16.0 * self.scale,
                    );
                    draw_part(d, SLOT_PARTS[0], dest);
                }
            }

            // Drawing borders and corners
            for x in 0..inventory.1.0 {
                draw_part(
                    d,
                    SLOT_PARTS[1],
                    Rectangle::new(
                        x as f32 * 16.0 * self.scale + 4.0 * self.scale + inventory.0.x,
                        inventory.0.y,
                        16.0 * self.scale,
                        4.0 * self.scale,
                    ),
                );
                draw_part(
                    d,
                    SLOT_PARTS[2],
                    Rectangle::new(
                        x as f32 * 16.0 * self.scale + 4.0 * self.scale + inventory.0.x,
                        inventory.1.1 as f32 * 16.0 * self.scale + 4.0 * self.scale + inventory.0.y,
                        16.0 * self.scale,
                        4.0 * self.scale,
                    ),
                );
            }

            for y in 0..inventory.1.1 {
                draw_part(
                    d,
                    SLOT_PARTS[3],
                    Rectangle::new(
                        inventory.0.x,
                        y as f32 * 16.0 * self.scale + 4.0 * self.scale + inventory.0.y,
                        4.0 * self.scale,
                        16.0 * self.scale,
                    ),
                );
                draw_part(
                    d,
                    SLOT_PARTS[4],
                    Rectangle::new(
                        inventory.1.0 as f32 * 16.0 * self.scale + 4.0 * self.scale + inventory.0.x,
                        y as f32 * 16.0 * self.scale + 4.0 * self.scale + inventory.0.y,
                        4.0 * self.scale,
                        16.0 * self.scale,
                    ),
                );
            }

            let corners = [
                (SLOT_PARTS[5], (0.0, 0.0)),
                (SLOT_PARTS[6], (inventory.1.0 as f32 * 16.0 * self.scale + 4.0 * self.scale, 0.0)),
                (
                    SLOT_PARTS[7],
                    (
                        inventory.1.0 as f32 * 16.0 * self.scale + 4.0 * self.scale,
                        inventory.1.1 as f32 * 16.0 * self.scale + 4.0 * self.scale,
                    ),
                ),
                (
                    SLOT_PARTS[8],
                    (0.0, inventory.1.1 as f32 * 16.0 * self.scale + 4.0 * self.scale),
                ),
            ];

            for (part, (dx, dy)) in corners.iter() {
                draw_part(
                    d,
                    *part,
                    Rectangle::new(
                        inventory.0.x + dx,
                        inventory.0.y + dy,
                        part.2 * self.scale,
                        part.3 * self.scale,
                    ),
                );
            }
        }    
    }
}
