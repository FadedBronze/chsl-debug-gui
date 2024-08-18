use std::{f64, u32};

use sdl2::{keyboard::Keycode, mouse::MouseButton, rect::Rect};

use crate::{renderer::Renderer, utils::input::Input};

use super::{Element, Panel, PADDING};

pub struct TextInput {
    content: String,
    placeholder: String,
    counter: f64,
    focused: bool,
    updated: bool,
}

impl TextInput {
    pub fn new(content: &str, placeholder: &str) -> Self {
        Self {
            content: content.to_string(),
            placeholder: placeholder.to_string(),
            counter: 0.0,
            focused: false,
            updated: false,
        }
    }
    pub fn get_value(&self) -> String {
        self.content.clone()
    }
    pub fn set_value(&mut self, value: String) {
        self.content = value;
    }
    pub fn updated(&self) -> bool {
        self.updated
    }
}

impl Element for TextInput {
    fn render(&mut self, renderer: &mut Renderer, input: &Input, panel: &mut Panel, delta_time: f64) {
        self.counter += delta_time;

        let bounds = Rect::new(
            panel.bounds.x as i32 + PADDING as i32 / 2,
            panel.bounds.y as i32 + panel.y_offset,
            panel.bounds.width as u32 - PADDING,
            self.bounds().0 as u32,
        );
            
        if self.focused {
            self.updated = input.just_pressed_text().len() > 0;

            self.content += &input.just_pressed_text();

            if input.just_pressed(&Keycode::BACKSPACE) {
                if self.content.len() > 0 {
                    self.content.pop();
                }
            }
        }
        
        let placeholder = self.content.len() == 0;
        
        let (x, y) = input.get_mouse_pos();

        let hovering = x > bounds.left() && x < bounds.right() && y > bounds.top() && y < bounds.bottom();

        if input.just_pressed_mouse(&MouseButton::Left) {
            self.focused = false;

            if hovering {
                self.focused = true;
            }
        }

        let text_height = (bounds.height() - PADDING / 2) as i32;

        //text texture
        let rendering_text = if placeholder { &self.placeholder } else { &self.content };
        let rendering_color = if placeholder { (205, 205, 205, 205) } else { (255, 255, 255, 255) };

        renderer.set_color(rendering_color.0, rendering_color.1, rendering_color.2, rendering_color.3);
        let text_texture = unsafe {
            renderer.get_text_texture("open_sans", rendering_text, text_height)
        };
        
        //background
        renderer.set_color(70, 70, 70, 255);  
        renderer.fill_rect(bounds.x, bounds.y, bounds.width() as i32, bounds.height() as i32);

        //outline
        if self.focused {
            renderer.set_color(200, 200, 200, 255);  
            renderer.outline_rect(bounds.x, bounds.y, bounds.width() as i32, bounds.height() as i32);
        }

        //text
        renderer.render_text_texture(
            &text_texture, 
            (bounds.x() as f64 + PADDING as f64 / 4.0) as i32,
            (bounds.y() as f64 + PADDING as f64 / 4.0) as i32,
        );

        unsafe {
            renderer.free_last_texture();
        }

        if self.updated {
            self.counter = 0.0;
        }

        let sin01 = ((self.counter * 6.0).sin() + 1.0) * 0.5;

        let gray = (127.5 * sin01 + 127.5) as u8;
        
        //blinky
        if self.focused {
            renderer.set_color(gray, gray, gray, 255);  

            renderer.fill_rect(
                bounds.x() + PADDING as i32 / 4 + if placeholder { 0 } else { text_texture.size.0 } as i32,
                bounds.y() + PADDING as i32 / 4 + 2,
                1,
                text_height as i32 - 4,
            );
        }
    }
    fn bounds(&mut self) -> (i32, i32) {
        (28, i32::MAX)
    }
}
