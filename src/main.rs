use std::collections::HashMap;

use crate::player::*;
use crate::rapier_world::*;
use crate::traits::*;
use assets::Assets;
use collision_world::*;
use debug::DebugInfo;
use game_map::GameMap;
use inventory::*;
use lighting::Light;
use lighting::LightEngine;
use lighting_renderer::LightingRenderer;
use raylib::prelude::*;
use world::*;

mod assets;
mod collision_world;
mod debug;
mod draw_collider;
mod game_map;
mod inventory;
mod lighting;
mod lighting_renderer;
mod player;
mod rapier_world;
mod traits;
mod world;
mod world_collider;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1080, 720)
        .resizable()
        .title("Physics")
        //.vsync()
        .build();
    let mut camera = Camera2D {
        offset: Vector2::new(0.0, 0.0),
        zoom: 50.0,
        ..Default::default()
    };
    let mut collision_world = CollisionWorld::default();
    let mut lighting_renderer = LightingRenderer::new(&mut rl, &thread);
    let mut light_engine = LightEngine::new(&mut lighting_renderer.shader);
    let mut game_world = GameWorld::new();
    let mut debugger = DebugInfo::new();
    let mut player = Player::new(&mut collision_world, &mut light_engine);
    let assets = Assets::new(&mut rl, &thread);
    let mut debug_colliders = vec![];
    light_engine
        .spawn_light(Light::Ambient {
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
        })
        .unwrap();

    let map = GameMap::load_map("maps/map.tmx");
    let mut inventory = Inventory {
        items: HashMap::new(),
        selected_item: None,
    };
    inventory
        .items
        .insert((0, 0), Item::Rifle.as_inventory_item(false));
    inventory
        .items
        .insert((4, 0), Item::Pistol.as_inventory_item(true));
    inventory
        .items
        .insert((6, 0), Item::MedKit.as_inventory_item(false));

    game_world
        .ground_items
        .push(Item::MedKit.as_ground_item(Vector2::new(0.0, 0.0)));

    spawn_debug_colldier_world(&mut debug_colliders, &mut collision_world);

    while !rl.window_should_close() {
        /*
         * Update
         */
        let mouse_pos = rl.get_mouse_position();
        debugger.update(&mut rl);
        player.apply_collision_damage(&mut collision_world, &mut game_world.bullets);
        game_world.handle_corpses(&rl);
        game_world.handle_dummies(&mut rl, &player, &mut collision_world, &mut light_engine);
        player.handle_controls(&rl, &camera, &mut collision_world);
        player.handle_shooting(
            &mut rl,
            &mut collision_world,
            &mut game_world.bullets,
            camera.to_world(mouse_pos),
        );
        player.update_player_light(&mut light_engine, &mut collision_world);
        camera.handle_camera_controls(&rl);
        camera.track(
            player.collider.get_center_of_mass(&collision_world),
            Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32),
        );
        player.handle_spawning_dunmmies(
            &rl,
            &camera,
            &mut collision_world,
            &mut game_world,
            &mut light_engine,
        );
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
        map.render_map(
            &mut d,
            &camera,
            &assets,
            &thread,
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
        game_world.render_entities(
            &mut d,
            &thread,
            &mut lighting_renderer,
            &mut collision_world,
            &camera,
            &assets,
            &player,
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
        inventory.render(&mut d, &player, &assets, &mut game_world, player.collider.get_pos(&collision_world));
        debugger.add(format!("{:?}", inventory.selected_item));
        debugger.draw(&mut d);
    }
}
