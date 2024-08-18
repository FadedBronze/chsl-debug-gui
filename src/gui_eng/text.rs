use std::i32;

use crate::{renderer::Renderer, utils::input::Input};

use super::{Element, Panel, PADDING};

const SIZES: [u8; 4] = [
    16,
    20,
    24,
    28,
];

pub struct Text {
    size: usize,
    pub content: String,
}

impl Text {
    pub fn new(size: usize, content: &str) -> Self {
        Self {
            size,
            content: content.to_string()
        }
    }
}

impl Element for Text {
    fn bounds(&mut self) -> (i32, i32) {
        (Into::<i32>::into(SIZES[self.size]) * 3/5, i32::MAX)
    }
    fn render(&mut self, renderer: &mut Renderer, _input: &Input, panel: &mut Panel, _delta_time: f64) {
        let line_height = SIZES[self.size].into();

        let texture = unsafe {
            renderer.get_text_texture("open_sans", &self.content, line_height)
        };
    
        renderer.render_text_texture(
            &texture, 
            panel.bounds.x as i32 + PADDING as i32 / 2, 
            panel.bounds.y as i32 + panel.y_offset - line_height / 5,
        );     

        unsafe {
            renderer.free_last_texture();
        }
    }
}
