use crate::assets::Assets;
use raylib::prelude::*;

pub fn source_rect_of_texture(texture: &Texture2D) -> Rectangle {
    Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32)
}

pub fn render_inventory_grid(
    d: &mut RaylibDrawHandle,
    render_pos: Vector2,
    size: (u32, u32),
    scale: f32,
    tint: Color,
    slot_texture: &Texture2D,
) {
    let draw_part = |d: &mut RaylibDrawHandle, part: (f32, f32, f32, f32), dest: Rectangle| {
        d.draw_texture_pro(
            slot_texture,
            Rectangle::new(part.0, part.1, part.2, part.3),
            dest,
            Vector2::zero(),
            0.0,
            tint,
        );
    };
    // Drawing inner slots
    for x in 0..size.0 {
        for y in 0..size.1 {
            let dest = Rectangle::new(
                x as f32 * 16.0 * scale + 2.0 * scale + render_pos.x,
                y as f32 * 16.0 * scale + 2.0 * scale + render_pos.y,
                16.0 * scale,
                16.0 * scale,
            );
            draw_part(d, SLOT_PARTS[0], dest);
        }
    }

    // Drawing borders and corners
    for x in 0..size.0 {
        draw_part(
            d,
            SLOT_PARTS[1],
            Rectangle::new(
                x as f32 * 16.0 * scale + 2.0 * scale + render_pos.x,
                render_pos.y,
                16.0 * scale,
                2.0 * scale,
            ),
        );
        draw_part(
            d,
            SLOT_PARTS[2],
            Rectangle::new(
                x as f32 * 16.0 * scale + 2.0 * scale + render_pos.x,
                size.1 as f32 * 16.0 * scale + 2.0 * scale + render_pos.y,
                16.0 * scale,
                2.0 * scale,
            ),
        );
    }

    for y in 0..size.1 {
        draw_part(
            d,
            SLOT_PARTS[3],
            Rectangle::new(
                render_pos.x,
                y as f32 * 16.0 * scale + 2.0 * scale + render_pos.y,
                2.0 * scale,
                16.0 * scale,
            ),
        );
        draw_part(
            d,
            SLOT_PARTS[4],
            Rectangle::new(
                size.0 as f32 * 16.0 * scale + 2.0 * scale + render_pos.x,
                y as f32 * 16.0 * scale + 2.0 * scale + render_pos.y,
                2.0 * scale,
                16.0 * scale,
            ),
        );
    }

    let corners = [
        (SLOT_PARTS[5], (0.0, 0.0)),
        (
            SLOT_PARTS[6],
            (size.0 as f32 * 16.0 * scale + 2.0 * scale, 0.0),
        ),
        (
            SLOT_PARTS[7],
            (
                size.0 as f32 * 16.0 * scale + 2.0 * scale,
                size.1 as f32 * 16.0 * scale + 2.0 * scale,
            ),
        ),
        (
            SLOT_PARTS[8],
            (0.0, size.1 as f32 * 16.0 * scale + 2.0 * scale),
        ),
    ];

    for (part, (dx, dy)) in corners.iter() {
        draw_part(
            d,
            *part,
            Rectangle::new(
                render_pos.x + dx,
                render_pos.y + dy,
                part.2 * scale,
                part.3 * scale,
            ),
        );
    }
}

const SLOT_PARTS: [(f32, f32, f32, f32); 9] = [
    (2.0, 2.0, 16.0, 16.0), // SLOT_INNER
    (2.0, 0.0, 16.0, 2.0),  // SLOT_UPPER
    (2.0, 18.0, 16.0, 2.0), // SLOT_LOWER
    (0.0, 2.0, 2.0, 16.0),  // SLOT_LEFT_BORDER
    (18.0, 2.0, 2.0, 16.0), // SLOT_RIGHT_BORDER
    (0.0, 0.0, 2.0, 2.0),   // SLOT_UPPER_LEFT_CORNER
    (18.0, 0.0, 2.0, 2.0),  // SLOT_UPPER_RIGHT_CORNER
    (18.0, 18.0, 2.0, 2.0), // SLOT_BOTTOM_RIGHT_CORNER
    (0.0, 18.0, 2.0, 2.0),  // SLOT_BOTTOM_LEFT_CORNER
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
                (Vector2::new(0.0, 100.0), (4, 8)),
                (Vector2::new(500.0, 100.0), (8, 4)),
            ],
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {}
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, assets: &Assets) {
        let slot_texture = assets.get_texture("slot.png");
        let parent_slot_texture = assets.get_texture("parent_slot.png");
        let inventory_tint = Color::new(255, 255, 255, 220);
        let background_tint = Color::new(0,0,0,100); 
        let parent_inventory_size = (2, 2);
        let font = d.get_font_default();


        // Tint background
        d.draw_rectangle(0, 0, d.get_screen_width(), d.get_screen_height(), background_tint);

        for inventory in &self.inventory_slots {
            d.draw_text_pro(
                &font,
                "Player Inventory",
                inventory.0,
                Vector2::zero(),
                0.0,
                6.0 * self.scale,
                1.0,
                Color::WHITE
            ); 

            let parent_inventory = Rectangle::new(
                inventory.0.x,
                inventory.0.y + font.base_size() as f32 / 1.5 * self.scale,
                (16.0 * parent_inventory_size.0 as f32 + 5.0) * self.scale,
                (16.0 * parent_inventory_size.1 as f32) * self.scale,
            );
            let render_pos = Vector2::new(
                parent_inventory.x + parent_inventory.width,
                parent_inventory.y,
            );
            render_inventory_grid(
                d,
                Vector2::new(parent_inventory.x, parent_inventory.y),
                parent_inventory_size,
                self.scale,
                inventory_tint,
                parent_slot_texture,
            );
            render_inventory_grid(
                d,
                render_pos,
                inventory.1,
                self.scale,
                inventory_tint,
                slot_texture,
            );
        }
    }
}
