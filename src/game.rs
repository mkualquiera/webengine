use glam::Vec3;
use log::info;
use wgpu::Color;
use winit::event::MouseButton;

use crate::{
    renderer::{Drawer, EngineColor, RenderingSystem},
    InputSystem,
};

pub struct Game {
    show_red: bool,
}

impl Game {
    pub fn init(renderer: &mut RenderingSystem) -> Self {
        Self { show_red: true }
    }

    pub fn update(&mut self, input: &InputSystem, delta_time: f32) {
        self.show_red = !input.is_mouse_down(MouseButton::Left);
    }

    pub fn render(&self, drawer: &mut Drawer) {
        drawer.clear_slow(Color::BLACK);
        // stress test
        if self.show_red {
            let t = drawer.ortho;
            drawer.draw_square_slow(
                Some(&t.scale(Vec3 {
                    x: 100.0,
                    y: 100.0,
                    z: 1.0,
                })),
                Some(&EngineColor::RED),
            );
        } else {
            let t = drawer.ortho.translate(Vec3::new(100.0, 0.0, 0.0));
            drawer.draw_square_slow(
                Some(&t.scale(Vec3 {
                    x: 100.0,
                    y: 100.0,
                    z: 1.0,
                })),
                Some(&EngineColor::GREEN),
            );
        }
    }
}
