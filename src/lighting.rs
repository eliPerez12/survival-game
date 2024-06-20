#![allow(dead_code)]
use raylib::prelude::*;
use std::collections::HashMap;

use crate::ImprovedCamera;

pub const AMBIENT_LIGHT_NIGHT: Light = Light::Ambient {
    color: Vector4::new(0.7, 0.7, 1.0, 0.25),
};
pub const AMBIENT_LIGHT_MIDNIGHT: Light = Light::Ambient {
    color: Vector4::new(0.0, 0.0, 0.0, 1.0),
};
pub const AMBIENT_LIGHT_SUNRISE: Light = Light::Ambient {
    color: Vector4::new(1.0, 0.7, 0.5, 0.5),
};
pub const AMBIENT_LIGHT_DAY: Light = Light::Ambient {
    color: Vector4::new(1.0, 1.0, 1.0, 1.00),
};

#[derive(Clone)]
pub enum Light {
    Radial {
        pos: Vector2,
        color: Vector4,
        radius: f32,
    },
    Ambient {
        color: Vector4,
    },
    Cone {
        pos: Vector2,
        color: Vector4,
        radius: f32,
        rotation: f32,
        angle: f32,
    },
}

impl Light {
    pub fn default_radial() -> Light {
        Light::Radial {
            pos: Vector2::new(0.0, 0.0),
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
            radius: 150.0,
        }
    }
    pub fn default_ambient() -> Light {
        Light::Ambient {
            color: Color::WHITE.into(),
        }
    }
    pub fn default_cone() -> Light {
        Light::Cone {
            pos: Vector2::new(0.0, 0.0),
            color: Color::WHEAT.into(),
            radius: 250.0,
            rotation: 0.0,
            angle: PI as f32 / 3.0,
        }
    }

    pub fn set_pos(&mut self, world_pos: Vector2) -> &mut Self {
        match self {
            Light::Radial { pos, .. } => *pos = world_pos,
            Light::Cone { pos, .. } => *pos = world_pos,
            _ => (),
        };
        self
    }
    pub fn set_radius(&mut self, new_radius: f32) -> &mut Self {
        match self {
            Light::Radial { radius, .. } => *radius = new_radius,
            Light::Cone { radius, .. } => *radius = new_radius,
            _ => (),
        };
        self
    }

    pub fn set_rotation(&mut self, rot: f32) -> &mut Self {
        if let Light::Cone { rotation, .. } = self {
            *rotation = rot
        }
        self
    }

    pub fn set_color(&mut self, new_color: Vector4) -> &mut Self {
        match self {
            Light::Radial { color, .. } => *color = new_color,
            Light::Ambient { color, .. } => *color = new_color,
            Light::Cone { color, .. } => *color = new_color,
        }
        self
    }

    pub fn color(&self) -> Vector4 {
        match self {
            Light::Radial { color, .. } => *color,
            Light::Ambient { color } => *color,
            Light::Cone { color, .. } => *color,
        }
    }
    pub fn pos(&self) -> Vector2 {
        match self {
            Light::Radial { pos, .. } => *pos,
            Light::Ambient { .. } => Vector2::zero(),
            Light::Cone { pos, .. } => *pos,
        }
    }
    pub fn radius(&self) -> f32 {
        match self {
            Light::Radial { radius, .. } => *radius,
            Light::Ambient { .. } => 0.0,
            Light::Cone { radius, .. } => *radius,
        }
    }
    pub fn light_type(&self) -> i32 {
        match self {
            Light::Radial { .. } => 0,
            Light::Ambient { .. } => 1,
            Light::Cone { .. } => 2,
        }
    }
    pub fn rotation(&self) -> f32 {
        match self {
            Light::Cone { rotation, .. } => *rotation,
            _ => 0.0,
        }
    }
    pub fn angle(&self) -> f32 {
        match self {
            Light::Cone { angle, .. } => *angle,
            _ => 0.0,
        }
    }
}

// Used to store the shader uniform locations. Each i32 is a loc.
struct ShaderUniforms {
    position: i32,
    color: i32,
    amount: i32,
    radius: i32,
    light_type: i32,
    screen_size: i32,
    rotation: i32,
    angle: i32,
}

