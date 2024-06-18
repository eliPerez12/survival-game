use raylib::prelude::*;

pub struct LightingRenderer {
    pub shader: Shader,
    pub target: RenderTexture2D,
}

impl LightingRenderer {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        LightingRenderer {
            shader: rl.load_shader_from_memory(
                thread,
                None,
                Some(include_str!("../shaders/lighting.fs")),
            ),
            target: rl
                .load_render_texture(
                    thread,
                    rl.get_screen_width() as u32,
                    rl.get_screen_height() as u32,
                )
                .unwrap(),
        }
    }

    // Updates internal renderer target to resize with the window
    pub fn update_target(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        if rl.is_window_resized() {
            self.target = rl
                .load_render_texture(thread, screen_size.x as u32, screen_size.y as u32)
                .unwrap();
        }
    }

    // Clears the internal target with black background
    pub fn clear_target(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        d.begin_texture_mode(thread, &mut self.target)
            .clear_background(Color::BLACK);
    }
}
