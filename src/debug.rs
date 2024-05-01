use raylib::prelude::*;

// Stores debug info about the world
pub struct DebugInfo {
    pub info: Vec<String>,
    pub debug: bool,
}

impl DebugInfo {
    pub fn new() -> DebugInfo {
        DebugInfo {
            info: vec![],
            debug: false,
        }
    }
    pub fn update(&mut self, rl: &mut RaylibHandle) {
        self.info = vec![];
        if rl.is_key_pressed(KeyboardKey::KEY_F1) {
            self.debug = !self.debug;
        }
        self.info
            .push("(Press F1 to shrink debug info)".to_string());
    }
    pub fn add(&mut self, info: String) {
        self.info.push(info)
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let font_size = 40;
        if self.debug {
            for (i, info) in self.info.iter().enumerate() {
                d.draw_text(
                    info,
                    font_size / 5,
                    i as i32 * font_size + font_size / 10,
                    font_size,
                    Color::WHITE,
                );
            }
        } else {
            d.draw_text(
                "(Press F1 to expand debug menu)",
                font_size / 5,
                font_size / 10,
                font_size,
                Color::WHITE,
            );
        }
    }
}
