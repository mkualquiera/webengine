use log::info;
use wgpu::Color;
use winit::event::MouseButton;

use crate::{
    renderer::{Drawer, Renderer, Vertex},
    InputManager,
};

const VERTICES_RED: &[Vertex] = &[
    Vertex {
        position: [-0.0, 0.0, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
];

const VERTICES_GREEN: &[Vertex] = &[
    Vertex {
        position: [-0.0, 0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
];

// Rendering order is "up, bottom left, bottom right"
const INDICES: &[u16] = &[0, 1, 2];

pub struct Game {
    vertex_buffer_red: wgpu::Buffer,
    vertex_buffer_green: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    show_red: bool,
}

impl Game {
    pub fn init(renderer: &mut Renderer) -> Self {
        let vertex_buffer_red = renderer.create_vertex_buffer(VERTICES_RED);
        let vertex_buffer_green = renderer.create_vertex_buffer(VERTICES_GREEN);
        let index_buffer = renderer.create_index_buffer(INDICES);
        let num_indices = INDICES.len() as u32;

        Self {
            vertex_buffer_red,
            vertex_buffer_green,
            index_buffer,
            num_indices,
            show_red: true,
        }
    }

    pub fn update(&mut self, input: &InputManager, delta_time: f32) {
        self.show_red = !input.is_mouse_down(MouseButton::Left);
    }

    pub fn render(&self, drawer: &mut Drawer) {
        drawer.clear_slow(Color::BLACK);
        // stress test
        for _ in 0..1500 {
            if self.show_red {
                drawer.draw_geometry_slow(
                    &self.vertex_buffer_red,
                    &self.index_buffer,
                    self.num_indices,
                );
            } else {
                drawer.draw_geometry_slow(
                    &self.vertex_buffer_green,
                    &self.index_buffer,
                    self.num_indices,
                );
            }
        }
    }
}
