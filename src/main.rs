extern crate sdl2;

mod cpu;
mod memory;
mod opcode;

pub fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("Usage: cargo run <filename>");

    let sdl_context = sdl2::init().expect("Failed to initialize SDL");

    let video_subsystem = sdl_context
        .video()
        .expect("Failed to initialize video subsystem");

    let window = video_subsystem
        .window("CHIP-8", 1024, 512)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to create canvas");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event pump");

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGBA8888, 64, 32)
        .expect("Failed to create texture");

    let mut cpu = cpu::Cpu::new();

    cpu.load_rom(&filename).expect("Failed to load ROM");

    'running: loop {
        cpu.emulate_cycle().expect("Failed to emulate cycle");

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();

        let pixels = cpu.graphics.iter().flat_map(|&pixel| {
            if pixel == 0 {
                vec![0, 0, 0, 0]
            } else {
                vec![255, 255, 255, 255]
            }
        });

        texture
            .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                for (i, pixel) in pixels.enumerate() {
                    buffer[i] = pixel;
                }
            })
            .expect("Failed to lock texture");

        canvas
            .copy(&texture, None, None)
            .expect("Failed to copy texture to canvas");

        canvas.present();
    }
}
