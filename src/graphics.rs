use sdl2;

pub struct Graphics {
    event_pump: sdl2::EventPump,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
}

pub enum Event {
    Quit,
}

impl Graphics {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Graphics, String> {
        let sdl_context = match sdl2::init() {
            Ok(sdl_context) => sdl_context,
            Err(e) => return Err(e),
        };

        let video_subsystem = match sdl_context.video() {
            Ok(video_subsystem) => video_subsystem,
            Err(e) => return Err(e),
        };

        let window = match video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
        {
            Ok(window) => window,
            Err(e) => return Err(e.to_string()),
        };

        let canvas = match window.into_canvas().build() {
            Ok(canvas) => canvas,
            Err(e) => return Err(e.to_string()),
        };

        let event_pump = match sdl_context.event_pump() {
            Ok(event_pump) => event_pump,
            Err(e) => return Err(e),
        };

        let texture_creator = canvas.texture_creator();

        Ok(Graphics {
            event_pump,
            canvas,
            texture_creator,
        })
    }

    pub fn handle_events(&mut self) -> Option<Event> {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => return Some(Event::Quit),
                _ => {}
            }
        }

        None
    }

    pub fn render(&mut self, pixels: &[u8]) -> Result<(), String> {
        self.canvas.clear();

        let mut texture = match self.texture_creator.create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::RGBA8888,
            64,
            32,
        ) {
            Ok(texture) => texture,
            Err(e) => return Err(e.to_string()),
        };

        match texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            for (i, pixel) in pixels.iter().enumerate() {
                buffer[i] = *pixel;
            }
        }) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        match self.canvas.copy(&texture, None, None) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        self.canvas.present();

        Ok(())
    }
}