pub struct LightEngine {
    lights: HashMap<u32, Light>,
    light_id: u32,
    shader_uniforms: ShaderUniforms,
}

pub struct LightHandle(u32);

impl LightEngine {
    // Setting the shader locations
    pub fn new(shader: &mut Shader) -> LightEngine {
        LightEngine {
            lights: HashMap::new(),
            light_id: 0,
            shader_uniforms: ShaderUniforms {
                position: shader.get_shader_location("lightsPosition"),
                color: shader.get_shader_location("lightsColor"),
                amount: shader.get_shader_location("lightsAmount"),
                radius: shader.get_shader_location("lightsRadius"),
                light_type: shader.get_shader_location("lightsType"),
                rotation: shader.get_shader_location("lightsRotation"),
                angle: shader.get_shader_location("lightsAngle"),
                screen_size: shader.get_shader_location("screenSize"),
            },
        }
    }
    pub fn spawn_light(&mut self, light: Light) -> Result<LightHandle, ()> {
        if self.light_id < 400 {
            self.lights.insert(self.light_id, light);
            self.light_id += 1;
            Ok(LightHandle(self.light_id - 1))
        } else {
            Err(())
        }
    }

    pub fn remove_light(&mut self, light_handle: &LightHandle) {
        &mut self.lights.remove(&light_handle.0);
    }

    pub fn update_light(&mut self, light_handle: &LightHandle, updated_light: Light) {
        self.lights.insert(light_handle.0, updated_light);
    }
    pub fn get_mut_light(&mut self, light_handle: &LightHandle) -> &mut Light {
        self.lights.get_mut(&light_handle.0).unwrap()
    }

    pub fn spawned_lights(&self) -> usize {
        self.lights.len()
    }


    // Updating the shader with new uniform values
    pub fn update_shader_values(
        &self,
        shader: &mut Shader,
        camera: &Camera2D,
        screen_size: Vector2,
    ) {
        shader.set_shader_value_v(
            self.shader_uniforms.position,
            self.lights
                .iter()
                .map(|light| (light.1.pos() + camera.offset) * camera.zoom)
                .collect::<Vec<Vector2>>()
                .as_slice(),
        );
        shader.set_shader_value_v(
            self.shader_uniforms.color,
            self.lights
                .iter()
                .map(|light| light.1.color())
                .collect::<Vec<Vector4>>()
                .as_slice(),
        );
        shader.set_shader_value(self.shader_uniforms.amount, self.lights.len() as i32);
        shader.set_shader_value_v(
            self.shader_uniforms.radius,
            self.lights
                .iter()
                .map(|light| light.1.radius() * camera.zoom)
                .collect::<Vec<f32>>()
                .as_slice(),
        );
        shader.set_shader_value_v(
            self.shader_uniforms.light_type,
            self.lights
                .iter()
                .map(|light| light.1.light_type())
                .collect::<Vec<i32>>()
                .as_slice(),
        );
        shader.set_shader_value_v(
            self.shader_uniforms.rotation,
            self.lights
                .iter()
                .map(|light| light.1.rotation())
                .collect::<Vec<f32>>()
                .as_slice(),
        );
        shader.set_shader_value_v(
            self.shader_uniforms.angle,
            self.lights
                .iter()
                .map(|light| light.1.angle())
                .collect::<Vec<f32>>()
                .as_slice(),
        );
        shader.set_shader_value(self.shader_uniforms.screen_size, screen_size);
    }

    pub fn handle_spawning_light(&mut self, rl: &mut RaylibHandle, camera: &Camera2D) {
        let pos = camera.to_world(rl.get_mouse_position());
        let light_radius = Light::default_radial().radius();
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            self.spawn_light(Light::Radial {
                pos,
                color: Color::WHITE.into(),
                radius: light_radius,
            })
            .unwrap();
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            self.spawn_light(Light::Radial {
                pos,
                color: Color::RED.into(),
                radius: light_radius,
            })
            .unwrap();
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            self.spawn_light(Light::Radial {
                pos,
                color: Color::BLUE.into(),
                radius: light_radius,
            })
            .unwrap();
        }
        if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
            self.spawn_light(Light::Radial {
                pos,
                color: Color::YELLOW.into(),
                radius: light_radius,
            })
            .unwrap();
        }
    }
}
