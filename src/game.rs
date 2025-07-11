use glam::Vec3;
use log::info;
use wgpu::Color;
use winit::{
    event::MouseButton,
    keyboard::{Key, KeyCode, PhysicalKey},
};

use crate::{
    geometry::Transform,
    renderer::{Drawer, EngineColor, RenderingSystem},
    InputSystem,
};

struct PaddleState {
    position: f32,
}

impl Default for PaddleState {
    fn default() -> Self {
        Self { position: 0.5 }
    }
}

impl PaddleState {
    const PADDLE_WIDTH: f32 = 0.2;
    const PADDLE_HEIGHT: f32 = PaddleState::PADDLE_WIDTH / 4.0;
    const PADDLE_SPEED: f32 = 0.5; // Speed in normalized units

    pub fn local_space(&self, ortho_si: &Transform, is_player_a: bool) -> Transform {
        // Position the origin at the top left

        let horizontal_range = 1.0 - PaddleState::PADDLE_WIDTH;
        let vertical_range = 1.0 - PaddleState::PADDLE_HEIGHT;

        let vertical_position = if is_player_a { 0.0 } else { 1.0 };

        let x = self.position * horizontal_range;
        let y = vertical_position * vertical_range;

        ortho_si.translate(Vec3::new(x, y, 0.0)).scale(Vec3 {
            x: PaddleState::PADDLE_WIDTH,
            y: PaddleState::PADDLE_HEIGHT,
            z: 1.0,
        })
    }

    pub fn move_left(&mut self, delta_time: f32) {
        self.position -= PaddleState::PADDLE_SPEED * delta_time;
        if self.position < 0.0 {
            self.position = 0.0;
        }
    }
    pub fn move_right(&mut self, delta_time: f32) {
        self.position += PaddleState::PADDLE_SPEED * delta_time;
        if self.position > 1.0 - PaddleState::PADDLE_WIDTH {
            self.position = 1.0 - PaddleState::PADDLE_WIDTH;
        }
    }
}

#[derive(Default)]
struct DualPaddleState {
    player_a: PaddleState,
    player_b: PaddleState,
}

impl DualPaddleState {
    pub fn local_spaces(&self, ortho_si: &Transform) -> (Transform, Transform) {
        (
            self.player_a.local_space(ortho_si, true),
            self.player_b.local_space(ortho_si, false),
        )
    }

    pub fn move_paddles(&mut self, input: &InputSystem, delta_time: f32) {
        if input.is_physical_key_down(KeyCode::KeyA) {
            self.player_a.move_left(delta_time);
        }
        if input.is_physical_key_down(KeyCode::KeyD) {
            self.player_a.move_right(delta_time);
        }
        if input.is_physical_key_down(KeyCode::ArrowLeft) {
            self.player_b.move_left(delta_time);
        }
        if input.is_physical_key_down(KeyCode::ArrowRight) {
            self.player_b.move_right(delta_time);
        }
    }
}

pub struct Game {
    paddles: DualPaddleState,
}

impl Game {
    pub fn init(rendering_system: &mut RenderingSystem) -> Self {
        Self {
            paddles: DualPaddleState::default(),
        }
    }

    pub fn update(&mut self, input: &InputSystem, delta_time: f32) {
        self.paddles.move_paddles(input, delta_time);
    }

    pub fn render(&self, drawer: &mut Drawer) {
        drawer.clear_slow(Color::BLACK);

        let t = &Transform::ortographic_size_invariant();
        let (player_a_space, player_b_space) = self.paddles.local_spaces(t);
        drawer.draw_square_slow(Some(&player_a_space), Some(&EngineColor::RED));
        drawer.draw_square_slow(Some(&player_b_space), Some(&EngineColor::BLUE));
    }
}
