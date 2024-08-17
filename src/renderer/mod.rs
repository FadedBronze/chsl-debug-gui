mod font_manager;

use std::{f64, u8};

use font_manager::FontManager;
use sdl2::{pixels::Color, rect::{Point, Rect}, render::{Canvas, Texture, TextureCreator, TextureQuery}, video::{Window, WindowContext}};

pub struct Renderer {
    canvas: Canvas<Window>,
    pub size: (u32, u32),
    texture_creator: TextureCreator<WindowContext>,
    font_manager: FontManager,
    textures: Vec<Texture>,
    color: Color,
}

pub struct TextTexure {
    pub size: (u32, u32),
    pub texture: usize,
}

impl Renderer {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
       
        // canvas
        let video_subsystem = sdl_context.video().unwrap();
        
        let window = video_subsystem
            .window("SDL2", 1240, 880)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        let size = window.size();
        
        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
       
        // text
        let texture_creator = canvas.texture_creator();

        Self {
            texture_creator,
            font_manager: FontManager::new(),
            textures: vec![],
            canvas,
            color: Color::RGBA(255, 255, 255, 255),
            size,
        }
    }

    pub fn set_color(&mut self, r: u8, b: u8, g: u8, a: u8) {
        self.color = Color::RGBA(r, g, b, a);
        self.canvas.set_draw_color(self.color);
    }
    
    pub fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.canvas.fill_rect(Rect::new(x, y, width as u32, height as u32)).unwrap();
    }

    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.canvas.draw_line(Point::new(x1, y1), Point::new(x2, y2)).unwrap();
    }
    
    pub fn outline_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.canvas.draw_rect(Rect::new(x, y, width as u32, height as u32)).unwrap();
    }
    
    pub fn clear(&mut self, r: u8, b: u8, g: u8, a: u8) {
        self.canvas.set_draw_color(Color::RGBA(r, g, b, a));
        let rect = Rect::new(0, 0, self.size.0, self.size.1);
        self.canvas.fill_rect(rect).unwrap();
    }

    pub unsafe fn get_text_texture(&mut self, font: &str, text: &String, line_height: i32) -> TextTexure {
        let font_handle = self.font_manager.get_font(font);

        let surface = font_handle
            .render(text)
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string()).unwrap();

        let texture = self.texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string()).unwrap();

        let TextureQuery { width, height, .. } = texture.query();

        let aspect = width as f64 / height as f64;

        self.textures.push(texture);

        TextTexure {
            texture: self.textures.len() - 1,
            size: ((line_height as f64 * aspect) as u32, line_height as u32),
        }
    }

    pub fn render_text_texture(&mut self, text_texture: &TextTexure, x: i32, y: i32) {
        self.canvas.copy(&self.textures[text_texture.texture], None, Rect::new(x, y, text_texture.size.0, text_texture.size.1)).unwrap();
    }

    //indexing is used rn so yeah
    pub unsafe fn free_last_texture(&mut self) {
        self.textures.pop().unwrap().destroy()
    }

    pub fn update(&mut self) {
        self.canvas.present();
    }
}
