use glam::Vec3;
use log::info;
use wgpu::Color;
use winit::event::MouseButton;

use crate::{
    renderer::{Drawer, EngineColor, RenderingSystem, Vertex},
    InputSystem,
};

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0, 0.0, 0.0],
        color: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [0.0, 100.0, 0.0],
        color: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [100.0, 100.0, 0.0],
        color: [1.0, 1.0, 1.0],
    },
];

// Rendering order is "up, bottom left, bottom right"
const INDICES: &[u16] = &[0, 1, 2];

pub struct Game {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    show_red: bool,
}

impl Game {
    pub fn init(renderer: &mut RenderingSystem) -> Self {
        let vertex_buffer_red = renderer.create_vertex_buffer(VERTICES);
        let index_buffer = renderer.create_index_buffer(INDICES);
        let num_indices = INDICES.len() as u32;

        Self {
            vertex_buffer: vertex_buffer_red,
            index_buffer,
            num_indices,
            show_red: true,
        }
    }

    pub fn update(&mut self, input: &InputSystem, delta_time: f32) {
        self.show_red = !input.is_mouse_down(MouseButton::Left);
    }

    pub fn render(&self, drawer: &mut Drawer) {
        drawer.clear_slow(Color::BLACK);
        // stress test
        if self.show_red {
            let t = drawer.ortho;
            drawer.draw_geometry_slow(
                &self.vertex_buffer,
                &self.index_buffer,
                self.num_indices,
                Some(t),
                Some(&EngineColor {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }),
            );
        } else {
            let t = drawer.ortho.translate(Vec3::new(100.0, 0.0, 0.0));
            drawer.draw_geometry_slow(
                &self.vertex_buffer,
                &self.index_buffer,
                self.num_indices,
                Some(&t),
                Some(&EngineColor {
                    r: 0.0,
                    g: 1.0,
                    b: 0.0,
                    a: 1.0,
                }),
            );
        }
    }
}
