use crate::player::*;
use crate::rapier_world::*;
use crate::traits::*;
use assets::Assets;
use collision_world::*;
use debug::DebugInfo;
use lighting::LightEngine;
use lighting_renderer::LightingRenderer;
use raylib::prelude::*;
use tiled::Map;
use world::*;

mod assets;
mod collision_world;
mod debug;
mod draw_collider;
mod lighting;
mod lighting_renderer;
mod player;
mod rapier_world;
mod traits;
mod world;
mod world_collider;

fn render_map(
    map: &Map,
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

    let tileset = map.tilesets().first().unwrap();
    for layer in map.layers() {
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
                    // if camera_world_rect.check_collision_recs(&dest_rect) {
                    d.draw_texture_pro(
                        texture,
                        source_rect,
                        camera.to_screen_rect(&dest_rect),
                        Vector2::zero(),
                        0.0,
                        Color::WHITE,
                    );
                    // }
                }
            }
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1080, 720)
        .resizable()
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
    light_engine
        .spawn_light(lighting::Light::Ambient {
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
        })
        .unwrap();

    let map = tiled::Loader::new().load_tmx_map("maps/map.tmx").unwrap();

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
        player.handle_controls(&rl, &camera, &mut collision_world);
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
        light_engine.update_shader_values(
            &mut lighting_renderer.shader,
            &camera,
            Vector2::new(d.get_screen_width() as f32, d.get_screen_height() as f32),
        );
        d.clear_background(Color::BLACK);
        render_map(
            &map,
            &mut d,
            &camera,
            &assets,
            &thread,
            &mut lighting_renderer.target,
        );
        game_world.render_bullets(
            &mut d,
            &thread,
            &camera,
            &mut collision_world,
            &mut lighting_renderer.target,
        );
        for debug_collider in &debug_colliders {
            debug_collider.draw(
                &collision_world,
                &camera,
                &mut d,
                &thread,
                &mut lighting_renderer.target,
            );
        }
        game_world.render_corpses(
            &camera,
            &mut d,
            &assets,
            &thread,
            &mut lighting_renderer.target,
        );
        game_world.render_dummies(
            &player,
            &camera,
            &mut collision_world,
            &mut d,
            &assets,
            &thread,
            &mut lighting_renderer.target,
        );
        player.render(
            &mut d,
            &camera,
            &mut collision_world,
            &assets,
            &thread,
            &mut lighting_renderer.target,
        );

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
        debugger.add(format!("Corpses: {:?}", game_world.corpses.len(),));
        let mut sh = d.begin_shader_mode(&lighting_renderer.shader);
        sh.draw_texture(&mut lighting_renderer.target, 0, 0, Color::WHITE);
        drop(sh);
        // UI
        if player.inventory_open {
            let screen_size = Vector2::new(d.get_screen_width() as f32, d.get_screen_height() as f32);
            let texture = assets.get_texture("inventory.png");
            let texture_size = Vector2::new(texture.width() as f32, texture.height() as f32);
            let scale = 5.0;
            let inventory_top_left = Vector2::new(screen_size.x / 2.0, screen_size.y)
                - Vector2::new(texture_size.x / 2.0 * scale, texture_size.y * scale);
                d.draw_rectangle(0, 0, screen_size.x as i32, screen_size.y as i32, Color::new(0, 0, 0, 100));
            let inventory_slot_pos = (3, 1);
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
                Color::new(255, 255, 255, 220),
            );
            let texture = assets.get_texture("417.png");
            d.draw_texture_pro(
                texture,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: texture.width() as f32,
                    height: texture.height() as f32,
                },
                Rectangle {
                    x: inventory_top_left.x + (inventory_slot_pos.0 as f32 * 17.0 * scale) + (4.0 * scale),
                    y: inventory_top_left.y + (inventory_slot_pos.1 as f32 * 17.0 * scale) + (4.0 * scale),
                    width: texture.width() as f32 * scale,
                    height: texture.height() as f32 * scale,
                },
                Vector2::zero(),
                0.0,
                Color::new(255, 255, 255, 255),
            );
        }
        debugger.draw(&mut d);
    }
}
