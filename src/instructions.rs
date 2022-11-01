use crate::MyCpu;
use crate::{Cpu6502, MapperType};
use tudelft_nes_ppu::{Mirroring, Ppu};

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
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    //Unofficial instructions
    ALR,
    ANC,
    ANE,
    ARR,
    DCP,
    ISC,
    LAS,
    LAX,
    LXA,
    RLA,
    RRA,
    SAX,
    SBX,
    SHA,
    SHX,
    SHY,
    SLO,
    SRE,
    TAS,
    USBC,
    NOPs,
    JAM,
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
            // CLV
            0xb8 => Instruction {
                instruction_name: InstructionName::CLV,
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
            // EOR
            0x49 => Instruction {
                instruction_name: InstructionName::EOR,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0x45 => Instruction {
                instruction_name: InstructionName::EOR,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0x55 => Instruction {
                instruction_name: InstructionName::EOR,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 4,
            },
            0x4D => Instruction {
                instruction_name: InstructionName::EOR,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            0x5D => Instruction {
                instruction_name: InstructionName::EOR,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 4,
            },
            0x59 => Instruction {
                instruction_name: InstructionName::EOR,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 4,
            },
            0x41 => Instruction {
                instruction_name: InstructionName::EOR,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 6,
            },
            0x51 => Instruction {
                instruction_name: InstructionName::EOR,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 5,
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
            // LSR
            0x4a => Instruction {
                instruction_name: InstructionName::LSR,
                addressing_mode: AddressingMode::Accumulator,
                cycle: 2,
            },
            0x46 => Instruction {
                instruction_name: InstructionName::LSR,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 5,
            },
            0x56 => Instruction {
                instruction_name: InstructionName::LSR,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 6,
            },
            0x4e => Instruction {
                instruction_name: InstructionName::LSR,
                addressing_mode: AddressingMode::Absolute,
                cycle: 6,
            },
            0x5e => Instruction {
                instruction_name: InstructionName::LSR,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 7,
            },
            // NOP
            0xea => Instruction {
                instruction_name: InstructionName::NOP,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // ORA
            0x09 => Instruction {
                instruction_name: InstructionName::ORA,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0x05 => Instruction {
                instruction_name: InstructionName::ORA,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0x15 => Instruction {
                instruction_name: InstructionName::ORA,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 4,
            },
            0x0D => Instruction {
                instruction_name: InstructionName::ORA,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            0x1d => Instruction {
                instruction_name: InstructionName::ORA,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 4,
            },
            0x19 => Instruction {
                instruction_name: InstructionName::ORA,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 4,
            },
            0x01 => Instruction {
                instruction_name: InstructionName::ORA,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 6,
            },
            0x11 => Instruction {
                instruction_name: InstructionName::ORA,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 5,
            },
            // PHA
            0x48 => Instruction {
                instruction_name: InstructionName::PHA,
                addressing_mode: AddressingMode::Implied,
                cycle: 3,
            },
            // PHP
            0x08 => Instruction {
                instruction_name: InstructionName::PHP,
                addressing_mode: AddressingMode::Implied,
                cycle: 3,
            },
            // PLA
            0x68 => Instruction {
                instruction_name: InstructionName::PLA,
                addressing_mode: AddressingMode::Implied,
                cycle: 4,
            },
            // PLP
            0x28 => Instruction {
                instruction_name: InstructionName::PLP,
                addressing_mode: AddressingMode::Implied,
                cycle: 4,
            },
            // RTI
            0x40 => Instruction {
                instruction_name: InstructionName::RTI,
                addressing_mode: AddressingMode::Implied,
                cycle: 6,
            },
            // RTS
            0x60 => Instruction {
                instruction_name: InstructionName::RTS,
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
            // JSR
            0x20 => Instruction {
                instruction_name: InstructionName::JSR,
                addressing_mode: AddressingMode::Absolute,
                cycle: 6,
            },
            // ROR
            0x6a => Instruction {
                instruction_name: InstructionName::ROR,
                addressing_mode: AddressingMode::Accumulator,
                cycle: 2,
            },
            0x66 => Instruction {
                instruction_name: InstructionName::ROR,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 5,
            },
            0x76 => Instruction {
                instruction_name: InstructionName::ROR,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 6,
            },
            0x6e => Instruction {
                instruction_name: InstructionName::ROR,
                addressing_mode: AddressingMode::Absolute,
                cycle: 6,
            },
            0x7e => Instruction {
                instruction_name: InstructionName::ROR,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 7,
            },
            // ROL
            0x2a => Instruction {
                instruction_name: InstructionName::ROL,
                addressing_mode: AddressingMode::Accumulator,
                cycle: 2,
            },
            0x26 => Instruction {
                instruction_name: InstructionName::ROL,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 5,
            },
            0x36 => Instruction {
                instruction_name: InstructionName::ROL,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 6,
            },
            0x2e => Instruction {
                instruction_name: InstructionName::ROL,
                addressing_mode: AddressingMode::Absolute,
                cycle: 6,
            },
            0x3e => Instruction {
                instruction_name: InstructionName::ROL,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 7,
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
            // SBC
            0xe9 => Instruction {
                instruction_name: InstructionName::SBC,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0xe5 => Instruction {
                instruction_name: InstructionName::SBC,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0xf5 => Instruction {
                instruction_name: InstructionName::SBC,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 4,
            },
            0xed => Instruction {
                instruction_name: InstructionName::SBC,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            0xfd => Instruction {
                instruction_name: InstructionName::SBC,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 4,
            },
            0xf9 => Instruction {
                instruction_name: InstructionName::SBC,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 4,
            },
            0xe1 => Instruction {
                instruction_name: InstructionName::SBC,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 6,
            },
            0xf1 => Instruction {
                instruction_name: InstructionName::SBC,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 5,
            },
            // TAX
            0xaa => Instruction {
                instruction_name: InstructionName::TAX,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // TAY
            0xa8 => Instruction {
                instruction_name: InstructionName::TAY,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // TSX
            0xba => Instruction {
                instruction_name: InstructionName::TSX,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // TXA
            0x8a => Instruction {
                instruction_name: InstructionName::TXA,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // TXS
            0x9a => Instruction {
                instruction_name: InstructionName::TXS,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // TYA
            0x98 => Instruction {
                instruction_name: InstructionName::TYA,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // Unofficial instructions
            // ALR
            0x4b => Instruction {
                instruction_name: InstructionName::ALR,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            // ANC
            0x0b => Instruction {
                instruction_name: InstructionName::ANC,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0x2b => Instruction {
                instruction_name: InstructionName::ANC,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            // ANE
            0x8b => Instruction {
                instruction_name: InstructionName::ANE,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            // ARR
            0x6b => Instruction {
                instruction_name: InstructionName::ARR,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            // DCP
            0xc7 => Instruction {
                instruction_name: InstructionName::DCP,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 5,
            },
            0xd7 => Instruction {
                instruction_name: InstructionName::DCP,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 6,
            },
            0xcf => Instruction {
                instruction_name: InstructionName::DCP,
                addressing_mode: AddressingMode::Absolute,
                cycle: 6,
            },
            0xdf => Instruction {
                instruction_name: InstructionName::DCP,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 7,
            },
            0xdb => Instruction {
                instruction_name: InstructionName::DCP,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 7,
            },
            0xc3 => Instruction {
                instruction_name: InstructionName::DCP,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 8,
            },
            0xd3 => Instruction {
                instruction_name: InstructionName::DCP,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 8,
            },
            // ISC
            0xe7 => Instruction {
                instruction_name: InstructionName::ISC,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 5,
            },
            0xf7 => Instruction {
                instruction_name: InstructionName::ISC,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 6,
            },
            0xef => Instruction {
                instruction_name: InstructionName::ISC,
                addressing_mode: AddressingMode::Absolute,
                cycle: 6,
            },
            0xff => Instruction {
                instruction_name: InstructionName::ISC,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 7,
            },
            0xfb => Instruction {
                instruction_name: InstructionName::ISC,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 7,
            },
            0xe3 => Instruction {
                instruction_name: InstructionName::ISC,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 8,
            },
            0xf3 => Instruction {
                instruction_name: InstructionName::ISC,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 8,
            },
            // LAS
            0xbb => Instruction {
                instruction_name: InstructionName::LAS,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 4,
            },
            // LAX
            0xa7 => Instruction {
                instruction_name: InstructionName::LAX,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0xb7 => Instruction {
                instruction_name: InstructionName::LAX,
                addressing_mode: AddressingMode::ZeroPageY,
                cycle: 4,
            },
            0xaf => Instruction {
                instruction_name: InstructionName::LAX,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            0xbf => Instruction {
                instruction_name: InstructionName::LAX,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 4,
            },
            0xa3 => Instruction {
                instruction_name: InstructionName::LAX,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 6,
            },
            0xb3 => Instruction {
                instruction_name: InstructionName::LAX,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 5,
            },
            // LXA
            0xab => Instruction {
                instruction_name: InstructionName::LXA,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            // RLA
            0x27 => Instruction {
                instruction_name: InstructionName::RLA,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 5,
            },
            0x37 => Instruction {
                instruction_name: InstructionName::RLA,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 6,
            },
            0x2f => Instruction {
                instruction_name: InstructionName::RLA,
                addressing_mode: AddressingMode::Absolute,
                cycle: 6,
            },
            0x3f => Instruction {
                instruction_name: InstructionName::RLA,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 7,
            },
            0x3b => Instruction {
                instruction_name: InstructionName::RLA,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 7,
            },
            0x23 => Instruction {
                instruction_name: InstructionName::RLA,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 8,
            },
            0x33 => Instruction {
                instruction_name: InstructionName::RLA,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 8,
            },
            // RRA
            0x67 => Instruction {
                instruction_name: InstructionName::RRA,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 5,
            },
            0x77 => Instruction {
                instruction_name: InstructionName::RRA,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 6,
            },
            0x6f => Instruction {
                instruction_name: InstructionName::RRA,
                addressing_mode: AddressingMode::Absolute,
                cycle: 6,
            },
            0x7f => Instruction {
                instruction_name: InstructionName::RRA,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 7,
            },
            0x7b => Instruction {
                instruction_name: InstructionName::RRA,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 7,
            },
            0x63 => Instruction {
                instruction_name: InstructionName::RRA,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 8,
            },
            0x73 => Instruction {
                instruction_name: InstructionName::RRA,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 8,
            },
            // SAX
            0x87 => Instruction {
                instruction_name: InstructionName::SAX,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0x97 => Instruction {
                instruction_name: InstructionName::SAX,
                addressing_mode: AddressingMode::ZeroPageY,
                cycle: 4,
            },
            0x8f => Instruction {
                instruction_name: InstructionName::SAX,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            0x83 => Instruction {
                instruction_name: InstructionName::SAX,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 6,
            },
            // SBX
            0xcb => Instruction {
                instruction_name: InstructionName::SBX,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            // SHA
            0x9f => Instruction {
                instruction_name: InstructionName::SHA,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 5,
            },
            0x93 => Instruction {
                instruction_name: InstructionName::SHA,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 6,
            },
            // SHX
            0x9e => Instruction {
                instruction_name: InstructionName::SHX,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 5,
            },
            // SHY
            0x9c => Instruction {
                instruction_name: InstructionName::SHY,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 5,
            },
            // SLO
            0x07 => Instruction {
                instruction_name: InstructionName::SLO,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 5,
            },
            0x17 => Instruction {
                instruction_name: InstructionName::SLO,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 6,
            },
            0x0f => Instruction {
                instruction_name: InstructionName::SLO,
                addressing_mode: AddressingMode::Absolute,
                cycle: 6,
            },
            0x1f => Instruction {
                instruction_name: InstructionName::SLO,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 7,
            },
            0x1b => Instruction {
                instruction_name: InstructionName::SLO,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 7,
            },
            0x03 => Instruction {
                instruction_name: InstructionName::SLO,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 8,
            },
            0x13 => Instruction {
                instruction_name: InstructionName::SLO,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 8,
            },
            // SRE
            0x47 => Instruction {
                instruction_name: InstructionName::SRE,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 5,
            },
            0x57 => Instruction {
                instruction_name: InstructionName::SRE,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 6,
            },
            0x4f => Instruction {
                instruction_name: InstructionName::SRE,
                addressing_mode: AddressingMode::Absolute,
                cycle: 6,
            },
            0x5f => Instruction {
                instruction_name: InstructionName::SRE,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 7,
            },
            0x5b => Instruction {
                instruction_name: InstructionName::SRE,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 7,
            },
            0x43 => Instruction {
                instruction_name: InstructionName::SRE,
                addressing_mode: AddressingMode::IndirectX,
                cycle: 8,
            },
            0x53 => Instruction {
                instruction_name: InstructionName::SRE,
                addressing_mode: AddressingMode::IndirectY,
                cycle: 8,
            },
            // TAS
            0x9b => Instruction {
                instruction_name: InstructionName::TAS,
                addressing_mode: AddressingMode::AbsoluteY,
                cycle: 5,
            },
            // USBC
            0xeb => Instruction {
                instruction_name: InstructionName::USBC,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            // NOPs
            // do nothing
            0x1a => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            0x3a => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            0x5a => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            0x7a => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            0xda => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            0xfa => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::Implied,
                cycle: 2,
            },
            // NOPs: SKB
            0x80 => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0x82 => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0x89 => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0xc2 => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            0xe2 => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::Immediate,
                cycle: 2,
            },
            // NOPs: IGN
            0x04 => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0x44 => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0x64 => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::ZeroPage,
                cycle: 3,
            },
            0x14 => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 4,
            },
            0x34 => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 4,
            },
            0x54 => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 4,
            },
            0x74 => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 4,
            },
            0xd4 => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 4,
            },
            0xf4 => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::ZeroPageX,
                cycle: 4,
            },
            0x0c => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::Absolute,
                cycle: 4,
            },
            0x1c => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 4,
            },
            0x3c => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 4,
            },
            0x5c => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 4,
            },
            0x7c => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 4,
            },
            0xdc => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 4,
            },
            0xfc => Instruction {
                instruction_name: InstructionName::NOPs,
                addressing_mode: AddressingMode::AbsoluteX,
                cycle: 4,
            },
            // JAM
            0x02 => Instruction {
                instruction_name: InstructionName::JAM,
                addressing_mode: AddressingMode::Implied,
                cycle: 1,
            },
            0x12 => Instruction {
                instruction_name: InstructionName::JAM,
                addressing_mode: AddressingMode::Implied,
                cycle: 1,
            },
            0x22 => Instruction {
                instruction_name: InstructionName::JAM,
                addressing_mode: AddressingMode::Implied,
                cycle: 1,
            },
            0x32 => Instruction {
                instruction_name: InstructionName::JAM,
                addressing_mode: AddressingMode::Implied,
                cycle: 1,
            },
            0x42 => Instruction {
                instruction_name: InstructionName::JAM,
                addressing_mode: AddressingMode::Implied,
                cycle: 1,
            },
            0x52 => Instruction {
                instruction_name: InstructionName::JAM,
                addressing_mode: AddressingMode::Implied,
                cycle: 1,
            },
            0x62 => Instruction {
                instruction_name: InstructionName::JAM,
                addressing_mode: AddressingMode::Implied,
                cycle: 1,
            },
            0x72 => Instruction {
                instruction_name: InstructionName::JAM,
                addressing_mode: AddressingMode::Implied,
                cycle: 1,
            },
            0x92 => Instruction {
                instruction_name: InstructionName::JAM,
                addressing_mode: AddressingMode::Implied,
                cycle: 1,
            },
            0xb2 => Instruction {
                instruction_name: InstructionName::JAM,
                addressing_mode: AddressingMode::Implied,
                cycle: 1,
            },
            0xd2 => Instruction {
                instruction_name: InstructionName::JAM,
                addressing_mode: AddressingMode::Implied,
                cycle: 1,
            },
            0xf2 => Instruction {
                instruction_name: InstructionName::JAM,
                addressing_mode: AddressingMode::Implied,
                cycle: 1,
            },
            _ => panic!("Invalid Opcode: {:08x}", opcode),
        }
    }
    pub fn do_instruction(mycpu: &mut MyCpu, ppu: &mut Ppu) {
        let opcode: u8 = mycpu.cpu.mem[mycpu.mapper.get_mapper_address(mycpu.cpu.pc) as usize];

        let instr = Instruction::get_instruction(opcode);
        mycpu.cycle = instr.cycle;
        mycpu.counter += 1;
        // let p = mycpu.cpu.carry as u8 |
        //     (mycpu.cpu.zero as u8) << 1 |
        //     (mycpu.cpu.irq_dis as u8) << 2 |
        //     (mycpu.cpu.dec as u8) << 3 |
        //     (mycpu.cpu.b as u8) << 4 |
        //     0b0010_0000 | //ignore_flag
        //     (mycpu.cpu.overflow as u8) << 6 |
        //     (mycpu.cpu.negative as u8) << 7;

        // if mycpu.counter > 0 {
        //     println!(
        //         "{:<02X},,A:{:<02X},,X:{:<02X},,Y:{:<02X},,P:{:<02X},,SP:{:<02X},,PC:{:<02X}",
        //         opcode,
        //         mycpu.cpu.a,
        //         mycpu.cpu.x,
        //         mycpu.cpu.y,
        //         p,
        //         mycpu.cpu.sp & 0xFF,
        //         mycpu.cpu.pc
        //     );
        // }

        match instr.instruction_name {
            InstructionName::ADC => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let m = mycpu.data_read(ppu, addr) as u16;

                let mut res: u16 = (mycpu.cpu.a as u16) + m;
                if mycpu.cpu.carry {
                    res += 1;
                }

                mycpu.cpu.overflow =
                    ((mycpu.cpu.a ^ (res as u8)) & ((m as u8) ^ (res as u8)) & 0x80) == 0x80;
                mycpu.cpu.a = res as u8;

                mycpu.cpu.negative = mycpu.cpu.a & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = mycpu.cpu.a == 0;

                mycpu.cpu.carry = res > 0xff;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::AND => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let m = mycpu.data_read(ppu, addr);

                mycpu.cpu.a &= m;

                mycpu.cpu.negative = mycpu.cpu.a & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = mycpu.cpu.a == 0;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::ASL => {
                let shift_accumulator =
                    matches!(instr.addressing_mode, AddressingMode::Accumulator);
                let mut addr = 0x00;

                let m = if shift_accumulator {
                    mycpu.cpu.a
                } else {
                    addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                    mycpu.data_read(ppu, addr)
                };

                let res: u16 = (m as u16) << 1;

                if shift_accumulator {
                    mycpu.cpu.a = (res & 0xFF) as u8;
                } else {
                    mycpu.data_write(ppu, addr, (res & 0xFF) as u8);
                }

                mycpu.cpu.negative = res & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = res & 0xFF == 0;
                mycpu.cpu.carry = res & 0x100 == 0x100;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::BCC => {
                if !mycpu.cpu.carry {
                    let addr =
                        get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                    mycpu.cpu.pc = addr;
                } else {
                    mycpu.cpu.pc += 2;
                }
            }
            InstructionName::BCS => {
                if mycpu.cpu.carry {
                    let addr =
                        get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                    mycpu.cpu.pc = addr;
                } else {
                    mycpu.cpu.pc += 2;
                }
            }
            InstructionName::BEQ => {
                if mycpu.cpu.zero {
                    let addr =
                        get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                    mycpu.cpu.pc = addr;
                } else {
                    mycpu.cpu.pc += 2; // Next instruction
                }
            }
            InstructionName::BIT => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let data = mycpu.data_read(ppu, addr);

                mycpu.cpu.zero = mycpu.cpu.a & data == 0x00;
                mycpu.cpu.negative = (data & 0b1000_0000) == 0b1000_0000;
                mycpu.cpu.overflow = (data & 0b0100_0000) == 0b0100_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::BMI => {
                if mycpu.cpu.negative {
                    let addr =
                        get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                    mycpu.cpu.pc = addr;
                } else {
                    mycpu.cpu.pc += 2;
                }
            }
            InstructionName::BNE => {
                if !mycpu.cpu.zero {
                    let addr =
                        get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                    mycpu.cpu.pc = addr;
                } else {
                    mycpu.cpu.pc += 2;
                }
            }
            InstructionName::BPL => {
                if !mycpu.cpu.negative {
                    let addr =
                        get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                    mycpu.cpu.pc = addr;
                } else {
                    mycpu.cpu.pc += 2;
                }
            }
            InstructionName::BRK => {
                mycpu.cpu.b = true;
                let p = mycpu.cpu.carry as u8
                    | (mycpu.cpu.zero as u8) << 1
                    | (mycpu.cpu.irq_dis as u8) << 2
                    | (mycpu.cpu.dec as u8) << 3
                    | (mycpu.cpu.b as u8) << 4
                    | 0b0011_0000
                    | (mycpu.cpu.overflow as u8) << 6
                    | (mycpu.cpu.negative as u8) << 7;
                mycpu.cpu.pc += 2;
                mycpu.cpu.stack_push(((mycpu.cpu.pc >> 8) & 0xff) as u8);
                mycpu.cpu.stack_push((mycpu.cpu.pc & 0xff) as u8);
                mycpu.cpu.stack_push(p);
                mycpu.cpu.pc = get_irq_addr(mycpu);
                mycpu.cpu.irq_dis = true;
            }
            InstructionName::BVC => {
                if !mycpu.cpu.overflow {
                    let addr =
                        get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                    mycpu.cpu.pc = addr;
                } else {
                    mycpu.cpu.pc += 2; // Next instruction
                }
            }
            InstructionName::BVS => {
                if mycpu.cpu.overflow {
                    let addr =
                        get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                    mycpu.cpu.pc = addr;
                } else {
                    mycpu.cpu.pc += 2; // Next instruction
                }
            }
            InstructionName::CLC => {
                mycpu.cpu.carry = false;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::CLD => {
                mycpu.cpu.dec = false;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::CLI => {
                mycpu.cpu.irq_dis = false;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::CLV => {
                mycpu.cpu.overflow = false;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::CMP => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let memory = mycpu.data_read(ppu, addr);
                mycpu.cpu.negative =
                    (mycpu.cpu.a.wrapping_sub(memory)) & 0b1000_0000 == 0b1000_0000;
                if mycpu.cpu.a >= memory {
                    mycpu.cpu.zero = false;
                    if mycpu.cpu.a == memory {
                        mycpu.cpu.zero = true;
                    }
                    mycpu.cpu.carry = true;
                } else {
                    mycpu.cpu.zero = false;
                    mycpu.cpu.carry = false;
                }
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::CPX => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let memory = mycpu.data_read(ppu, addr);
                mycpu.cpu.negative =
                    (mycpu.cpu.x.wrapping_sub(memory)) & 0b1000_0000 == 0b1000_0000;
                if mycpu.cpu.x >= memory {
                    mycpu.cpu.zero = false;
                    if mycpu.cpu.x == memory {
                        mycpu.cpu.zero = true;
                    }
                    mycpu.cpu.carry = true;
                } else {
                    mycpu.cpu.zero = false;
                    mycpu.cpu.carry = false;
                }
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::CPY => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let memory = mycpu.data_read(ppu, addr);
                mycpu.cpu.negative =
                    (mycpu.cpu.y.wrapping_sub(memory)) & 0b1000_0000 == 0b1000_0000;
                if mycpu.cpu.y >= memory {
                    mycpu.cpu.zero = false;
                    if mycpu.cpu.y == memory {
                        mycpu.cpu.zero = true;
                    }
                    mycpu.cpu.carry = true;
                } else {
                    mycpu.cpu.zero = false;
                    mycpu.cpu.carry = false;
                }
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::DEC => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let value = mycpu.data_read(ppu, addr).wrapping_sub(1);
                mycpu.data_write(ppu, addr, value);
                mycpu.cpu.zero = value == 0;
                mycpu.cpu.negative = value & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::DEX => {
                mycpu.cpu.x = mycpu.cpu.x.wrapping_sub(1);
                mycpu.cpu.zero = mycpu.cpu.x == 0;
                mycpu.cpu.negative = mycpu.cpu.x & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::DEY => {
                mycpu.cpu.y = mycpu.cpu.y.wrapping_sub(1);
                mycpu.cpu.zero = mycpu.cpu.y == 0;
                mycpu.cpu.negative = mycpu.cpu.y & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::EOR => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let m = mycpu.data_read(ppu, addr);

                mycpu.cpu.a ^= m;

                mycpu.cpu.negative = mycpu.cpu.a & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = mycpu.cpu.a == 0;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::LDA => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                mycpu.cpu.a = mycpu.data_read(ppu, addr);

                mycpu.cpu.zero = mycpu.cpu.a == 0;
                mycpu.cpu.negative = mycpu.cpu.a & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::LDX => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                mycpu.cpu.x = mycpu.data_read(ppu, addr);
                mycpu.cpu.zero = mycpu.cpu.x == 0;
                mycpu.cpu.negative = mycpu.cpu.x & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::LDY => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                mycpu.cpu.y = mycpu.data_read(ppu, addr);
                mycpu.cpu.zero = mycpu.cpu.y == 0;
                mycpu.cpu.negative = mycpu.cpu.y & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::LSR => {
                let shift_accumulator =
                    matches!(instr.addressing_mode, AddressingMode::Accumulator);
                let mut addr = 0x00;

                let m = if shift_accumulator {
                    mycpu.cpu.a
                } else {
                    addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                    mycpu.data_read(ppu, addr)
                };

                mycpu.cpu.carry = m & 0x1 == 0x1;

                let res = m >> 1;

                if shift_accumulator {
                    mycpu.cpu.a = res;
                } else {
                    mycpu.data_write(ppu, addr, res);
                }

                mycpu.cpu.negative = false; // The 7th bit cannot be 1 when shifted to the right
                mycpu.cpu.zero = res == 0;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::ORA => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let m = mycpu.data_read(ppu, addr);

                mycpu.cpu.a |= m;

                mycpu.cpu.negative = mycpu.cpu.a & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = mycpu.cpu.a == 0;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::PHA => {
                mycpu.cpu.stack_push(mycpu.cpu.a);
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::PHP => {
                let p = mycpu.cpu.carry as u8 |
                    (mycpu.cpu.zero as u8) << 1 |
                    (mycpu.cpu.irq_dis as u8) << 2 |
                    (mycpu.cpu.dec as u8) << 3 |
                    0b0011_0000 | // write b flag
                    (mycpu.cpu.overflow as u8) << 6 |
                    (mycpu.cpu.negative as u8) << 7;
                mycpu.cpu.stack_push(p);
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::PLA => {
                mycpu.cpu.a = mycpu.cpu.stack_pop();
                mycpu.cpu.zero = mycpu.cpu.a == 0;
                mycpu.cpu.negative = mycpu.cpu.a & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::PLP => {
                let p = mycpu.cpu.stack_pop();
                mycpu.cpu.carry = (p & 0b0000_0001) == 0b0000_0001;
                mycpu.cpu.zero = (p & 0b0000_0010) == 0b0000_0010;
                mycpu.cpu.irq_dis = (p & 0b0000_0100) == 0b0000_0100;
                mycpu.cpu.dec = (p & 0b0000_1000) == 0b0000_1000;
                mycpu.cpu.overflow = (p & 0b0100_0000) == 0b0100_0000;
                mycpu.cpu.negative = (p & 0b1000_0000) == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
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
                mycpu.cpu.pc =
                    (mycpu.cpu.stack_pop() as u16) | ((mycpu.cpu.stack_pop() as u16) << 8);
            }
            InstructionName::RTS => {
                mycpu.cpu.pc = (mycpu.cpu.stack_pop() as u16) | (mycpu.cpu.stack_pop() as u16) << 8;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::NOP => {
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::STX => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                mycpu.data_write(ppu, addr, mycpu.cpu.x);
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::STY => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                mycpu.data_write(ppu, addr, mycpu.cpu.y);
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::INC => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let value = mycpu.data_read(ppu, addr).wrapping_add(1);
                mycpu.data_write(ppu, addr, value);
                mycpu.cpu.zero = value == 0;
                mycpu.cpu.negative = value & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::INX => {
                mycpu.cpu.x = mycpu.cpu.x.wrapping_add(1);
                mycpu.cpu.negative = mycpu.cpu.x & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = mycpu.cpu.x == 0;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::INY => {
                mycpu.cpu.y = mycpu.cpu.y.wrapping_add(1);
                mycpu.cpu.negative = mycpu.cpu.y & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = mycpu.cpu.y == 0;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::JMP => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                mycpu.cpu.pc = addr;
            }
            InstructionName::JSR => {
                mycpu.cpu.pc += 2;
                mycpu.cpu.stack_push(((mycpu.cpu.pc >> 8) & 0xff) as u8);
                mycpu.cpu.stack_push((mycpu.cpu.pc & 0xff) as u8);
                mycpu.cpu.pc -= 2;
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                mycpu.cpu.pc = addr;
            }
            InstructionName::ROL => {
                let shift_accumulator =
                    matches!(instr.addressing_mode, AddressingMode::Accumulator);
                let mut addr = 0x00;

                let m = if shift_accumulator {
                    mycpu.cpu.a
                } else {
                    addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                    mycpu.data_read(ppu, addr)
                };

                let res: u16 = ((m as u16) << 1) | (mycpu.cpu.carry as u16);

                if shift_accumulator {
                    mycpu.cpu.a = (res & 0xFF) as u8;
                } else {
                    mycpu.data_write(ppu, addr, (res & 0xFF) as u8);
                }

                mycpu.cpu.negative = res & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = res & 0xFF == 0;
                mycpu.cpu.carry = res & 0x100 == 0x100;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::ROR => {
                let shift_accumulator =
                    matches!(instr.addressing_mode, AddressingMode::Accumulator);
                let mut addr = 0x00;

                let m = if shift_accumulator {
                    mycpu.cpu.a
                } else {
                    addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                    mycpu.data_read(ppu, addr)
                };

                let res = (m >> 1) | ((mycpu.cpu.carry as u8) << 7);

                if shift_accumulator {
                    mycpu.cpu.a = res;
                } else {
                    mycpu.data_write(ppu, addr, res);
                }

                mycpu.cpu.negative = res & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = res == 0;
                mycpu.cpu.carry = m & 0x1 == 0x1;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::SBC => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let m = mycpu.data_read(ppu, addr);

                let a_is_positive = mycpu.cpu.a < 0b1000_0000;
                let m_is_positive = m < 0b1000_0000;
                let a_positive_and_m_negative = a_is_positive && !m_is_positive;
                let a_negative_and_m_positive = !a_is_positive && m_is_positive;

                let (temp, carry1) = mycpu.cpu.a.overflowing_sub(m);
                let (res, carry2) = temp.overflowing_sub(if mycpu.cpu.carry { 0 } else { 1 });
                mycpu.cpu.carry = !(carry1 || carry2);
                mycpu.cpu.zero = res == 0;
                mycpu.cpu.negative = (res & 0b1000_0000) == 0b1000_0000;
                mycpu.cpu.overflow = (a_positive_and_m_negative && res > 0b0111_1111)
                    || (a_negative_and_m_positive && res < 0b1000_0000);
                mycpu.cpu.a = res as u8;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::SEC => {
                mycpu.cpu.carry = true;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::SED => {
                mycpu.cpu.dec = true;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::SEI => {
                mycpu.cpu.irq_dis = true;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::STA => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                mycpu.data_write(ppu, addr, mycpu.cpu.a);
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::TAX => {
                mycpu.cpu.x = mycpu.cpu.a;
                mycpu.cpu.zero = mycpu.cpu.x == 0;
                mycpu.cpu.negative = mycpu.cpu.x & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::TAY => {
                mycpu.cpu.y = mycpu.cpu.a;
                mycpu.cpu.zero = mycpu.cpu.y == 0;
                mycpu.cpu.negative = mycpu.cpu.y & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::TSX => {
                mycpu.cpu.x = (mycpu.cpu.sp & 0xff) as u8;
                mycpu.cpu.zero = mycpu.cpu.x == 0;
                mycpu.cpu.negative = mycpu.cpu.x & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::TXA => {
                mycpu.cpu.a = mycpu.cpu.x;
                mycpu.cpu.zero = mycpu.cpu.a == 0;
                mycpu.cpu.negative = mycpu.cpu.a & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::TXS => {
                mycpu.cpu.sp = 0x100 | (mycpu.cpu.x as u16);
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::TYA => {
                mycpu.cpu.a = mycpu.cpu.y;
                mycpu.cpu.zero = mycpu.cpu.a == 0;
                mycpu.cpu.negative = mycpu.cpu.a & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::ALR => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let mut m = mycpu.data_read(ppu, addr);
                mycpu.cpu.a &= m;
                mycpu.cpu.carry = mycpu.cpu.a & 0x1 == 0x1;
                let res = mycpu.cpu.a >> 1;

                mycpu.cpu.a = res;

                mycpu.cpu.negative = false; // The 7th bit cannot be 1 when shifted to the right
                mycpu.cpu.zero = res == 0;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::ANC => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let m = mycpu.data_read(ppu, addr);

                // operation
                mycpu.cpu.a &= m;

                //flags
                mycpu.cpu.negative = mycpu.cpu.a & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = mycpu.cpu.a == 0;
                //copy flag n to c
                mycpu.cpu.carry = mycpu.cpu.negative;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::ANE => {
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::ARR => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let m = mycpu.data_read(ppu, addr);

                // operation AND and shift
                mycpu.cpu.a &= m;
                //mycpu.cpu.carry = mycpu.cpu.a & 0x1 == 0x1;

                let res = (mycpu.cpu.a >> 1) | ((mycpu.cpu.carry as u8) << 7);
                mycpu.cpu.a = res;

                mycpu.cpu.negative = res & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = res == 0;
                mycpu.cpu.carry = res & 0b0100_0000 == 0b0100_0000;
                mycpu.cpu.overflow = ((mycpu.cpu.a << 1) ^ mycpu.cpu.a) & 0b0100_0000 != 0;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::DCP => {
                // do DEC
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let value = mycpu.data_read(ppu, addr).wrapping_sub(1);
                let result = mycpu.cpu.a.wrapping_sub(value);
                mycpu.data_write(ppu, addr, value);
                // flags
                mycpu.cpu.negative = result & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = result == 0;
                mycpu.cpu.carry = value <= mycpu.cpu.a as u8;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::ISC => {
                // do INC
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let value = mycpu.data_read(ppu, addr).wrapping_add(1);
                mycpu.data_write(ppu, addr, value);
                // do SBC
                let m = mycpu.data_read(ppu, addr);

                let a_is_positive = mycpu.cpu.a < 0b1000_0000;
                let m_is_positive = m < 0b1000_0000;
                let a_positive_and_m_negative = a_is_positive && !m_is_positive;
                let a_negative_and_m_positive = !a_is_positive && m_is_positive;

                let (temp, carry1) = mycpu.cpu.a.overflowing_sub(m);
                let (res, carry2) = temp.overflowing_sub(if mycpu.cpu.carry { 0 } else { 1 });
                mycpu.cpu.carry = !(carry1 || carry2);
                mycpu.cpu.zero = res == 0;
                mycpu.cpu.negative = (res & 0b1000_0000) == 0b1000_0000;
                mycpu.cpu.overflow = (a_positive_and_m_negative && res > 0b0111_1111)
                    || (a_negative_and_m_positive && res < 0b1000_0000);
                mycpu.cpu.a = res as u8;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::LAS => {
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::LAX => {
                // do LDA
                //value & a
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                mycpu.cpu.a = mycpu.data_read(ppu, addr);
                //ps
                mycpu.cpu.zero = mycpu.cpu.a == 0;
                mycpu.cpu.negative = mycpu.cpu.a & 0b1000_0000 == 0b1000_0000;
                // do TAX
                mycpu.cpu.x = mycpu.cpu.a;
                //ps
                mycpu.cpu.zero = mycpu.cpu.x == 0;
                mycpu.cpu.negative = mycpu.cpu.x & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::LXA => {
                // do LDA
                //value & a
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                mycpu.cpu.a = mycpu.data_read(ppu, addr);
                //ps
                mycpu.cpu.zero = mycpu.cpu.a == 0;
                mycpu.cpu.negative = mycpu.cpu.a & 0b1000_0000 == 0b1000_0000;
                // do TAX
                mycpu.cpu.x = mycpu.cpu.a;
                //ps
                mycpu.cpu.zero = mycpu.cpu.x == 0;
                mycpu.cpu.negative = mycpu.cpu.x & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::RLA => {
                // ROL
                let shift_accumulator =
                    matches!(instr.addressing_mode, AddressingMode::Accumulator);
                let mut addr = 0x00;

                let m = if shift_accumulator {
                    mycpu.cpu.a
                } else {
                    addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                    mycpu.data_read(ppu, addr)
                };

                let res: u16 = ((m as u16) << 1) | (mycpu.cpu.carry as u16);

                if shift_accumulator {
                    mycpu.cpu.a = (res & 0xFF) as u8;
                } else {
                    mycpu.data_write(ppu, addr, (res & 0xFF) as u8);
                }
                mycpu.cpu.carry = res & 0x100 == 0x100;

                // AND
                let m1 = mycpu.data_read(ppu, addr);
                mycpu.cpu.a &= m1;

                mycpu.cpu.negative = mycpu.cpu.a & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = mycpu.cpu.a == 0;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::RRA => {
                // ROR
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);

                let m = mycpu.data_read(ppu, addr);

                let res_ror = ((m >> 1) | ((mycpu.cpu.carry as u8) << 7)) as u16;
                mycpu.data_write(ppu, addr, res_ror as u8);
                mycpu.cpu.carry = m & 0b0000_0001 != 0;

                // ADC according to the carry flag
                let mut res = (mycpu.cpu.a as u16) + res_ror;
                if mycpu.cpu.carry {
                    res += 1;
                }
                //mycpu.data_write(ppu, addr, res as u8);

                mycpu.cpu.overflow =
                    ((mycpu.cpu.a ^ (res as u8)) & ((res_ror as u8) ^ (res as u8)) & 0x80) == 0x80;
                mycpu.cpu.a = res as u8;

                mycpu.cpu.negative = mycpu.cpu.a & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = mycpu.cpu.a == 0;

                mycpu.cpu.carry = res > 0xff;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::SAX => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                // operation
                let res = mycpu.cpu.a & mycpu.cpu.x;
                mycpu.data_write(ppu, addr, res);
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::SBX => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let res = mycpu.cpu.x & mycpu.cpu.a;
                let m = mycpu.data_read(ppu, addr);
                mycpu.cpu.x = res.wrapping_sub(m);
                //set flags as CMP
                mycpu.cpu.zero = mycpu.cpu.x == 0;
                mycpu.cpu.negative = mycpu.cpu.x & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.carry = m <= res;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::SHA => {
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::SHX => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let hh = (addr >> 8).wrapping_add(1);
                let res = (mycpu.cpu.x as u16 & hh) as u8;

                if mycpu.cpu.x.wrapping_add(mycpu.data_read(ppu, addr - 2)) <= 0xFF {
                    mycpu.data_write(ppu, addr, res);
                }

                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::SHY => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let hh = (addr >> 8).wrapping_add(1);
                let res = (mycpu.cpu.y as u16 & hh) as u8;

                if mycpu.cpu.y.wrapping_add(mycpu.data_read(ppu, addr - 2)) <= 0xFF {
                    mycpu.data_write(ppu, addr, res);
                };
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::SLO => {
                // ASL
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let m = mycpu.data_read(ppu, addr);
                // Carry flag
                mycpu.cpu.carry = m & 0b1000_0000 == 0b1000_0000;
                let res = m << 1;
                mycpu.data_write(ppu, addr, res);
                // ORA
                mycpu.cpu.a |= res as u8;
                // Negative and zero flags
                mycpu.cpu.negative = mycpu.cpu.a & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = mycpu.cpu.a == 0;

                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::SRE => {
                // LSR
                let shift_accumulator =
                    matches!(instr.addressing_mode, AddressingMode::Accumulator);
                let mut addr = 0x00;

                let m = if shift_accumulator {
                    mycpu.cpu.a
                } else {
                    addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                    mycpu.data_read(ppu, addr)
                };

                // Carry flag
                mycpu.cpu.carry = m & 0x1 == 0x1;

                // operation
                let res = m >> 1;

                // Writeback
                if shift_accumulator {
                    mycpu.cpu.a = res;
                } else {
                    mycpu.data_write(ppu, addr, res);
                }

                // EOR
                // operation
                mycpu.cpu.a ^= res;

                //flags
                mycpu.cpu.negative = mycpu.cpu.a & 0b1000_0000 == 0b1000_0000;
                mycpu.cpu.zero = mycpu.cpu.a == 0;
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::TAS => {
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::USBC => {
                let addr = get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                let m = mycpu.data_read(ppu, addr);

                let a_is_positive = mycpu.cpu.a < 0b1000_0000;
                let m_is_positive = m < 0b1000_0000;
                let a_positive_and_m_negative = a_is_positive && !m_is_positive;
                let a_negative_and_m_positive = !a_is_positive && m_is_positive;

                let (temp, carry1) = mycpu.cpu.a.overflowing_sub(m);
                let (res, carry2) = temp.overflowing_sub(if mycpu.cpu.carry { 0 } else { 1 });
                mycpu.cpu.carry = !(carry1 || carry2);
                mycpu.cpu.zero = res == 0;
                mycpu.cpu.negative = (res & 0b1000_0000) == 0b1000_0000;
                mycpu.cpu.overflow = (a_positive_and_m_negative && res > 0b0111_1111)
                    || (a_negative_and_m_positive && res < 0b1000_0000);
                mycpu.cpu.a = res as u8;

                mycpu.cpu.pc += 1; //Next instruction
            }
            InstructionName::NOPs => {
                if matches!(instr.addressing_mode, AddressingMode::Implied) {
                    0;
                } else {
                    let addr =
                        get_data_address(&mut mycpu.cpu, instr.addressing_mode, &mycpu.mapper);
                    mycpu.data_read(ppu, addr);
                }
                mycpu.cpu.pc += 1; // Next instruction
            }
            InstructionName::JAM => {
                // to kill the program
                mycpu.jam = true;
            }
        }
    }
}

pub fn get_irq_addr(mycpu: &mut MyCpu) -> u16 {
    let mut dummy_ppu = Ppu::new(Mirroring::Horizontal);
    ((mycpu.data_read(&mut dummy_ppu, 0xffff) as u16) << 8)
        | mycpu.data_read(&mut dummy_ppu, 0xfffe) as u16
}

pub fn get_data_address(
    cpu: &mut Cpu6502,
    address_mode: AddressingMode,
    mapper: &MapperType,
) -> u16 {
    match address_mode {
        AddressingMode::Implied => cpu.pc,
        AddressingMode::Immediate => {
            cpu.pc += 1;
            cpu.pc
        }
        AddressingMode::Absolute => {
            let ret_addr: u16 =
                ((cpu.mem[(mapper.get_mapper_address(cpu.pc + 2)) as usize]) as u16) << 8
                    | cpu.mem[(mapper.get_mapper_address(cpu.pc + 1)) as usize] as u16;
            cpu.pc += 2;
            ret_addr
        }
        AddressingMode::AbsoluteX => {
            let ret_addr: u16 =
                ((((cpu.mem[(mapper.get_mapper_address(cpu.pc + 2)) as usize] as u16) << 8)
                    | (cpu.mem[(mapper.get_mapper_address(cpu.pc + 1)) as usize] as u16))
                    .wrapping_add(cpu.x as u16) as u16) as u16;
            cpu.pc += 2;
            ret_addr
        }
        AddressingMode::AbsoluteY => {
            let ret_addr: u16 =
                ((((cpu.mem[(mapper.get_mapper_address(cpu.pc + 2)) as usize] as u16) << 8)
                    | (cpu.mem[(mapper.get_mapper_address(cpu.pc + 1)) as usize] as u16))
                    .wrapping_add(cpu.y as u16) as u16) as u16;
            cpu.pc += 2;
            ret_addr
        }
        AddressingMode::ZeroPage => {
            cpu.pc += 1;
            cpu.mem[mapper.get_mapper_address(cpu.pc) as usize] as u16 & 0xFF
        }
        AddressingMode::ZeroPageX => {
            cpu.pc += 1;
            (cpu.mem[mapper.get_mapper_address(cpu.pc) as usize].wrapping_add(cpu.x)) as u16 & 0xFF
        }
        AddressingMode::ZeroPageY => {
            cpu.pc += 1;
            (cpu.mem[mapper.get_mapper_address(cpu.pc) as usize].wrapping_add(cpu.y)) as u16 & 0xFF
        }
        AddressingMode::Relative => {
            let add = cpu.mem[(mapper.get_mapper_address(cpu.pc + 1)) as usize];
            if add & 0b1000_0000 != 0b1000_0000 {
                // Positive
                cpu.pc += (add & 0b111_1111) as u16;
            } else {
                // Negative
                cpu.pc -= add.wrapping_neg() as u16;
            }
            cpu.pc + 2
        }
        AddressingMode::Indirect => {
            let pl_addr1: u16 = (cpu.mem[(mapper.get_mapper_address(cpu.pc + 2)) as usize] as u16)
                << 8
                | cpu.mem[(mapper.get_mapper_address(cpu.pc + 1)) as usize] as u16;
            let pl_addr2 = if cpu.mem[(mapper.get_mapper_address(cpu.pc + 1)) as usize] == 0xff {
                (cpu.mem[(mapper.get_mapper_address(cpu.pc + 2)) as usize] as u16) << 8
            } else {
                pl_addr1.wrapping_add(1)
            };
            let ret_addr: u16 = (cpu.mem[pl_addr1 as usize] as u16)
                | ((cpu.mem[((pl_addr2) as usize)] as u16) << 8);
            cpu.pc += 1;
            ret_addr
        }
        AddressingMode::IndirectX => {
            let ret_addr_low: u16 = cpu.mem[(cpu.mem
                [(mapper.get_mapper_address(cpu.pc + 1)) as usize]
                .wrapping_add(cpu.x)) as usize] as u16;
            let ret_addr_high: u16 = cpu.mem[(cpu.mem
                [(mapper.get_mapper_address(cpu.pc + 1)) as usize]
                .wrapping_add(cpu.x + 1)) as usize] as u16;
            let ret_addr = (ret_addr_high << 8) | ret_addr_low;
            cpu.pc += 1;
            ret_addr
        }
        AddressingMode::IndirectY => {
            let ret_addr_low: u16 = (cpu.mem
                [cpu.mem[(mapper.get_mapper_address(cpu.pc + 1)) as usize] as usize])
                as u16;
            let ret_addr_high: u16 = (cpu.mem[cpu.mem
                [(mapper.get_mapper_address(cpu.pc + 1)) as usize]
                .wrapping_add(1) as usize]) as u16;
            let ret_addr = ((ret_addr_high << 8) | ret_addr_low).wrapping_add(cpu.y as u16);
            cpu.pc += 1;
            ret_addr
        }
        AddressingMode::Accumulator => panic!("Accumulator does not have an address!"),
    }
}
