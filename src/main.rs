mod gui;
mod ui;
mod utils;
mod renderer;

use utils::input::Input;
use renderer::Renderer;
use ui::AppUI;
use utils::TimerUtil;
use std::{f64, time::{SystemTime, UNIX_EPOCH}};

pub fn loop_with_dt<F: FnMut(f64) -> bool>(mut tick: F) {
    let mut last_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    loop { 
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let dt = now - last_time;
        last_time = now;

        if !tick(dt.as_secs_f64()) {
            return;
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();  

    let mut input = Input::new(sdl_context.event_pump().unwrap());

    let mut physics_world = PhysicsWorld::new(BoundingBox {
        x: 0.0,
        y: 0.0,
        width: 1240.0,
        height: 880.0,
    });
    
    let mut ui = AppUI::new();

    let mut renderer = Renderer::new();

    let mut timer_util = TimerUtil::new();

    loop_with_dt(| delta_time | {
        renderer.clear(255, 255, 255, 255);

        if input.close_button() {
            return false;
        }

        input.update();

        timer_util.start("UI");
        ui.render(&mut physics_world, &mut renderer, &input, delta_time);
        timer_util.stop_log_secs("UI");
        
        timer_util.start("physics");
        physics_world.update(delta_time, 1);
        timer_util.stop_log_secs("physics");

        physics_world.debug_render(&mut renderer);
       
        renderer.update();

        true
    })
}
