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

pub struct Inventory {}

impl Inventory {
    pub fn new() -> Self {
        Inventory {}
    }

    pub fn update(&mut self, _rl: &mut RaylibHandle) {}

    pub fn render(&self, d: &mut RaylibDrawHandle, assets: &Assets) {
        let slot_texture = assets.get_texture("slot.png");
        let scale = 3.5;
        let inventory_size = (8, 12);
        let inventory_offset = Vector2::new(0.0, 0.0);
        let inventory_tint = Color::new(255, 255, 255, 230);

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

        // Drawing inner slots
        for x in 0..inventory_size.0 {
            for y in 0..inventory_size.1 {
                let dest = Rectangle::new(
                    x as f32 * 16.0 * scale + 4.0 * scale + inventory_offset.x,
                    y as f32 * 16.0 * scale + 4.0 * scale + inventory_offset.y,
                    16.0 * scale,
                    16.0 * scale,
                );
                draw_part(d, SLOT_PARTS[0], dest);
            }
        }

        // Drawing borders and corners
        for x in 0..inventory_size.0 {
            draw_part(
                d,
                SLOT_PARTS[1],
                Rectangle::new(
                    x as f32 * 16.0 * scale + 4.0 * scale + inventory_offset.x,
                    inventory_offset.y,
                    16.0 * scale,
                    4.0 * scale,
                ),
            );
            draw_part(
                d,
                SLOT_PARTS[2],
                Rectangle::new(
                    x as f32 * 16.0 * scale + 4.0 * scale + inventory_offset.x,
                    inventory_size.1 as f32 * 16.0 * scale + 4.0 * scale + inventory_offset.y,
                    16.0 * scale,
                    4.0 * scale,
                ),
            );
        }

        for y in 0..inventory_size.1 {
            draw_part(
                d,
                SLOT_PARTS[3],
                Rectangle::new(
                    inventory_offset.x,
                    y as f32 * 16.0 * scale + 4.0 * scale + inventory_offset.y,
                    4.0 * scale,
                    16.0 * scale,
                ),
            );
            draw_part(
                d,
                SLOT_PARTS[4],
                Rectangle::new(
                    inventory_size.0 as f32 * 16.0 * scale + 4.0 * scale + inventory_offset.x,
                    y as f32 * 16.0 * scale + 4.0 * scale + inventory_offset.y,
                    4.0 * scale,
                    16.0 * scale,
                ),
            );
        }

        let corners = [
            (SLOT_PARTS[5], (0.0, 0.0)),
            (SLOT_PARTS[6], (inventory_size.0 as f32 * 16.0 * scale + 4.0 * scale, 0.0)),
            (
                SLOT_PARTS[7],
                (
                    inventory_size.0 as f32 * 16.0 * scale + 4.0 * scale,
                    inventory_size.1 as f32 * 16.0 * scale + 4.0 * scale,
                ),
            ),
            (
                SLOT_PARTS[8],
                (0.0, inventory_size.1 as f32 * 16.0 * scale + 4.0 * scale),
            ),
        ];

        for (part, (dx, dy)) in corners.iter() {
            draw_part(
                d,
                *part,
                Rectangle::new(
                    inventory_offset.x + dx,
                    inventory_offset.y + dy,
                    part.2 * scale,
                    part.3 * scale,
                ),
            );
        }
    }
}
