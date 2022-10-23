use crate::mapper::get_mapped_address;
use crate::Cpu6502;
use crate::MyCpu;
use crate::Ppu;
#[allow(clippy::upper_case_acronyms)] // 6502 uses upper case acronyms so we do too
pub enum InstructionName {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,

    CMP,
    CPX,
    CPY,

    DEC,
    DEX,
    DEY,

    INC,
    INX,
    INY,

    JMP,

    LDA,
    LDX,
    LDY,

    RTI,

    SEC,
    SED,
    SEI,

    STA,
    STX,
    STY,
}

pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Indirect,
    IndirectX,
    IndirectY,
}

pub struct Instruction {
    instruction_name: InstructionName,
    addressing_mode: AddressingMode,
    cycle: u8,
}
impl Instruction {
    pub fn get_instruction(opcode: u8) -> Instruction {
        match opcode {
            // ADC
            0x69 => Instruction {
                instruction_name: InstructionName::ADC,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0x65 => Instruction {
                instruction_name: InstructionName::ADC,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0x75 => Instruction {
                instruction_name: InstructionName::ADC,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 4,
            },
            0x6d => Instruction {
                instruction_name: InstructionName::ADC,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            0x7d => Instruction {
                instruction_name: InstructionName::ADC,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 4,
            },
            0x79 => Instruction {
                instruction_name: InstructionName::ADC,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 4, // NOTE this is variable on page crossed
            },
            0x61 => Instruction {
                instruction_name: InstructionName::ADC,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 6,
            },
            0x71 => Instruction {
                instruction_name: InstructionName::ADC,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 5,
            },
            // AND
            0x29 => Instruction {
                instruction_name: InstructionName::AND,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0x25 => Instruction {
                instruction_name: InstructionName::AND,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0x35 => Instruction {
                instruction_name: InstructionName::AND,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 4,
            },
            0x2d => Instruction {
                instruction_name: InstructionName::AND,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            0x3d => Instruction {
                instruction_name: InstructionName::AND,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 4, // NOTE this is variable on page crossed
            },
            0x39 => Instruction {
                instruction_name: InstructionName::AND,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 4, // NOTE this is variable on page crossed
            },
            0x21 => Instruction {
                instruction_name: InstructionName::AND,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 6,
            },
            0x31 => Instruction {
                instruction_name: InstructionName::AND,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 6,
            },
            // ASL
            0x0a => Instruction {
                instruction_name: InstructionName::ASL,
                addressing_mode: AddressingMode::Accumulator,
                cycle: 2,
            },
            0x06 => Instruction {
                instruction_name: InstructionName::ASL,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 5,
            },
            0x16 => Instruction {
                instruction_name: InstructionName::ASL,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 6,
            },
            0x0e => Instruction {
                instruction_name: InstructionName::ASL,
                addressing_mode: AddressingMode::Absolute,
                cycle: 6,
            },
            0x1e => Instruction {
                instruction_name: InstructionName::ASL,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 7,
            },
            // BCC
            0x90 => Instruction {
                instruction_name: InstructionName::BCC,
                addressing_mode: AddressingMode::Relative,
                cycle: 2, // NOTE this is variable on branch success or new page
            },
            // BCS
            0xb0 => Instruction {
                instruction_name: InstructionName::BCS,
                addressing_mode: AddressingMode::Relative,
                cycle: 2, // NOTE this is variable on branch success or new page
            },
            // BEQ
            0xf0 => Instruction {
                instruction_name: InstructionName::BEQ,
                addressing_mode: AddressingMode::Relative,
                cycle: 2, // NOTE this is variable on branch success or new page
            },
            // BIT
            0x24 => Instruction {
                instruction_name: InstructionName::BIT,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0x2c => Instruction {
                instruction_name: InstructionName::BIT,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            // BMI
            0x30 => Instruction {
                instruction_name: InstructionName::BMI,
                addressing_mode: AddressingMode::Relative,
                cycle: 2, // NOTE this is variable on branch success or new page
            },
            // BNE
            0xd0 => Instruction {
                instruction_name: InstructionName::BNE,
                addressing_mode: AddressingMode::Relative,
                cycle: 2, // NOTE this is variable on branch success or new page
            },
            // BPL
            0x10 => Instruction {
                instruction_name: InstructionName::BPL,
                addressing_mode: AddressingMode::Relative,
                cycle: 2, // NOTE this is variable on branch success or new page
            },
            // BRK
            0x00 => Instruction {
                instruction_name: InstructionName::BRK,
                addressing_mode: AddressingMode::Implied,
                cycle: 7,
            },
            // BVC
            0x50 => Instruction {
                instruction_name: InstructionName::BVC,
                addressing_mode: AddressingMode::Relative,
                cycle: 2, // NOTE this is variable on branch success or new page
            },
            // BVS
            0x70 => Instruction {
                instruction_name: InstructionName::BVS,
                addressing_mode: AddressingMode::Relative,
                cycle: 2, // NOTE this is variable on branch success or new page
            },
            // CLC
            0x18 => Instruction {
                instruction_name: InstructionName::CLC,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // CLD
            0xd8 => Instruction {
                instruction_name: InstructionName::CLD,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // CLI
            0x58 => Instruction {
                instruction_name: InstructionName::CLI,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // CMP
            0xc9 => Instruction {
                instruction_name: InstructionName::CMP,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0xc5 => Instruction {
                instruction_name: InstructionName::CMP,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0xd5 => Instruction {
                instruction_name: InstructionName::CMP,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 4,
            },
            0xcd => Instruction {
                instruction_name: InstructionName::CMP,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            0xdd => Instruction {
                instruction_name: InstructionName::CMP,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 4,
            },
            0xd9 => Instruction {
                instruction_name: InstructionName::CMP,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 4,
            },
            0xc1 => Instruction {
                instruction_name: InstructionName::CMP,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 6,
            },
            0xd1 => Instruction {
                instruction_name: InstructionName::CMP,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 5,
            },
            // CPX
            0xe0 => Instruction {
                instruction_name: InstructionName::CPX,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0xe4 => Instruction {
                instruction_name: InstructionName::CPX,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0xec => Instruction {
                instruction_name: InstructionName::CPX,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            // CPY
            0xc0 => Instruction {
                instruction_name: InstructionName::CPY,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0xc4 => Instruction {
                instruction_name: InstructionName::CPY,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0xcc => Instruction {
                instruction_name: InstructionName::CPY,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },

            // DEC
            0xc6 => Instruction {
                instruction_name: InstructionName::DEC,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 5,
            },
            0xd6 => Instruction {
                instruction_name: InstructionName::DEC,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 6,
            },
            0xce => Instruction {
                instruction_name: InstructionName::DEC,
                addressing_mode: AddressingMode::Absolute,
                cycle: 6,
            },
            0xde => Instruction {
                instruction_name: InstructionName::DEC,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 7,
            },
            // DEX
            0xca => Instruction {
                instruction_name: InstructionName::DEX,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // DEY
            0x88 => Instruction {
                instruction_name: InstructionName::DEY,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },

            // LDA
            0xa9 => Instruction {
                instruction_name: InstructionName::LDA,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0xa5 => Instruction {
                instruction_name: InstructionName::LDA,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0xb5 => Instruction {
                instruction_name: InstructionName::LDA,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 4,
            },
            0xad => Instruction {
                instruction_name: InstructionName::LDA,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            0xbd => Instruction {
                instruction_name: InstructionName::LDA,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 4, // NOTE this is variable on page crossed
            },
            0xb9 => Instruction {
                instruction_name: InstructionName::LDA,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 4, // NOTE this is variable on page crossed
            },
            0xa1 => Instruction {
                instruction_name: InstructionName::LDA,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 6,
            },
            0xb1 => Instruction {
                instruction_name: InstructionName::LDA,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 5, // NOTE this is variable on page crossed
            },
            // LDX
            0xa2 => Instruction {
                instruction_name: InstructionName::LDX,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0xa6 => Instruction {
                instruction_name: InstructionName::LDX,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0xb6 => Instruction {
                instruction_name: InstructionName::LDX,
                addressing_mode: AddressingMode::ZeroPageY,
                cycle: 4,
            },
            0xae => Instruction {
                instruction_name: InstructionName::LDX,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            0xbe => Instruction {
                instruction_name: InstructionName::LDX,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 4, // NOTE this is variable on page crossed
            },
            // LDY
            0xa0 => Instruction {
                instruction_name: InstructionName::LDY,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0xa4 => Instruction {
                instruction_name: InstructionName::LDY,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0xb4 => Instruction {
                instruction_name: InstructionName::LDY,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 4,
            },
            0xac => Instruction {
                instruction_name: InstructionName::LDY,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            0xbc => Instruction {
                instruction_name: InstructionName::LDY,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 4, // NOTE this is variable on page crossed
            },

            // RTI
            0x40 => Instruction {
                instruction_name: InstructionName::RTI,
                addressing_mode: AddressingMode::Implied,
                cycle: 6,
            },

            // STA
            0x85 => Instruction {
                instruction_name: InstructionName::STA,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0x95 => Instruction {
                instruction_name: InstructionName::STA,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 4,
            },
            0x8d => Instruction {
                instruction_name: InstructionName::STA,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            0x9d => Instruction {
                instruction_name: InstructionName::STA,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 5,
            },
            0x99 => Instruction {
                instruction_name: InstructionName::STA,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 5,
            },
            0x81 => Instruction {
                instruction_name: InstructionName::STA,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 6,
            },
            0x91 => Instruction {
                instruction_name: InstructionName::STA,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 6,
            },

            // STX
            0x86 => Instruction {
                instruction_name: InstructionName::STX,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0x96 => Instruction {
                instruction_name: InstructionName::STX,
                addressing_mode: AddressingMode::ZeroPageY,
                cycle: 4,
            },
            0x8e => Instruction {
                instruction_name: InstructionName::STX,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            // STY
            0x84 => Instruction {
                instruction_name: InstructionName::STY,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0x94 => Instruction {
                instruction_name: InstructionName::STY,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 3,
            },
            0x8c => Instruction {
                instruction_name: InstructionName::STY,
                addressing_mode: AddressingMode::Absolute,
                cycle: 3,
            },

            // INC
            0xe6 => Instruction {
                instruction_name: InstructionName::INC,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 5,
            },
            0xf6 => Instruction {
                instruction_name: InstructionName::INC,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 6,
            },
            0xee => Instruction {
                instruction_name: InstructionName::INC,
                addressing_mode: AddressingMode::Absolute,
                cycle: 6,
            },
            0xfe => Instruction {
                instruction_name: InstructionName::INC,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 7,
            },
            // INX
            0xe8 => Instruction {
                instruction_name: InstructionName::INX,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // INY
            0xc8 => Instruction {
                instruction_name: InstructionName::INY,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // JMP
            0x4c => Instruction {
                instruction_name: InstructionName::JMP,
                addressing_mode: AddressingMode::Absolute,
                cycle: 3,
            },
            0x6c => Instruction {
                instruction_name: InstructionName::JMP,
                addressing_mode: AddressingMode::Indirect,
                cycle: 5,
            },
            // SEC
            0x38 => Instruction {
                instruction_name: InstructionName::SEC,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // SED
            0xf8 => Instruction {
                instruction_name: InstructionName::SED,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // SEI
            0x78 => Instruction {
                instruction_name: InstructionName::SEI,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            _ => panic!("Invalid Opcode: {:08x}", opcode),
        }
    }
    pub fn do_instruction(mycpu: &mut MyCpu, ppu: &mut Ppu) {
        let opcode: u8 = mycpu.cpu.mem[mycpu.cpu.pc as usize];
        let instr = Instruction::get_instruction(opcode);
        mycpu.cycle = instr.cycle;
        println!("Opcode: {:#0x}", opcode); // TODO remove this (or replace with debug)
        match instr.instruction_name {
            InstructionName::BRK => {
                let p = mycpu.cpu.carry as u8 |
                    (mycpu.cpu.zero as u8) << 1 |
                    (mycpu.cpu.irq_dis as u8) << 2 |
                    (mycpu.cpu.dec as u8) << 3 |
                    (mycpu.cpu.b as u8) << 4 |
                    0b0010_0000 | //ignore_flag
                    (mycpu.cpu.overflow as u8) << 6 |
                    (mycpu.cpu.negative as u8) << 7;
                mycpu.cpu.stack_push((mycpu.cpu.pc & 0xff) as u8);
                mycpu.cpu.stack_push(((mycpu.cpu.pc >> 8) & 0xff) as u8);
                mycpu.cpu.stack_push(p);
                mycpu.cpu.pc = 0xFFFE - 1;
            }
            InstructionName::LDX => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode);
                mycpu.cpu.x = mycpu.data_read(Some(ppu), addr);
                // ps
                if mycpu.cpu.x == 0 {
                    mycpu.cpu.zero = true;
                }
                if mycpu.cpu.x & 0b1000_0000 == 0b1000_0000 {
                    mycpu.cpu.negative = true;
                }
            }
            InstructionName::LDY => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode);
                mycpu.cpu.y = mycpu.data_read(Some(ppu), addr);
                // ps
                if mycpu.cpu.y == 0 {
                    mycpu.cpu.zero = true;
                }
                if mycpu.cpu.y & 0b1000_0000 == 0b1000_0000 {
                    mycpu.cpu.negative = true;
                }
            }
            InstructionName::RTI => {
                let p = mycpu.cpu.stack_pop();
                mycpu.cpu.carry = (p & 0b0000_0001) == 0b0000_0001;
                mycpu.cpu.zero = (p & 0b0000_0010) == 0b0000_0010;
                mycpu.cpu.irq_dis = (p & 0b0000_0100) == 0b0000_0100;
                mycpu.cpu.dec = (p & 0b0000_1000) == 0b0000_1000;
                mycpu.cpu.b = (p & 0b0001_0000) == 0b0001_0000;
                mycpu.cpu.overflow = (p & 0b0100_0000) == 0b0100_0000;
                mycpu.cpu.negative = (p & 0b1000_0000) == 0b1000_0000;
                mycpu.cpu.pc = ((mycpu.cpu.stack_pop() as u16) << 8) | mycpu.cpu.stack_pop() as u16;
            }
            InstructionName::STX => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode);
                mycpu.data_write(Some(ppu), addr, mycpu.cpu.x);
            }
            InstructionName::STY => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode);
                mycpu.data_write(Some(ppu), addr, mycpu.cpu.y);
            }
            InstructionName::INX => {
                if mycpu.cpu.x == 0xff {
                    mycpu.cpu.zero = true;
                    mycpu.cpu.x = 0;
                } else {
                    mycpu.cpu.x += 1;
                }
                if mycpu.cpu.x & 0b1000_0000 == 0b1000_0000 {
                    mycpu.cpu.negative = true;
                }
            }
            InstructionName::INY => {
                if mycpu.cpu.y == 0xff {
                    mycpu.cpu.zero = true;
                    mycpu.cpu.y = 0;
                } else {
                    mycpu.cpu.y += 1;
                }
                if mycpu.cpu.y & 0b1000_0000 == 0b1000_0000 {
                    mycpu.cpu.negative = true;
                }
            }
            InstructionName::JMP => {
                // pc
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode);
                mycpu.cpu.pc = memory_jump(mycpu, addr) - 1;
            }
            _ => panic!("Instruction not available"),
        }
    }
}

pub fn memory_jump(mycpu: &mut MyCpu, addr: u16) -> u16 {
    if addr >= 0x8000 {
        get_mapped_address(
            mycpu.cartridge.mapper_number,
            addr,
            mycpu.cartridge.prg_rom_size_in_16kb,
        )
    } else {
        addr
    }
}

pub fn get_data_address(cpu: &mut Cpu6502, address_mode: AddressingMode) -> u16 {
    match address_mode {
        AddressingMode::Implied => cpu.pc,
        AddressingMode::Immediate => {
            cpu.pc += 1;
            cpu.pc
        }
        AddressingMode::Absolute => {
            let ret_addr: u16 = ((cpu.mem[(cpu.pc + 2) as usize]) as u16) << 8
                | cpu.mem[(cpu.pc + 1) as usize] as u16;
            cpu.pc += 2;
            ret_addr
        }
        AddressingMode::AbsoluteX => {
            let ret_addr: u16 = (((cpu.mem[(cpu.pc + 2) as usize]) as u16) << 8
                | (cpu.mem[(cpu.pc + 1) as usize] + cpu.x) as u16)
                as u16;
            cpu.pc += 2;
            ret_addr
        }
        AddressingMode::AbsoluteY => {
            let ret_addr: u16 = (((cpu.mem[(cpu.pc + 2) as usize]) as u16) << 8
                | (cpu.mem[(cpu.pc + 1) as usize] + cpu.y) as u16)
                as u16;
            cpu.pc += 2;
            ret_addr
        }
        AddressingMode::ZeroPage => {
            cpu.pc += 1;
            cpu.mem[cpu.pc as usize] as u16 & 0xFF
        }
        AddressingMode::ZeroPageX => {
            cpu.pc += 1;
            (cpu.mem[cpu.pc as usize] + cpu.x) as u16 & 0xFF
        }
        AddressingMode::ZeroPageY => {
            cpu.pc += 1;
            (cpu.mem[cpu.pc as usize] + cpu.y) as u16 & 0xFF
        }
        AddressingMode::Relative => cpu.pc + (cpu.mem[(cpu.pc + 1) as usize] + 1) as u16,
        AddressingMode::Indirect => {
            let pl_addr: u16 = (cpu.mem[(cpu.pc + 2) as usize] as u16) << 8
                | cpu.mem[(cpu.pc + 1) as usize] as u16;
            let ret_addr: u16 = ((cpu.mem[pl_addr as usize] as u16) << 8)
                | (cpu.mem[((pl_addr + 1) as usize)] as u16);
            cpu.pc += 2;
            ret_addr
        }
        AddressingMode::IndirectX => {
            let ret_addr: u16 = (cpu.mem[cpu.mem[(cpu.pc + 1) as usize] as usize] + cpu.x) as u16;
            cpu.pc += 1;
            ret_addr & 0xFF
        }
        AddressingMode::IndirectY => {
            let ret_addr: u16 = (cpu.mem[cpu.mem[(cpu.pc + 1) as usize] as usize] + cpu.x) as u16;
            cpu.pc += 1;
            ret_addr & 0xFF
        }
        _ => panic!("Addressing mode not implemented yet"),
    }
}

#[cfg(test)]
mod instruction_tests {
    use crate::instructions::Instruction;
    use crate::MyCpu;
    use tudelft_nes_ppu::{Mirroring, Ppu};

    #[test]
    fn test_inx() {
        let mut test_cpu = MyCpu::default();
        let mut ppu = Ppu::new(Mirroring::Vertical);
        test_cpu.cpu.x = 6;
        test_cpu.cpu.mem[0x8000] = 0xe8; // INX Implied
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.x, 7);

        test_cpu.cpu.x = 0xff;
        test_cpu.cpu.mem[0x8000] = 0xe8; // INX Implied
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert!(test_cpu.cpu.zero);

        test_cpu.cpu.x = 0b0111_1111;
        test_cpu.cpu.mem[0x8000] = 0xe8; // INX Implied
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert!(test_cpu.cpu.negative);
    }
    #[test]
    fn test_iny() {
        let mut test_cpu = MyCpu::default();
        let mut ppu = Ppu::new(Mirroring::Vertical);
        test_cpu.cpu.y = 6;
        test_cpu.cpu.mem[0x8000] = 0xc8; // INY Implied
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.y, 7);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.y = 0xff;
        test_cpu.cpu.mem[0x8000] = 0xc8; // INY Implied
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert!(test_cpu.cpu.zero);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.y = 0b0111_1111;
        test_cpu.cpu.mem[0x8000] = 0xc8; // INY Implied
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert!(test_cpu.cpu.negative);
    }

    #[test]
    fn test_jmp() {
        let mut test_cpu = MyCpu::default();
        let mut ppu = Ppu::new(Mirroring::Vertical);
        test_cpu.cpu.mem[0x8000] = 0x4c; // JMP Absolute
        test_cpu.cpu.mem[0x8001] = 0x01;
        test_cpu.cpu.mem[0x8002] = 0x01;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.pc, (0x0101 - 1));

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x1100] = 0x66;
        test_cpu.cpu.mem[0x1101] = 0x67;
        test_cpu.cpu.mem[0x8000] = 0x6c; // JMP Indirect
        test_cpu.cpu.mem[0x8001] = 0x00;
        test_cpu.cpu.mem[0x8002] = 0x11;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.pc, (0x6667 - 1));
    }

    #[test]
    fn test_ldx() {
        let mut test_cpu = MyCpu::default();
        let mut ppu = Ppu::new(Mirroring::Vertical);
        test_cpu.cpu.mem[0x8000] = 0xa2; // LDX Immediate
        test_cpu.cpu.mem[0x8001] = 0x42;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.x, 0x42);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x0001] = 0x43;
        test_cpu.cpu.mem[0x8000] = 0xa6; // LDX Zero Page
        test_cpu.cpu.mem[0x8001] = 0x01;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.x, 0x43);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.y = 0x2;
        test_cpu.cpu.mem[0x0003] = 0x44;
        test_cpu.cpu.mem[0x8000] = 0xb6; // LDX Zero Page Y
        test_cpu.cpu.mem[0x8001] = 0x01;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.x, 0x44);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x0101] = 0x45;
        test_cpu.cpu.mem[0x8000] = 0xae; // LDX Absolute
        test_cpu.cpu.mem[0x8001] = 0x01;
        test_cpu.cpu.mem[0x8002] = 0x01;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.x, 0x45);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.y = 0x3;
        test_cpu.cpu.mem[0x8000] = 0xbe; // LDX Absolute Y
        test_cpu.cpu.mem[0x0105] = 0x46;
        test_cpu.cpu.mem[0x8001] = 0x02;
        test_cpu.cpu.mem[0x8002] = 0x01;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.x, 0x46);
    }
    #[test]
    fn test_ldy() {
        let mut test_cpu = MyCpu::default();
        let mut ppu = Ppu::new(Mirroring::Vertical);
        test_cpu.cpu.mem[0x8000] = 0xa0; // LDY Immediate
        test_cpu.cpu.mem[0x8001] = 0x42;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.y, 0x42);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x0001] = 42;
        test_cpu.cpu.mem[0x8000] = 0xa4; // LDY Zero Page
        test_cpu.cpu.mem[0x8001] = 0x01;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.y, 42);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.x = 0x2;
        test_cpu.cpu.mem[0x0003] = 0x44;
        test_cpu.cpu.mem[0x8000] = 0xb4; // LDY Zero Page X
        test_cpu.cpu.mem[0x8001] = 0x01;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.y, 0x44);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x0101] = 45;
        test_cpu.cpu.mem[0x8000] = 0xac; // LDY Absolute
        test_cpu.cpu.mem[0x8001] = 0x01;
        test_cpu.cpu.mem[0x8002] = 0x01;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.y, 45);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.x = 0x3;
        test_cpu.cpu.mem[0x8000] = 0xbc; // LDY Absolute X
        test_cpu.cpu.mem[0x0105] = 0x46;
        test_cpu.cpu.mem[0x8001] = 0x02;
        test_cpu.cpu.mem[0x8002] = 0x01;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.y, 0x46);
    }
    #[test]
    fn test_rti_brk() {
        let mut test_cpu = MyCpu::default();
        let mut ppu = Ppu::new(Mirroring::Vertical);
        test_cpu.cpu.mem[0x8000] = 0x00; // BRK Implied
        test_cpu.cpu.zero = true; // Set a status flag
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.pc, 0xfffe - 1);
        assert_eq!(test_cpu.cpu.mem[test_cpu.cpu.sp as usize + 3], 0x00);
        assert_eq!(test_cpu.cpu.mem[test_cpu.cpu.sp as usize + 2], 0x80);
        assert_eq!(test_cpu.cpu.mem[test_cpu.cpu.sp as usize + 1], 0x22); // NOTE 6th bit is alwasy set to 1

        test_cpu.cpu.pc += 1;
        test_cpu.cpu.mem[0xfffe] = 0x40; // RTI
        test_cpu.cpu.zero = false; // Set a status flag
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.pc, 0x8000);
        assert_eq!(test_cpu.cpu.pc, 0x8000);
        assert_eq!(test_cpu.cpu.zero, true);
    }
    #[test]
    fn test_stx() {
        let mut test_cpu = MyCpu::default();
        let mut ppu = Ppu::new(Mirroring::Vertical);
        test_cpu.cpu.x = 55;
        test_cpu.cpu.mem[0x8000] = 0x86; // STX Zero Page
        test_cpu.cpu.mem[0x8001] = 0x66;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.mem[0x0066], 55);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.x = 57;
        test_cpu.cpu.y = 0x01;
        test_cpu.cpu.mem[0x8000] = 0x96; // STX Zero Page Y
        test_cpu.cpu.mem[0x8001] = 0x67;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.mem[0x0068], 57);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.x = 58;
        test_cpu.cpu.mem[0x8000] = 0x8e; // STX Absolute
        test_cpu.cpu.mem[0x8001] = 0x67;
        test_cpu.cpu.mem[0x8002] = 0x68;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.mem[0x6867], 58);
    }
    #[test]
    fn test_sty() {
        let mut test_cpu = MyCpu::default();
        let mut ppu = Ppu::new(Mirroring::Vertical);
        test_cpu.cpu.y = 100;
        test_cpu.cpu.mem[0x8000] = 0x84; // STY Zero Page
        test_cpu.cpu.mem[0x8001] = 0x66;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.mem[0x0066], 100);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.y = 101;
        test_cpu.cpu.x = 0x03;
        test_cpu.cpu.mem[0x8000] = 0x94; // STY Zero Page Y
        test_cpu.cpu.mem[0x8001] = 0x67;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.mem[0x006a], 101);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.y = 200;
        test_cpu.cpu.mem[0x8000] = 0x8c; // STY Absolute
        test_cpu.cpu.mem[0x8001] = 0x67;
        test_cpu.cpu.mem[0x8002] = 0x44;
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(test_cpu.cpu.mem[0x4467], 200);
    }
}
