use std::collections::HashMap;
use std::fmt::format;

use crate::player::*;
use crate::rapier_world::*;
use crate::traits::*;
use collision_world::*;
use debug::DebugInfo;
use raylib::prelude::*;
use world::*;

mod collision_world;
mod debug;
mod draw_collider;
mod player;
mod rapier_world;
mod traits;
mod world;
mod world_collider;

pub struct Assets {
    textures: HashMap<String, Texture2D>,
    error_texture: Texture2D,
}

impl Assets {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Assets {
            textures: Self::load_assets_in_dir(rl, thread, "assets".to_string()),
            error_texture: rl.load_texture_from_image(&thread, &Image::load_image_from_mem(".png", include_bytes!("../assets/error.png")).unwrap()).unwrap()
        }
    }

    fn load_assets_in_dir(rl: &mut RaylibHandle, thread: &RaylibThread, path: String) -> HashMap<String, Texture2D>{
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
                assets.insert(texture_name, rl.load_texture(thread, &format!("{}/{}", path, file_name)).unwrap());
            }
        }
        assets
    }

    pub fn get_texture(&self, texture_name: &str) -> &Texture2D {
        self.textures.get(texture_name).or(Some(&self.error_texture)).unwrap()
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1080, 720)
        .title("Physics")
        .vsync()
        .build();
    let mut camera = Camera2D {
        offset: Vector2::new(0.0, 0.0),
        zoom: 100.0,
        ..Default::default()
    };
    let mut collision_world = CollisionWorld::default();
    let mut game_world = GameWorld::new();
    let mut debugger = DebugInfo::new();
    let mut player = Player::new(&mut collision_world);
    let assets = Assets::new(&mut rl, &thread);
    let mut debug_colliders = vec![];

    spawn_debug_colldier_world(&mut debug_colliders, &mut collision_world);

    while !rl.window_should_close() {
        /*
         * Update
         */
        let mouse_pos = rl.get_mouse_position();
        debugger.update(&mut rl);
        collision_world.step(&rl);
        player.apply_collision_damage(&mut collision_world, &mut game_world.bullets);
        game_world.apply_damage_dummies(&mut rl, &mut collision_world);
        player.control_movement(&rl, &mut collision_world);
        player.handle_shooting(
            &mut rl,
            &mut collision_world,
            &mut game_world.bullets,
            camera.to_world(mouse_pos),
        );
        camera.handle_camera_controls(&rl);
        camera.track(
            player.collider.get_center_of_mass(&collision_world),
            Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32),
        );
        player.handle_spawning_dunmmies(&rl, &camera, &mut collision_world, &mut game_world);
        game_world.handle_bullet_physics(&rl, &mut collision_world);

        debugger.add(format!("Game FPS: {}", rl.get_fps()));
        debugger.add(format!(
            "Num Colliders: {}",
            collision_world.rapier.rigid_body_set.len()
        ));
        debugger.add(format!("Health: {:?} ", player.health));

        /*
         * Drawing
         */

        // World
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        game_world.render_bullets(&mut d, &camera, &mut collision_world);
        for debug_collider in &debug_colliders {
            debug_collider.draw(&collision_world, &camera, &mut d);
        }
        game_world.render_dummies(
            &player,
            &camera,
            &mut collision_world,
            &mut d,
            &assets,
        );
        player.render(
            &mut d,
            &camera,
            &mut collision_world,
            &assets,
            mouse_pos,
        );
        debugger.add(format!(
            "Mouse_pos: ({:?}, {:?})",
            camera.to_world(mouse_pos).x as i32,
            camera.to_world(mouse_pos).y as i32
        ));

        // UI
        debugger.draw(&mut d);
    }
}
