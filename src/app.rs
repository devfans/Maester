use crate::span::cursor::CursorSpan;
use crate::span::godswood;

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
        let world_span = godswood::create_godswood(&app);
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
