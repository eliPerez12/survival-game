use std::collections::HashMap;
use raylib::prelude::*;


pub struct Assets {
    textures: HashMap<String, Texture2D>,
    error_texture: Texture2D,
}

impl Assets {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Assets {
            textures: Self::load_assets_in_dir(rl, thread, "assets".to_string()),
            error_texture: rl
                .load_texture_from_image(
                    thread,
                    &Image::load_image_from_mem(".png", include_bytes!("../assets/error.png"))
                        .unwrap(),
                )
                .unwrap(),
        }
    }

    fn load_assets_in_dir(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: String,
    ) -> HashMap<String, Texture2D> {
        let mut assets = HashMap::new();
        let dir = std::fs::read_dir(path.clone()).unwrap();
        for entry in dir {
            let entry = entry.unwrap();
            let file_name = entry.file_name().to_string_lossy().to_string();
            let mut full_path = format!("{}/{}", path, file_name);
            if entry.file_type().unwrap().is_dir() {
                assets.extend(Self::load_assets_in_dir(rl, thread, full_path));
            } else {
                let texture_name = full_path.split_off(7);
                assets.insert(
                    texture_name,
                    rl.load_texture(thread, &format!("{}/{}", path, file_name))
                        .unwrap(),
                );
            }
        }
        assets
    }

    pub fn get_texture(&self, texture_name: &str) -> &Texture2D {
        self.textures
            .get(texture_name)
            .unwrap_or(&self.error_texture)
    }
}