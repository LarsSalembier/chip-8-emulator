extern crate sdl2;

mod cpu;
mod graphics;
mod keyboard;
mod memory;
mod opcode;
mod registers;
mod screen;
mod stack;

pub fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("Usage: cargo run <filename>");

    let mut graphics = match graphics::Graphics::new("Chip-8", 1024, 512) {
        Ok(graphics) => graphics,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let mut cpu = cpu::Cpu::new();

    cpu.load_rom(&filename).expect("Failed to load ROM");

    'running: loop {
        cpu.emulate_cycle().expect("Failed to emulate cycle");

        if let Some(event) = graphics.handle_events() {
            match event {
                graphics::Event::Quit => break 'running,
            }
        }

        let pixels = cpu.screen.get_pixels();

        graphics.render(&pixels).expect("Failed to render");
    }
}
