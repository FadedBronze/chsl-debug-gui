use sdl2::mouse::MouseButton;
use crate::renderer::Renderer;

use crate::utils::input::Input;
use super::{Element, Panel, PADDING};

pub enum ButtonOrToggle {
    Button,
    Toggle
}

pub struct ClickElement {
    pub text: String,
    just_clicked: bool,
    state_or_held: bool,
    button_toggle: ButtonOrToggle,
}

impl Element for ClickElement { 
    fn render(&mut self, renderer: &mut Renderer, input: &Input,  panel: &mut Panel, delta_time: f64) {
        self.just_clicked = false;

        let bounds = (
            panel.bounds.x as i32 + PADDING as i32 / 2,
            panel.bounds.y as i32 + panel.y_offset as i32,
            panel.bounds.width as i32 - PADDING as i32,
            self.bounds().0,
        );
 
        let (x, y) = input.get_mouse_pos();

        let hovering = x > bounds.0 && x < bounds.0 + bounds.2 && y > bounds.1 && y < bounds.1 + bounds.3;

        if input.just_pressed_mouse(&MouseButton::Left) && hovering {
            self.just_clicked = true;

            if let ButtonOrToggle::Toggle = self.button_toggle {
                self.state_or_held = !self.state_or_held;
            }
        }

        if let ButtonOrToggle::Button = self.button_toggle {
            self.state_or_held = input.held_mouse(&MouseButton::Left) && hovering;
        }

        if self.state_or_held {
            renderer.set_color(140, 140, 140, 255);
        } else {
            renderer.set_color(120, 120, 120, 255);
        }
        
        let text_height = bounds.3 - PADDING as i32 / 2;
        
        renderer.fill_rect(bounds.0, bounds.1, bounds.2, bounds.3);

        let text_texture = unsafe {
            renderer.get_text_texture("open_sans", &self.text, text_height)
        };

        renderer.render_text_texture(
            &text_texture, 
            bounds.0 + (bounds.2 / 2 - text_texture.size.0 as i32 / 2) as i32, 
            bounds.1 + PADDING as i32 / 4,
        );
        
        unsafe {
            renderer.free_last_texture();
        }
    }

    fn bounds(&mut self) -> (i32, i32) {
        (30, i32::MAX)
    }
}

impl ClickElement {
    pub fn just_clicked(&self) -> bool {
        self.just_clicked
    }

    pub fn force_off(&mut self) {
        self.state_or_held = false
    }

    pub fn on(&self) -> bool {
        self.state_or_held
    }
    
    pub fn held(&self) -> bool {
        self.state_or_held
    }
    
    pub fn new_toggle(text: &str) -> Self {
        Self {
            text: text.to_string(),
            just_clicked: false,
            state_or_held: false,
            button_toggle: ButtonOrToggle::Toggle,
        }
    }

    pub fn new_button(text: &str) -> Self {
        Self {
            text: text.to_string(),
            just_clicked: false,
            state_or_held: false,
            button_toggle: ButtonOrToggle::Button,
        }
    }
}
