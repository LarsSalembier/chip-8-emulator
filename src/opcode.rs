pub enum Opcode {
    ClearScreen,
    ReturnFromSubroutine,
    JumpToAddress {
        address: u16,
    },
    CallAddress {
        address: u16,
    },
    SkipIfEqual {
        register: u8,
        byte: u8,
    },
    SkipIfNotEqual {
        register: u8,
        byte: u8,
    },
    SkipIfRegistersEqual {
        register1: u8,
        register2: u8,
    },
    SetRegisterToByte {
        register: u8,
        byte: u8,
    },
    AddByteToRegister {
        register: u8,
        byte: u8,
    },
    SetRegisterToRegister {
        register1: u8,
        register2: u8,
    },
    SetRegisterToRegisterOrRegister {
        register1: u8,
        register2: u8,
    },
    SetRegisterToRegisterAndRegister {
        register1: u8,
        register2: u8,
    },
    SetRegisterToRegisterXorRegister {
        register1: u8,
        register2: u8,
    },
    AddRegisterToRegister {
        register1: u8,
        register2: u8,
    },
    SubtractRegisterFromRegister {
        register1: u8,
        register2: u8,
    },
    ShiftRegisterRight {
        register: u8,
    },
    SetRegisterToRegisterMinusRegister {
        register1: u8,
        register2: u8,
    },
    ShiftRegisterLeft {
        register: u8,
    },
    SkipIfRegisterNotEqualRegister {
        register1: u8,
        register2: u8,
    },
    SetIndexToAddress {
        address: u16,
    },
    JumpToAddressPlusRegister0 {
        address: u16,
    },
    SetRegisterToRandAndByte {
        register: u8,
        byte: u8,
    },
    DrawSprite {
        register1: u8,
        register2: u8,
        size: u8,
    },
    SkipIfKeyPressed {
        key: u8,
    },
    SkipIfKeyNotPressed {
        key: u8,
    },
    SetRegisterToDelayTimer {
        register: u8,
    },
    WaitForKeyPress {
        key: u8,
    },
    SetDelayTimerToRegister {
        register: u8,
    },
    SetSoundTimerToRegister {
        register: u8,
    },
    AddRegisterToIndex {
        register: u8,
    },
    SetIndexToSpriteLocation {
        register: u8,
    },
    StoreBCD {
        register: u8,
    },
    StoreRegisters {
        last_index: u8,
    },
    LoadRegisters {
        last_memory_address: u8,
    },
}

impl Opcode {
    pub fn decode(opcode: u16) -> Option<Opcode> {
        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                0x00E0 => Some(Opcode::ClearScreen),
                0x00EE => Some(Opcode::ReturnFromSubroutine),
                _ => None,
            },
            0x1000 => Some(Opcode::JumpToAddress {
                address: opcode & 0x0FFF,
            }),
            0x2000 => Some(Opcode::CallAddress {
                address: opcode & 0x0FFF,
            }),
            0x3000 => Some(Opcode::SkipIfEqual {
                register: ((opcode & 0x0F00) >> 8) as u8,
                byte: (opcode & 0x00FF) as u8,
            }),
            0x4000 => Some(Opcode::SkipIfNotEqual {
                register: ((opcode & 0x0F00) >> 8) as u8,
                byte: (opcode & 0x00FF) as u8,
            }),
            0x5000 => Some(Opcode::SkipIfRegistersEqual {
                register1: ((opcode & 0x0F00) >> 8) as u8,
                register2: ((opcode & 0x00F0) >> 4) as u8,
            }),
            0x6000 => Some(Opcode::SetRegisterToByte {
                register: ((opcode & 0x0F00) >> 8) as u8,
                byte: (opcode & 0x00FF) as u8,
            }),
            0x7000 => Some(Opcode::AddByteToRegister {
                register: ((opcode & 0x0F00) >> 8) as u8,
                byte: (opcode & 0x00FF) as u8,
            }),
            0x8000 => {
                let x = (opcode & 0x0F00) >> 8;
                let y = (opcode & 0x00F0) >> 4;
                let mode = opcode & 0x000F;

                match mode {
                    0x0000 => Some(Opcode::SetRegisterToRegister {
                        register1: x as u8,
                        register2: y as u8,
                    }),
                    0x0001 => Some(Opcode::SetRegisterToRegisterOrRegister {
                        register1: x as u8,
                        register2: y as u8,
                    }),
                    0x0002 => Some(Opcode::SetRegisterToRegisterAndRegister {
                        register1: x as u8,
                        register2: y as u8,
                    }),
                    0x0003 => Some(Opcode::SetRegisterToRegisterXorRegister {
                        register1: x as u8,
                        register2: y as u8,
                    }),
                    0x0004 => Some(Opcode::AddRegisterToRegister {
                        register1: x as u8,
                        register2: y as u8,
                    }),
                    0x0005 => Some(Opcode::SubtractRegisterFromRegister {
                        register1: x as u8,
                        register2: y as u8,
                    }),
                    0x0006 => Some(Opcode::ShiftRegisterRight { register: x as u8 }),
                    0x0007 => Some(Opcode::SetRegisterToRegisterMinusRegister {
                        register1: x as u8,
                        register2: y as u8,
                    }),
                    0x000E => Some(Opcode::ShiftRegisterLeft { register: x as u8 }),
                    _ => None,
                }
            }
            0x9000 => Some(Opcode::SkipIfRegisterNotEqualRegister {
                register1: ((opcode & 0x0F00) >> 8) as u8,
                register2: ((opcode & 0x00F0) >> 4) as u8,
            }),
            0xA000 => Some(Opcode::SetIndexToAddress {
                address: opcode & 0x0FFF,
            }),
            0xB000 => Some(Opcode::JumpToAddressPlusRegister0 {
                address: opcode & 0x0FFF,
            }),
            0xC000 => Some(Opcode::SetRegisterToRandAndByte {
                register: ((opcode & 0x0F00) >> 8) as u8,
                byte: (opcode & 0x00FF) as u8,
            }),
            0xD000 => Some(Opcode::DrawSprite {
                register1: ((opcode & 0x0F00) >> 8) as u8,
                register2: ((opcode & 0x00F0) >> 4) as u8,
                size: (opcode & 0x000F) as u8,
            }),
            0xE000 => match opcode & 0x00FF {
                0x009E => Some(Opcode::SkipIfKeyPressed {
                    key: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x00A1 => Some(Opcode::SkipIfKeyNotPressed {
                    key: ((opcode & 0x0F00) >> 8) as u8,
                }),
                _ => None,
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => Some(Opcode::SetRegisterToDelayTimer {
                    register: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x000A => Some(Opcode::WaitForKeyPress {
                    key: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0015 => Some(Opcode::SetDelayTimerToRegister {
                    register: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0018 => Some(Opcode::SetSoundTimerToRegister {
                    register: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x001E => Some(Opcode::AddRegisterToIndex {
                    register: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0029 => Some(Opcode::SetIndexToSpriteLocation {
                    register: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0033 => Some(Opcode::StoreBCD {
                    register: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0055 => Some(Opcode::StoreRegisters {
                    last_index: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0065 => Some(Opcode::LoadRegisters {
                    last_memory_address: ((opcode & 0x0F00) >> 8) as u8,
                }),
                _ => None,
            },
            _ => None,
        }
    }
}
