pub enum Opcode {
    ClearScreen,
    ReturnFromSubroutine,
    JumpToAddr { addr: u16 },
    CallAddr { addr: u16 },
    SkipIfEqual { vx: u8, byte: u8 },
    SkipIfNotEqual { vx: u8, byte: u8 },
    SkipIfVxEqualVy { vx: u8, vy: u8 },
    SetVxToByte { vx: u8, byte: u8 },
    AddByteToVx { vx: u8, byte: u8 },
    SetVxToVy { vx: u8, vy: u8 },
    SetVxToVxOrVy { vx: u8, vy: u8 },
    SetVxToVxAndVy { vx: u8, vy: u8 },
    SetVxToVxXorVy { vx: u8, vy: u8 },
    AddVyToVx { vx: u8, vy: u8 },
    SubtractVyFromVx { vx: u8, vy: u8 },
    ShiftVxRight { vx: u8 },
    SetVxToVyMinusVx { vx: u8, vy: u8 },
    ShiftVxLeft { vx: u8 },
    SkipIfVxNotEqualVy { vx: u8, vy: u8 },
    SetIndexToAddr { addr: u16 },
    JumpToAddrPlusV0 { addr: u16 },
    SetVxToRandAndByte { vx: u8, byte: u8 },
    DrawSprite { vx: u8, vy: u8, n: u8 },
    SkipIfKeyPressed { vx: u8 },
    SkipIfKeyNotPressed { vx: u8 },
    SetVxToDelayTimer { vx: u8 },
    WaitForKeyPress { vx: u8 },
    SetDelayTimerToVx { vx: u8 },
    SetSoundTimerToVx { vx: u8 },
    AddVxToIndex { vx: u8 },
    SetIndexToSpriteLocation { vx: u8 },
    StoreBCD { vx: u8 },
    StoreRegisters { vx: u8 },
    LoadRegisters { vx: u8 },
}

impl Opcode {
    pub fn decode(opcode: u16) -> Option<Opcode> {
        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                0x00E0 => Some(Opcode::ClearScreen),
                0x00EE => Some(Opcode::ReturnFromSubroutine),
                _ => None,
            },
            0x1000 => Some(Opcode::JumpToAddr {
                addr: opcode & 0x0FFF,
            }),
            0x2000 => Some(Opcode::CallAddr {
                addr: opcode & 0x0FFF,
            }),
            0x3000 => Some(Opcode::SkipIfEqual {
                vx: ((opcode & 0x0F00) >> 8) as u8,
                byte: (opcode & 0x00FF) as u8,
            }),
            0x4000 => Some(Opcode::SkipIfNotEqual {
                vx: ((opcode & 0x0F00) >> 8) as u8,
                byte: (opcode & 0x00FF) as u8,
            }),
            0x5000 => Some(Opcode::SkipIfVxEqualVy {
                vx: ((opcode & 0x0F00) >> 8) as u8,
                vy: ((opcode & 0x00F0) >> 4) as u8,
            }),
            0x6000 => Some(Opcode::SetVxToByte {
                vx: ((opcode & 0x0F00) >> 8) as u8,
                byte: (opcode & 0x00FF) as u8,
            }),
            0x7000 => Some(Opcode::AddByteToVx {
                vx: ((opcode & 0x0F00) >> 8) as u8,
                byte: (opcode & 0x00FF) as u8,
            }),
            0x8000 => {
                let x = (opcode & 0x0F00) >> 8;
                let y = (opcode & 0x00F0) >> 4;
                let mode = opcode & 0x000F;

                match mode {
                    0x0000 => Some(Opcode::SetVxToVy {
                        vx: x as u8,
                        vy: y as u8,
                    }),
                    0x0001 => Some(Opcode::SetVxToVxOrVy {
                        vx: x as u8,
                        vy: y as u8,
                    }),
                    0x0002 => Some(Opcode::SetVxToVxAndVy {
                        vx: x as u8,
                        vy: y as u8,
                    }),
                    0x0003 => Some(Opcode::SetVxToVxXorVy {
                        vx: x as u8,
                        vy: y as u8,
                    }),
                    0x0004 => Some(Opcode::AddVyToVx {
                        vx: x as u8,
                        vy: y as u8,
                    }),
                    0x0005 => Some(Opcode::SubtractVyFromVx {
                        vx: x as u8,
                        vy: y as u8,
                    }),
                    0x0006 => Some(Opcode::ShiftVxRight { vx: x as u8 }),
                    0x0007 => Some(Opcode::SetVxToVyMinusVx {
                        vx: x as u8,
                        vy: y as u8,
                    }),
                    0x000E => Some(Opcode::ShiftVxLeft { vx: x as u8 }),
                    _ => None,
                }
            }
            0x9000 => Some(Opcode::SkipIfVxNotEqualVy {
                vx: ((opcode & 0x0F00) >> 8) as u8,
                vy: ((opcode & 0x00F0) >> 4) as u8,
            }),
            0xA000 => Some(Opcode::SetIndexToAddr {
                addr: opcode & 0x0FFF,
            }),
            0xB000 => Some(Opcode::JumpToAddrPlusV0 {
                addr: opcode & 0x0FFF,
            }),
            0xC000 => Some(Opcode::SetVxToRandAndByte {
                vx: ((opcode & 0x0F00) >> 8) as u8,
                byte: (opcode & 0x00FF) as u8,
            }),
            0xD000 => Some(Opcode::DrawSprite {
                vx: ((opcode & 0x0F00) >> 8) as u8,
                vy: ((opcode & 0x00F0) >> 4) as u8,
                n: (opcode & 0x000F) as u8,
            }),
            0xE000 => match opcode & 0x00FF {
                0x009E => Some(Opcode::SkipIfKeyPressed {
                    vx: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x00A1 => Some(Opcode::SkipIfKeyNotPressed {
                    vx: ((opcode & 0x0F00) >> 8) as u8,
                }),
                _ => None,
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => Some(Opcode::SetVxToDelayTimer {
                    vx: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x000A => Some(Opcode::WaitForKeyPress {
                    vx: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0015 => Some(Opcode::SetDelayTimerToVx {
                    vx: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0018 => Some(Opcode::SetSoundTimerToVx {
                    vx: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x001E => Some(Opcode::AddVxToIndex {
                    vx: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0029 => Some(Opcode::SetIndexToSpriteLocation {
                    vx: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0033 => Some(Opcode::StoreBCD {
                    vx: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0055 => Some(Opcode::StoreRegisters {
                    vx: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0065 => Some(Opcode::LoadRegisters {
                    vx: ((opcode & 0x0F00) >> 8) as u8,
                }),
                _ => None,
            },
            _ => None,
        }
    }
}
