use crate::renderer::Renderer;
use crate::Input;
use super::{button::ClickElement, Element, Panel, PADDING};

pub struct ToggleGroup {
    toggles: Vec<ClickElement>,
    on_toggle: Option<String>,
}

impl ToggleGroup {
    pub fn new(toggles: Vec<ClickElement>) -> Self {
        Self { 
            toggles,
            on_toggle: None,
        }
    }
    
    pub fn active_toggle(&self) -> Option<String> {
        self.on_toggle.clone()
    }
}

impl Element for ToggleGroup {
    fn render(&mut self, renderer: &mut Renderer, input: &Input, panel: &mut Panel, delta_time: f64) {
        for toggle in self.toggles.iter_mut() {
            panel.display(renderer, input, delta_time, toggle);
        }

        let mut on = usize::MAX;
        let mut any_on = false;

        for i in 0..self.toggles.len() {
            let toggle = &mut self.toggles[i];
            if toggle.on() {
                any_on = true;
            }
            if toggle.just_clicked() {
                on = i;
                break;
            }
        }
        
        if !any_on {
            self.on_toggle = None;
        }

        if on != usize::MAX {
            for i in 0..self.toggles.len() {
                if i != on {
                    self.toggles[i].force_off();
                }
            }

            self.on_toggle = Some(self.toggles[on].text.clone());
        }
    }

    fn bounds(&mut self) -> (i32, i32) {
        (-(PADDING as i32 / 2), i32::MAX)
    }
}
