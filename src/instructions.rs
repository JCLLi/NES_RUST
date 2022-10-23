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
    pub fn do_instruction(mycpu: &mut MyCpu, ppu: Option<&mut Ppu>) {
        let opcode: u8 = mycpu.cpu.mem[mycpu.cpu.pc as usize];
        let instr = Instruction::get_instruction(opcode);
        mycpu.cycle = instr.cycle;
        println!("Opcode: {:#0x}", opcode); // TODO remove this (or replace with debug)
        match instr.instruction_name {
            InstructionName::BRK => {
                // NOTE some assembles make this instruction 2 bytes, this implementation is only 1
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
                mycpu.cpu.pc = get_irq_addr(mycpu) - 1; // NOTE pc incremented after each instruction
                mycpu.cpu.b = true; // NOTE this has to happen after pushing status register
            }
            InstructionName::LDX => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode);
                mycpu.cpu.x = mycpu.data_read(ppu, addr);
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
                mycpu.cpu.y = mycpu.data_read(ppu, addr);
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
                mycpu.data_write(ppu, addr, mycpu.cpu.x);
            }
            InstructionName::STY => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode);
                mycpu.data_write(ppu, addr, mycpu.cpu.y);
            }
            InstructionName::INX => {
                if mycpu.cpu.x == 0xff {
                    // Handle overflow
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
                    // Handle overflow
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
                mycpu.cpu.pc = get_jump_addr(mycpu, addr) - 1; // NOTE pc incremented after each instruction
            }
            _ => panic!("Instruction not available"),
        }
        mycpu.cpu.pc += 1; // Next instruction
    }
}

pub fn get_jump_addr(mycpu: &mut MyCpu, addr: u16) -> u16 {
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

pub fn get_irq_addr(mycpu: &mut MyCpu) -> u16 {
    ((mycpu.data_read(None, 0xffff) as u16) << 8) | mycpu.data_read(None, 0xfffe) as u16
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
