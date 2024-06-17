use crate::player::*;
use crate::rapier_world::*;
use crate::traits::*;
use assets::Assets;
use collision_world::*;
use debug::DebugInfo;
use lighting::LightEngine;
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
mod lighting;
mod assets;

pub struct LightingRenderer {
    pub shader: Shader,
    target: RenderTexture2D,
    shadow_target: RenderTexture2D,
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
            shadow_target: rl
                .load_render_texture(
                    thread,
                    rl.get_screen_width() as u32,
                    rl.get_screen_height() as u32,
                )
                .unwrap(),
        }
    }

    // Updates internal renderer target to resize with the window
    pub fn update_target(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) {
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        if rl.is_window_resized() {
            self.target = rl
                .load_render_texture(thread, screen_size.x as u32, screen_size.y as u32)
                .unwrap();
            self.shadow_target = rl
                .load_render_texture(thread, screen_size.x as u32, screen_size.y as u32)
                .unwrap();
        }
    }

    // Clears the internal target with black background
    fn clear_target(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        d.begin_texture_mode(thread, &mut self.target)
            .clear_background(Color::BLACK);
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
    let mut lighting_renderer = LightingRenderer::new(&mut rl, &thread);
    let mut light_engine = LightEngine::new(&mut lighting_renderer.shader);
    let mut game_world = GameWorld::new();
    let mut debugger = DebugInfo::new();
    let mut player = Player::new(&mut collision_world);
    let assets = Assets::new(&mut rl, &thread);
    let mut debug_colliders = vec![];
    light_engine.spawn_light(lighting::Light::Ambient { color: Vector4::new(1.0, 1.0, 1.0, 1.0)}).unwrap();

    spawn_debug_colldier_world(&mut debug_colliders, &mut collision_world);

    while !rl.window_should_close() {
        /*
         * Update
         */
        let mouse_pos = rl.get_mouse_position();
        debugger.update(&mut rl);
        player.apply_collision_damage(&mut collision_world, &mut game_world.bullets);
        game_world.handle_corpses(&rl);
        game_world.handle_dummies(&mut rl, &player, &mut collision_world);
        player.control_movement(&rl, &camera, &mut collision_world);
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

        collision_world.step(&rl);

        /*
         * Drawing
         */

        // World
        lighting_renderer.update_target(&mut rl, &thread);
        let mut d = rl.begin_drawing(&thread);
        lighting_renderer.clear_target(&mut d, &thread);
        light_engine.update_shader_values(&mut lighting_renderer.shader, &camera, Vector2::new(d.get_screen_width() as f32, d.get_screen_height() as f32));
        d.clear_background(Color::BLACK);
        game_world.render_bullets(&mut d, &thread, &camera, &mut collision_world, &mut lighting_renderer.target);
        for debug_collider in &debug_colliders {
            debug_collider.draw(&collision_world, &camera, &mut d, &thread, &mut lighting_renderer.target);
        }
        game_world.render_corpses(&camera, &mut d, &assets, &thread, &mut lighting_renderer.target);
        game_world.render_dummies(&player, &camera, &mut collision_world, &mut d, &assets, &thread, &mut lighting_renderer.target);
        player.render(&mut d, &camera, &mut collision_world, &assets, &thread, &mut lighting_renderer.target);

        // Debugger
        debugger.add(format!("Game FPS: {}", d.get_fps()));
        debugger.add(format!(
            "Num Colliders: {}",
            collision_world.rapier.rigid_body_set.len()
        ));
        debugger.add(format!("Health: {:?} ", player.health));
        debugger.add(format!(
            "Mouse_pos: ({:?}, {:?})",
            camera.to_world(mouse_pos).x,
            camera.to_world(mouse_pos).y,
        ));
        debugger.add(format!(
            "Corpses: {:?}",
            game_world.corpses.len(),
        ));
        let mut sh = d.begin_shader_mode(&lighting_renderer.shader);
        sh.draw_texture(&mut lighting_renderer.target, 0, 0, Color::WHITE);
        drop(sh);
        // UI
        debugger.draw(&mut d);
    }
}
