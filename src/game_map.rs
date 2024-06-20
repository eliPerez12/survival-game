use raylib::prelude::*;
use tiled::Map;

use crate::{assets::Assets, ImprovedCamera};

pub struct GameMap {
    map: Map,
}

impl GameMap {
    pub fn load_map(path: &str) -> Self {
        GameMap {
            map: tiled::Loader::new().load_tmx_map(path).unwrap(),
        }
    }

    pub fn render_map(
        &self,
        d: &mut RaylibDrawHandle,
        camera: &Camera2D,
        assets: &Assets,
        thread: &RaylibThread,
        target: &mut RenderTexture2D,
    ) {
        let mut d = d.begin_texture_mode(thread, target);
        let scale = 0.1;
        let texture = assets.get_texture("tiles/tilelist.png");
        let camera_world_rect = camera.get_visible_rect(Vector2::new(
            d.get_screen_width() as f32,
            d.get_screen_height() as f32,
        ));

        let tileset = self.map.tilesets().first().unwrap();
        for layer in self.map.layers() {
            let tile_layer = layer.as_tile_layer().unwrap();
            for y in 0..tile_layer.width().unwrap() {
                for x in 0..tile_layer.height().unwrap() {
                    if let Some(tile_id) = tile_layer.get_tile(x as i32, y as i32) {
                        let tileset_index = tile_id.id();
                        let source_rect = Rectangle::new(
                            (tileset_index % tileset.columns) as f32 * 64.0,
                            (tileset_index / tileset.columns) as f32 * 64.0,
                            tileset.tile_width as f32,
                            tileset.tile_height as f32,
                        );
                        let dest_rect = Rectangle::new(
                            x as f32 * tileset.tile_width as f32 * scale,
                            y as f32 * tileset.tile_height as f32 * scale,
                            tileset.tile_width as f32 * scale * 1.001,
                            tileset.tile_height as f32 * scale * 1.001,
                        );
                        if camera_world_rect.check_collision_recs(&dest_rect) {
                            d.draw_texture_pro(
                                texture,
                                source_rect,
                                camera.to_screen_rect(&dest_rect),
                                Vector2::zero(),
                                0.0,
                                Color::WHITE,
                            );
                        }
                    }
                }
            }
        }
    }
}
