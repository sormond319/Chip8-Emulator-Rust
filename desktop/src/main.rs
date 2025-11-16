use std::env;
use std::fs::File;
use std::io::Read;
use chip8_core::*;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCALE: u32 = 15;
const WINDOW_W: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_H: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const TICKS_PER_FRAME: usize = 10;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    //Set up SDL2 window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Chip-8", WINDOW_W, WINDOW_H).position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut emulator = Emu::new();

    let mut rom = File::open(&args[1]).expect("Failed to open ROM file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    emulator.load(&buffer);

    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'game_loop;
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    if let Some(idx) = sdl_key_to_chip8(key) {
                        emulator.keypress(idx, true);
                    }
                },
                Event::KeyUp { keycode: Some(key), .. } => {
                    if let Some(idx) = sdl_key_to_chip8(key) {
                        emulator.keypress(idx, false);
                    }
                },
                _ => {},
            }
        }

        for _ in 0..TICKS_PER_FRAME {
            emulator.tick();
        }
        emulator.tick_timers();
        draw_screen(&emulator, &mut canvas);
    }
}

fn draw_screen(emu: &Emu, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = emu.get_display();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for(i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            //Transform array index into x,y coordinates
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;
            //Draw scaled rect at coordinates
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

fn sdl_key_to_chip8(key: sdl2::keyboard::Keycode) -> Option<usize> {
    match key {
        sdl2::keyboard::Keycode::Num1 => Some(0x1),
        sdl2::keyboard::Keycode::Num2 => Some(0x2),
        sdl2::keyboard::Keycode::Num3 => Some(0x3),
        sdl2::keyboard::Keycode::Num4 => Some(0xC),
        sdl2::keyboard::Keycode::Q => Some(0x4),
        sdl2::keyboard::Keycode::W => Some(0x5),
        sdl2::keyboard::Keycode::E => Some(0x6),
        sdl2::keyboard::Keycode::R => Some(0xD),
        sdl2::keyboard::Keycode::A => Some(0x7),
        sdl2::keyboard::Keycode::S => Some(0x8),
        sdl2::keyboard::Keycode::D => Some(0x9),
        sdl2::keyboard::Keycode::F => Some(0xE),
        sdl2::keyboard::Keycode::Z => Some(0xA),
        sdl2::keyboard::Keycode::X => Some(0x0),
        sdl2::keyboard::Keycode::C => Some(0xB),
        sdl2::keyboard::Keycode::V => Some(0xF),
        _ => None,
    }
}