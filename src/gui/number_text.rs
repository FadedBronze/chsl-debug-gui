use std::f64;
use crate::utils::input::Input;
use super::{text_input::TextInput, Element, Panel};

pub struct NumberTextInput {
    text: TextInput,
    current_value: f64,
}

impl NumberTextInput {
    pub fn new(value: f64, placeholder: &str) -> Self {
        Self { text: TextInput::new(value.to_string().as_str(), placeholder), current_value: value}
    }    
    pub fn get_value(&self) -> f64 {
        self.current_value
    }
    pub fn set_value(&mut self, num: f64) {
        self.text.set_value(num.to_string());
    } 
}

impl Element for NumberTextInput {
    fn render(&mut self, renderer: &mut crate::renderer::Renderer, input: &Input, panel: &mut Panel, delta_time: f64) {
        self.text.render(renderer, input, panel, delta_time);
     
        let mut num = self.text.get_value();

        if let Some(last) = num.pop() {
            if last != '.' {
                num.push(last);
            }
        }

        let current_val = num.parse::<f64>();

        if let Ok(num) = current_val {
            self.current_value = num;
        } else {
            self.current_value = 0.0;
            self.text.set_value("".to_string());
        }
    }

    fn bounds(&mut self) -> (i32, i32) {
        self.text.bounds()
    }
}
