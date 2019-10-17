use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use std::any::Any;

pub struct CursorSpan {
    pub name: String,
    text: String,

    x: f64,
    y: f64,
    w: f64,
    h: f64,

    pub width: f32,
    pub height: f32,
    pub order: u8,

    state: wand::core::State,
    fps: wand::FpsCounter, 
    font_cache: RefCell<Option<String>>, // Caching proper font for the string
}

impl CursorSpan {
    pub fn new(
        state: wand::core::State,
        fps: wand::FpsCounter, 
        name: &str,
        text: &str,
        width: f32, height: f32) -> Self {
        Self {
            name: name.to_string(),
            text: text.to_string(),
            x: 0.,
            y: 0.,
            w: 0.,
            h: 0.,

            width,
            height,
            state,
            order: 1,
            fps,
            font_cache: RefCell::new(None),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    fn draw_outline(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_fill_style(&JsValue::from_str("black"));
        ctx.fill_rect(self.x, self.y, self.w, self.h);
    }

}

impl wand::SpanTrait for CursorSpan {

    fn get_name(&self) -> &str {
        &self.name
    }

    fn dispatch_event(&mut self, ev: &mut wand::component::Event) {
    }

    fn dispath(&mut self, data: Box<dyn Any>) {
        if let Ok(text) = data.downcast::<String>() {
            self.text = text.to_string();
            // Clear font cache
            let mut font = self.font_cache.borrow_mut();
            *font = None;
        }
    }

    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        self.draw_outline(ctx);
        let mut font = self.font_cache.borrow_mut();
        if font.is_none() {
            let size = wand::utils::get_font_with_limit(ctx, &self.text, self.w * 0.8, "Arial").min(20).max(10);
            *font = Some(format!("{}px {}", size, "Arail"));
        }
        if !font.is_none() {
            ctx.set_font(font.as_ref().unwrap());
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            ctx.set_fill_style(&JsValue::from_str("green"));
            let _ = ctx.fill_text(&format!("FPS: {}/s", self.fps.borrow().get()), self.x + self.w/2., self.y + self.h/2. - 10.);
            let _ = ctx.fill_text(&self.text, self.x + self.w/2., self.y + self.h/2. + 10.);
        }

    }

    fn on_resize(&mut self, left: f64, top: f64, right: f64, bottom: f64) -> (f64, f64, bool) {
        self.x = left;
        self.y = top;
        self.w = self.width as f64 * (right - left);
        self.h = self.height as f64 * (bottom - top);
        // Clear font cache
        let mut font = self.font_cache.borrow_mut();
        *font = None;
        (0., 0., true)
    }

    fn get_order(&self) -> u8 {
        self.order
    }

}



