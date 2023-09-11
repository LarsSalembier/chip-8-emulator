extern crate sdl2;

pub fn main() {
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
    let texture = texture_creator
        .create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGBA8888, 64, 32)
        .expect("Failed to create texture");

    'running: loop {
        // todo: emulator cycle

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

        // TODO: build a texture from the chip-8 display buffer

        canvas
            .copy(&texture, None, None)
            .expect("Failed to copy texture to canvas");

        canvas.present();
    }
}
