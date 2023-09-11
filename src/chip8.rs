pub struct Chip8 {
    opcode: u16,          // current opcode
    memory: [u8; 4096],   // Chip-8 has 4K of memory
    registers: [u8; 16],  // 16 general purpose 8-bit registers, usually referred to as Vx
    index: u16,           // register used for memory addresses
    program_counter: u16, // register used for program counter

    // Timers:
    delay_timer: u8,
    sound_timer: u8,

    // Stack and Stack pointer:
    stack: [u16; 16],
    stack_pointer: u16,

    // Graphics:
    // Chip-8 has 64 x 32 resolution, pixels are either on (1) or off (0)
    graphics: [u8; 64 * 32],

    // Chip-8 keypad has 16 keys
    keys: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            opcode: 0,
            memory: [0; 4096],
            registers: [0; 16],
            index: 0,
            program_counter: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            stack_pointer: 0,
            graphics: [0; 64 * 32],
            keys: [0; 16],
        }
    }
}
