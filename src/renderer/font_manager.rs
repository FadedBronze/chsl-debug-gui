use std::collections::HashMap;
use sdl2::ttf::{Font, Sdl2TtfContext};

pub struct FontManager {
    ttf_context: Sdl2TtfContext,
    fonts: HashMap<&'static str, Font<'static, 'static>>,
}

impl Drop for FontManager {
    fn drop(&mut self) {
        for (_, font) in self.fonts.iter() {
            std::mem::drop(font);
        }
        std::mem::drop(self);
    }
}

impl FontManager {
    unsafe fn load_font(ttf_context: *mut Sdl2TtfContext) -> Font<'static, 'static> {
        (*ttf_context).load_font("src/assets/OpenSans.ttf", 128).unwrap()
    }

    pub fn get_font(&mut self, key: &str) -> &Font<'static, 'static> {
        self.fonts.get(key).unwrap()
    }

    pub fn new() -> FontManager {
        let mut ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let open_sans = unsafe {
            FontManager::load_font(&mut ttf_context)
        };

        FontManager {
            fonts: HashMap::from([("open_sans", open_sans)]),
            ttf_context,
        }
    }
}
