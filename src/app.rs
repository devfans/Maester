use wand;
use dragon::{core, ecs};
use crate::span::cursor::CursorSpan;
use crate::component::movement;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Application {
    app: wand::core::Application,
}

#[wasm_bindgen]
impl Application {
    pub fn new() -> Self {
        let mut app = wand::Application::new_with_canvas_id("canvas");
        let state = app.get_state();

        let mut scene = wand::Scene::default(state.clone());
        let section1 = app.new_section("section1", 1., 1., 0.);
        let cursor_span = CursorSpan::new(state.clone(), app.counter.clone(), "cursor", "Cursor:(N/A)", 0.2, 0.2);
        let world_span = wand::WorldSpan::new(state.clone(), app.context.clone(), "world", "World", 1., 1.);
        let w = &world_span.world.state;
        {
            // Attach cube entity
            let entity = world_span.world.state.create_entity();
            let vertices = [
                (-2., -2., 4.,),
                (2., -2., 4.),
                (2., 2., 4.),
                (-2., 2., 4.),
                (-2., -2., 4.,),
                (-2., -2., -4.,),
                (2., -2., -4.),
                (2., 2., -4.),
                (-2., 2., -4.),
                (-2., -2., -4.,),
                // other lines
                (2., -2., 4.),
                (2., -2., -4.),
                (2., 2., 4.),
                (2., 2., -4.),
                (-2., 2., 4.),
                (-2., 2., -4.)
            ]
                .into_iter()
                .map(|v| core::Point3::new(v.0, v.1, v.2)).collect();
            let mesh = core::BasicMesh::new(vertices, vec!(9, 11, 13));
            let mut transform = ecs::TransformComponent::default();
            transform.set_translation_xyz(-5., 0., -16.);
            w.bind_component(entity, mesh);
            w.bind_component(entity, transform);
            let movement_system = movement::MovementSystem::new(w.clone(), app.input.clone());
            w.register_system("movement", movement_system);
        }

        // Add a simple mesh entity
        {
            // Attach cube entity
            let entity = world_span.world.state.create_entity();
            let vertices = [
                (-2., -2., 4.,),  // 0
                (2., -2., 4.),    // 1
                (2., 2., 4.),     // 2
                (-2., 2., 4.),    // 3
                (-2., -2., -4.,), // 0' 4
                (2., -2., -4.),   // 1' 5
                (2., 2., -4.),    // 2' 6
                (-2., 2., -4.),   // 3' 7
            ]
                .into_iter()
                .map(|v| core::Point3::new(v.0, v.1, v.2)).collect();
            let polygons = vec!(
                (0, 2, 1, "silver"),
                (0, 2, 3, "silver"),
                (4, 5, 6, "grey"),
                (4, 7, 6, "grey"),
                (0, 7, 3, "white"),
                (0, 7, 4, "white"),
                (1, 6, 2, "blue"),
                (1, 6, 5, "blue"),
                (2, 6, 7, "red"),
                (2, 3, 7, "red"),
                (0, 5, 1, "orange"),
                (0, 5, 4, "orange"),
            ).into_iter().map(|(a, b, c, d)| (a, b, c, d.to_string())).collect();
            let mesh = core::SimpleMesh::new(vertices, polygons);
            let mut transform = ecs::TransformComponent::default();
            transform.set_translation_xyz(5., 0., -16.);
            w.bind_component(entity, mesh);
            w.bind_component(entity, transform);
            let movement_system = movement::MovementSystem::new(w.clone(), app.input.clone());
            w.register_system("movement", movement_system);
        }

        {
            let mut section = section1.borrow_mut();
            section.register_span(cursor_span);
            section.register_span(world_span);
        }

        scene.add_section(&section1);
        app.register(scene);

        Self {
            app,
        }
    }

    pub fn draw(&self) {
        self.app.draw();
    }

    pub fn tick(&mut self) {
        self.app.tick();
    }


    pub fn on_size_change(&mut self) {
        self.app.on_resize();
    }

    pub fn on_keyup(&mut self, key: &str) {
        self.app.on_keyup(key);
    }
 
    pub fn on_keydown(&mut self, key: &str) {
        self.app.on_keydown(key);
    }
    
    pub fn on_mouse_move(&mut self, x: f64, y: f64) {
        self.app.on_mouse_move(x, y);
        {
            let state = self.app.get_state();
            let state = state.borrow_mut();
            let cursor = state.fetch_span("cursor").unwrap();
            let mut cursor = cursor.borrow_mut();
            // log!("Call {}", cursor.get_name());
            cursor.as_mut().dispath(
                Box::new(format!("Cursor: x: {}, y: {}", x, y))
            );
        }
        // self.app.draw();
    }
}
