mod game;
mod renderer;

use core::panic;
use game::Game;
use log::info;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::{cell::RefCell, sync::Mutex};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, Window};
use winit::event::{ElementState, MouseButton};
use winit::window;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    platform::web::WindowExtWebSys,
    window::{Window as WinitWindow, WindowId},
};

use crate::renderer::RenderingSystem;

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");

    wasm_bindgen_futures::spawn_local(run());
}

async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = WebApp::new();

    event_loop.run_app(&mut app).unwrap();
}

enum AppState {
    Loading {
        game: Arc<Mutex<Option<Game>>>,
        renderer: Arc<Mutex<Option<RenderingSystem>>>,
        window: Arc<Mutex<Option<Arc<WinitWindow>>>>,
    },
    Loaded {
        game: Game,
        renderer: RenderingSystem,
        window: Arc<WinitWindow>,
        input: InputSystem,
    },
}

#[derive(Default)]
struct InputSystem {
    mouse_position: (f64, f64),
    mouse_buttons: HashMap<MouseButton, ElementState>,
}

impl InputSystem {
    fn is_mouse_down(&self, button: MouseButton) -> bool {
        matches!(self.mouse_buttons.get(&button), Some(ElementState::Pressed))
    }
    fn is_mouse_up(&self, button: MouseButton) -> bool {
        matches!(
            self.mouse_buttons.get(&button),
            Some(ElementState::Released)
        )
    }
}

impl AppState {
    fn is_loading(&self) -> bool {
        matches!(self, AppState::Loading { .. })
    }

    fn is_loaded(&self) -> bool {
        matches!(self, AppState::Loaded { .. })
    }

    // Mutably advances the state in place, returns true if advancement happened
    fn advance_in_place(&mut self) -> bool {
        match self {
            AppState::Loading {
                game,
                renderer,
                window,
            } => {
                // Check if all components are ready
                let renderer_ready = renderer.lock().unwrap().is_some();
                let game_ready = game.lock().unwrap().is_some();
                let window_ready = window.lock().unwrap().is_some();

                if renderer_ready && game_ready && window_ready {
                    // Take the values out
                    let renderer = renderer.lock().unwrap().take().unwrap();
                    let game = game.lock().unwrap().take().unwrap();
                    let window = window.lock().unwrap().take().unwrap();

                    // Replace self with the new state
                    *self = AppState::Loaded {
                        game,
                        renderer,
                        window,
                        input: InputSystem::default(),
                    };
                    true
                } else {
                    false
                }
            }
            AppState::Loaded { .. } => false,
        }
    }
}

struct WebApp {
    state: Box<AppState>,
    last_time: Option<f64>,
}

impl WebApp {
    fn new() -> Self {
        Self {
            state: Box::new(AppState::Loading {
                game: Arc::new(Mutex::new(None)),
                renderer: Arc::new(Mutex::new(None)),
                window: Arc::new(Mutex::new(None)),
            }),
            last_time: None,
        }
    }
}

impl ApplicationHandler for WebApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = std::sync::Arc::new(
            event_loop
                .create_window(winit::window::WindowAttributes::default().with_title("WebEngine"))
                .unwrap(),
        );

        let web_window = web_sys::window().unwrap();
        let document = web_window.document().unwrap();
        let canvas: HtmlCanvasElement = window.canvas().unwrap();

        let container = document
            .get_element_by_id("webengine-container")
            .unwrap_or_else(|| {
                let body = document.body().unwrap();
                let container = document.create_element("div").unwrap();
                container.set_id("webengine-container");
                body.append_child(&container).unwrap();
                container
            });

        container.append_child(&canvas).unwrap();

        canvas.set_width(800);
        canvas.set_height(600);
        canvas.style().set_property("width", "800px").unwrap();
        canvas.style().set_property("height", "600px").unwrap();

        let _ = window.request_inner_size(winit::dpi::PhysicalSize::new(800u32, 600u32));

        if let AppState::Loading {
            game,
            renderer,
            window: window_state,
        } = &mut *self.state
        {
            // Store the window in the state
            *window_state.lock().unwrap() = Some(window.clone());

            let renderer_clone = Arc::clone(renderer);
            let game_clone = Arc::clone(game);
            wasm_bindgen_futures::spawn_local(async move {
                let mut renderer = RenderingSystem::new(window.clone(), 800, 600).await;
                let game = Game::init(&mut renderer);

                *renderer_clone.lock().unwrap() = Some(renderer);
                *game_clone.lock().unwrap() = Some(game);
            });
        } else {
            panic!("AppState is not Loading");
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // Try to advance the state
        self.state.advance_in_place();

        // Handle events if we're loaded
        if let AppState::Loaded {
            game,
            renderer,
            window,
            input,
        } = &mut *self.state
        {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    // Handle resize - you'll need to implement this method on your renderer
                    // renderer.resize(physical_size.width, physical_size.height);
                    renderer.resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    // Handle render - you'll need to implement this method
                    // match renderer.render(&game) {
                    //     Ok(_) => {}
                    //     Err(e) => log::error!("Render error: {:?}", e),
                    // }
                    let now = web_sys::window().unwrap().performance().unwrap().now();
                    // Only call update if we have a last time
                    if let Some(last_time) = self.last_time {
                        let delta_time = (now - last_time) as f32 / 1000.0; // Convert to seconds
                        game.update(input, delta_time);
                    }
                    self.last_time = Some(now);

                    //match game.render(renderer) {
                    //    Ok(_) => {}
                    //    Err(wgpu::SurfaceError::Lost) => {
                    //        renderer.canonical_resize();
                    //    }
                    //    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    //    Err(e) => log::error!("{:?}", e),
                    //}

                    match renderer.render(game) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            renderer.canonical_resize();
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => log::error!("{:?}", e),
                    }

                    window.request_redraw();
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    // Update mouse input state
                    input.mouse_buttons.insert(button, state);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    // Update mouse position
                    input.mouse_position = (position.x, position.y);
                }
                _ => {}
            }
        }
    }
}
