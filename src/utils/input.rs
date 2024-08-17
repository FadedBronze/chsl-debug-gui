use std::collections::HashSet;
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, EventPump};

pub struct Input {
    events: EventPump,
    
    held_keys: HashSet<Keycode>,
    just_pressed_keys: HashSet<Keycode>,
    just_released_keys: HashSet<Keycode>,
    just_pressed_text: String,
    
    just_pressed_mouse: HashSet<MouseButton>,
    just_released_mouse: HashSet<MouseButton>,
    held_mouse: HashSet<MouseButton>, 
    mouse_x: i32,
    mouse_y: i32,

    close: bool,
}

impl Input {
    pub fn update(&mut self) {
        self.just_pressed_keys.clear();
        self.just_pressed_mouse.clear();
        self.just_released_keys.clear();
        self.just_released_mouse.clear();
        self.just_pressed_text = String::new();

        for e in self.events.poll_iter() {
            match e {
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    self.just_pressed_mouse.insert(mouse_btn);
                    self.held_mouse.insert(mouse_btn);
                    self.mouse_x = x;
                    self.mouse_y = y;
                }
                Event::TextInput { text, .. } => {
                    self.just_pressed_text = text;
                }
                Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    self.held_mouse.remove(&mouse_btn);
                    self.just_released_mouse.insert(mouse_btn);
                    self.mouse_x = x;
                    self.mouse_y = y;
                }
                Event::MouseMotion { x, y, .. } => {
                    self.mouse_x = x;
                    self.mouse_y = y;
                }
                Event::KeyDown { keycode, .. } => {
                    self.just_pressed_keys.insert(keycode.unwrap());
                    self.held_keys.insert(keycode.unwrap());
                }
                Event::KeyUp { keycode, .. } => {
                    self.held_keys.remove(&keycode.unwrap());
                    self.just_released_keys.insert(keycode.unwrap());
                }
                Event::Quit { .. } => {
                    self.close = true;
                }
                _ => {}
            }
        };
    }
    
    pub fn just_pressed(&self, key: &Keycode) -> bool {
        self.just_pressed_keys.contains(key)
    }

    pub fn just_pressed_text(&self) -> String {
        self.just_pressed_text.clone()
    }
    
    pub fn just_released(&self, key: &Keycode) -> bool {
        self.just_released_keys.contains(key)
    }
    
    pub fn held(&self, key: &Keycode) -> bool {
        self.held_keys.contains(key)
    }

    pub fn just_pressed_mouse(&self, button: &MouseButton) -> bool {
        self.just_pressed_mouse.contains(button)
    }

    pub fn just_released_mouse(&self, button: &MouseButton) -> bool {
        self.just_released_mouse.contains(button)
    }
    
    pub fn held_mouse(&self, button: &MouseButton) -> bool {
        self.held_mouse.contains(button)
    }

    pub fn get_mouse_pos(&self) -> (i32, i32) {
        (self.mouse_x, self.mouse_y)
    }

    pub fn close_button(&self) -> bool {
        self.close
    }

    pub fn new(event_pump: EventPump) -> Self {
        Self {
            held_keys: HashSet::new(),
            just_pressed_keys: HashSet::new(),
            just_pressed_mouse: HashSet::new(),
            just_released_keys: HashSet::new(),
            just_released_mouse: HashSet::new(),
            just_pressed_text: String::new(),
            mouse_y: 0,
            mouse_x: 0,
            close: false,
            held_mouse: HashSet::new(),
            events: event_pump,
        }
    }
}
