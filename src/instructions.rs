//! This module contains the logic of the 6502 instructions. It is used by the Bus module.

use crate::Bus;
use tudelft_nes_ppu::Ppu;

#[allow(clippy::upper_case_acronyms)] // 6502 uses upper case acronyms so we do too
/// Enum of instruction names for all supported instructions
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

/// Enum of all possible addressing modes of the 6502 instruction set. (Note: Not every instruction makes used of all addressing modes.)
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

/// A struct representing a single Instruction, which is identified by an instruction name, an addressing mode, and the needed CPU cycles to execute it.
pub struct Instruction {
    instruction_name: InstructionName,
    addressing_mode: AddressingMode,
    cycle: u16,
}

impl Instruction {
    /// Determines an `Instruction` including instruction_name and addressing_mode for the provided `opcode`.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode of the instruction
    ///
    /// # Return
    ///
    /// * `Instruction` - the instructions associated opcode, addressing mode, and cycles.
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
        }
    }

    /// Gets the data address used by an `Instruction` depending on the addressing mode. The mapper module, managed by the bus, is used to translate the given address to the correct physical location.
    ///
    /// # Arguments
    ///
    /// * `bus` - borrowed instance of Bus, which holds the CPU.
    /// * `address_mode` - The addressing mode used to determine the address.
    /// * `ppu` - borrowed instance of PPU in case PPU memory needs to be accessed.
    ///
    /// # Return
    /// * `u16` - address of the opcode. If the addressing modes is `AddressingMode::Accumulator`, the function panics because the accumulator must be addressed directly through the CPU struct.
    pub fn get_data_address(bus: &mut Bus, address_mode: AddressingMode, ppu: &mut Ppu) -> u16 {
        match address_mode {
            AddressingMode::Implied => bus.cpu.pc,
            AddressingMode::Immediate => {
                bus.cpu.pc += 1;
                bus.cpu.pc
            }
            AddressingMode::Absolute => {
                let ret_addr: u16 = ((bus.data_read(ppu, bus.cpu.pc + 2) as u16) << 8)
                    | (bus.data_read(ppu, bus.cpu.pc + 1) as u16);
                bus.cpu.pc += 2;
                ret_addr
            }
            AddressingMode::AbsoluteX => {
                let ret_addr: u16 = ((((bus.data_read(ppu, bus.cpu.pc + 2) as u16) << 8)
                    | (bus.data_read(ppu, bus.cpu.pc + 1) as u16))
                    .wrapping_add(bus.cpu.x as u16) as u16)
                    as u16;
                bus.cpu.pc += 2;
                ret_addr
            }
            AddressingMode::AbsoluteY => {
                let ret_addr: u16 = ((((bus.data_read(ppu, bus.cpu.pc + 2) as u16) << 8)
                    | (bus.data_read(ppu, bus.cpu.pc + 1) as u16))
                    .wrapping_add(bus.cpu.y as u16) as u16)
                    as u16;
                bus.cpu.pc += 2;
                ret_addr
            }
            AddressingMode::ZeroPage => {
                bus.cpu.pc += 1;
                bus.data_read(ppu, bus.cpu.pc) as u16 & 0xFF
            }
            AddressingMode::ZeroPageX => {
                bus.cpu.pc += 1;
                (bus.data_read(ppu, bus.cpu.pc).wrapping_add(bus.cpu.x)) as u16 & 0xFF
            }
            AddressingMode::ZeroPageY => {
                bus.cpu.pc += 1;
                (bus.data_read(ppu, bus.cpu.pc).wrapping_add(bus.cpu.y)) as u16 & 0xFF
            }
            AddressingMode::Relative => {
                let add = bus.data_read(ppu, bus.cpu.pc + 1);
                if add & 0b1000_0000 != 0b1000_0000 {
                    // Positive
                    bus.cpu.pc += (add & 0b111_1111) as u16;
                } else {
                    // Negative
                    bus.cpu.pc -= add.wrapping_neg() as u16;
                }
                bus.cpu.pc + 2
            }
            AddressingMode::Indirect => {
                let pl_addr1: u16 = ((bus.data_read(ppu, bus.cpu.pc + 2) as u16) << 8)
                    | (bus.data_read(ppu, bus.cpu.pc + 1) as u16);
                let pl_addr2 = if bus.data_read(ppu, bus.cpu.pc + 1) == 0xff {
                    (bus.data_read(ppu, bus.cpu.pc + 2) as u16) << 8
                } else {
                    pl_addr1.wrapping_add(1)
                };
                let ret_addr: u16 = (bus.data_read(ppu, pl_addr1) as u16)
                    | ((bus.data_read(ppu, pl_addr2) as u16) << 8);
                bus.cpu.pc += 1;
                ret_addr
            }
            AddressingMode::IndirectX => {
                let ret_addr_low_addr = ((bus.data_read(ppu, bus.cpu.pc + 1) as u16)
                    .wrapping_add(bus.cpu.x as u16)) as u16;
                let ret_addr_low: u16 = (bus.data_read(ppu, ret_addr_low_addr & 0xff)) as u16;
                let ret_addr_high_addr = (bus
                    .data_read(ppu, bus.cpu.pc + 1)
                    .wrapping_add(bus.cpu.x + 1)) as u16;
                let ret_addr_high: u16 = (bus.data_read(ppu, ret_addr_high_addr)) as u16;
                let ret_addr = (ret_addr_high << 8) | ret_addr_low;
                bus.cpu.pc += 1;
                ret_addr
            }
            AddressingMode::IndirectY => {
                let ret_addr_low_addr = bus.data_read(ppu, bus.cpu.pc + 1) as u16;
                let ret_addr_low: u16 = (bus.data_read(ppu, ret_addr_low_addr)) as u16;
                let ret_addr_high_addr =
                    (bus.data_read(ppu, bus.cpu.pc + 1).wrapping_add(1)) as u16;
                let ret_addr_high: u16 = (bus.data_read(ppu, ret_addr_high_addr)) as u16;
                let ret_addr = ((ret_addr_high << 8) | ret_addr_low).wrapping_add(bus.cpu.y as u16);
                bus.cpu.pc += 1;
                ret_addr
            }
            AddressingMode::Accumulator => panic!("Accumulator does not have an address!"),
        }
    }

    /// Finds and executes the next `Instruction` of the CPU on the bus.
    ///
    /// # Arguments
    ///
    /// * `bus` - A borrowed instance of `Bus` on which holds the CPU on which the instruction shall be executed
    /// * `ppu` - A borrowed instance of `Ppu` in case the instruction needs to access PPU memory
    ///
    /// # Return
    /// Returns nothing. If the opcode for the next instruction is invalid, either the function `get_mapper_address()` or `get_instruction()` will panic.
    pub fn do_instruction(bus: &mut Bus, ppu: &mut Ppu) {
        let opcode: u8 = bus.data_read(ppu, bus.cpu.pc);
        let instr = Instruction::get_instruction(opcode);
        bus.cycle = instr.cycle;

        match instr.instruction_name {
            InstructionName::ADC => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let m = bus.data_read(ppu, addr) as u16;

                let mut res: u16 = (bus.cpu.a as u16) + m;
                if bus.cpu.carry {
                    res += 1;
                }

                bus.cpu.overflow =
                    ((bus.cpu.a ^ (res as u8)) & ((m as u8) ^ (res as u8)) & 0x80) == 0x80;
                bus.cpu.a = res as u8;

                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = bus.cpu.a == 0;

                bus.cpu.carry = res > 0xff;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::AND => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let m = bus.data_read(ppu, addr);

                bus.cpu.a &= m;

                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = bus.cpu.a == 0;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::ASL => {
                let shift_accumulator =
                    matches!(instr.addressing_mode, AddressingMode::Accumulator);
                let mut addr = 0x00;

                let m = if shift_accumulator {
                    bus.cpu.a
                } else {
                    addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                    bus.data_read(ppu, addr)
                };

                let res: u16 = (m as u16) << 1;

                if shift_accumulator {
                    bus.cpu.a = (res & 0xFF) as u8;
                } else {
                    bus.data_write(ppu, addr, (res & 0xFF) as u8);
                }

                bus.cpu.negative = res & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = res & 0xFF == 0;
                bus.cpu.carry = res & 0x100 == 0x100;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::BCC => {
                if !bus.cpu.carry {
                    let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                    bus.cpu.pc = addr;
                } else {
                    bus.cpu.pc += 2; // Next instruction
                }
            }
            InstructionName::BCS => {
                if bus.cpu.carry {
                    let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                    bus.cpu.pc = addr;
                } else {
                    bus.cpu.pc += 2; // Next instruction
                }
            }
            InstructionName::BEQ => {
                if bus.cpu.zero {
                    let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                    bus.cpu.pc = addr;
                } else {
                    bus.cpu.pc += 2; // Next instruction
                }
            }
            InstructionName::BIT => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let data = bus.data_read(ppu, addr);

                bus.cpu.zero = bus.cpu.a & data == 0x00;
                bus.cpu.negative = (data & 0b1000_0000) == 0b1000_0000;
                bus.cpu.overflow = (data & 0b0100_0000) == 0b0100_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::BMI => {
                if bus.cpu.negative {
                    let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                    bus.cpu.pc = addr;
                } else {
                    bus.cpu.pc += 2;
                }
            }
            InstructionName::BNE => {
                if !bus.cpu.zero {
                    let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                    bus.cpu.pc = addr;
                } else {
                    bus.cpu.pc += 2;
                }
            }
            InstructionName::BPL => {
                if !bus.cpu.negative {
                    let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                    bus.cpu.pc = addr;
                } else {
                    bus.cpu.pc += 2;
                }
            }
            InstructionName::BRK => {
                bus.cpu.b = true;
                let p = bus.cpu.carry as u8
                    | (bus.cpu.zero as u8) << 1
                    | (bus.cpu.irq_dis as u8) << 2
                    | (bus.cpu.dec as u8) << 3
                    | (bus.cpu.b as u8) << 4
                    | 0b0011_0000
                    | (bus.cpu.overflow as u8) << 6
                    | (bus.cpu.negative as u8) << 7;
                bus.cpu.pc += 2;
                bus.cpu.stack_push(((bus.cpu.pc >> 8) & 0xff) as u8);
                bus.cpu.stack_push((bus.cpu.pc & 0xff) as u8);
                bus.cpu.stack_push(p);
                bus.cpu.pc =
                    ((bus.data_read(ppu, 0xffff) as u16) << 8) | bus.data_read(ppu, 0xfffe) as u16;
                bus.cpu.irq_dis = true;
            }
            InstructionName::BVC => {
                if !bus.cpu.overflow {
                    let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                    bus.cpu.pc = addr;
                } else {
                    bus.cpu.pc += 2; // Next instruction
                }
            }
            InstructionName::BVS => {
                if bus.cpu.overflow {
                    let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                    bus.cpu.pc = addr;
                } else {
                    bus.cpu.pc += 2; // Next instruction
                }
            }
            InstructionName::CLC => {
                bus.cpu.carry = false;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::CLD => {
                bus.cpu.dec = false;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::CLI => {
                bus.cpu.irq_dis = false;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::CLV => {
                bus.cpu.overflow = false;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::CMP => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let memory = bus.data_read(ppu, addr);
                bus.cpu.negative = (bus.cpu.a.wrapping_sub(memory)) & 0b1000_0000 == 0b1000_0000;
                if bus.cpu.a >= memory {
                    bus.cpu.zero = false;
                    if bus.cpu.a == memory {
                        bus.cpu.zero = true;
                    }
                    bus.cpu.carry = true;
                } else {
                    bus.cpu.zero = false;
                    bus.cpu.carry = false;
                }
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::CPX => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let memory = bus.data_read(ppu, addr);
                bus.cpu.negative = (bus.cpu.x.wrapping_sub(memory)) & 0b1000_0000 == 0b1000_0000;
                if bus.cpu.x >= memory {
                    bus.cpu.zero = false;
                    if bus.cpu.x == memory {
                        bus.cpu.zero = true;
                    }
                    bus.cpu.carry = true;
                } else {
                    bus.cpu.zero = false;
                    bus.cpu.carry = false;
                }
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::CPY => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let memory = bus.data_read(ppu, addr);
                bus.cpu.negative = (bus.cpu.y.wrapping_sub(memory)) & 0b1000_0000 == 0b1000_0000;
                if bus.cpu.y >= memory {
                    bus.cpu.zero = false;
                    if bus.cpu.y == memory {
                        bus.cpu.zero = true;
                    }
                    bus.cpu.carry = true;
                } else {
                    bus.cpu.zero = false;
                    bus.cpu.carry = false;
                }
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::DEC => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let value = bus.data_read(ppu, addr).wrapping_sub(1);
                bus.data_write(ppu, addr, value);
                bus.cpu.zero = value == 0;
                bus.cpu.negative = value & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::DEX => {
                bus.cpu.x = bus.cpu.x.wrapping_sub(1);
                bus.cpu.zero = bus.cpu.x == 0;
                bus.cpu.negative = bus.cpu.x & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::DEY => {
                bus.cpu.y = bus.cpu.y.wrapping_sub(1);
                bus.cpu.zero = bus.cpu.y == 0;
                bus.cpu.negative = bus.cpu.y & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::EOR => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let m = bus.data_read(ppu, addr);

                bus.cpu.a ^= m;

                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = bus.cpu.a == 0;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::LDA => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                bus.cpu.a = bus.data_read(ppu, addr);

                bus.cpu.zero = bus.cpu.a == 0;
                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::LDX => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                bus.cpu.x = bus.data_read(ppu, addr);
                bus.cpu.zero = bus.cpu.x == 0;
                bus.cpu.negative = bus.cpu.x & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::LDY => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                bus.cpu.y = bus.data_read(ppu, addr);
                bus.cpu.zero = bus.cpu.y == 0;
                bus.cpu.negative = bus.cpu.y & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::LSR => {
                let shift_accumulator =
                    matches!(instr.addressing_mode, AddressingMode::Accumulator);
                let mut addr = 0x00;

                let m = if shift_accumulator {
                    bus.cpu.a
                } else {
                    addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                    bus.data_read(ppu, addr)
                };

                bus.cpu.carry = m & 0x1 == 0x1;

                let res = m >> 1;

                if shift_accumulator {
                    bus.cpu.a = res;
                } else {
                    bus.data_write(ppu, addr, res);
                }

                bus.cpu.negative = false; // The 7th bit cannot be 1 when shifted to the right
                bus.cpu.zero = res == 0;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::ORA => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let m = bus.data_read(ppu, addr);

                bus.cpu.a |= m;

                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = bus.cpu.a == 0;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::PHA => {
                bus.cpu.stack_push(bus.cpu.a);
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::PHP => {
                let p = bus.cpu.carry as u8 |
                    (bus.cpu.zero as u8) << 1 |
                    (bus.cpu.irq_dis as u8) << 2 |
                    (bus.cpu.dec as u8) << 3 |
                    0b0011_0000 | // write b flag
                    (bus.cpu.overflow as u8) << 6 |
                    (bus.cpu.negative as u8) << 7;
                bus.cpu.stack_push(p);
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::PLA => {
                bus.cpu.a = bus.cpu.stack_pop();
                bus.cpu.zero = bus.cpu.a == 0;
                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::PLP => {
                let p = bus.cpu.stack_pop();
                bus.cpu.carry = (p & 0b0000_0001) == 0b0000_0001;
                bus.cpu.zero = (p & 0b0000_0010) == 0b0000_0010;
                bus.cpu.irq_dis = (p & 0b0000_0100) == 0b0000_0100;
                bus.cpu.dec = (p & 0b0000_1000) == 0b0000_1000;
                bus.cpu.overflow = (p & 0b0100_0000) == 0b0100_0000;
                bus.cpu.negative = (p & 0b1000_0000) == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }

            InstructionName::RTI => {
                let p = bus.cpu.stack_pop();
                bus.cpu.carry = (p & 0b0000_0001) == 0b0000_0001;
                bus.cpu.zero = (p & 0b0000_0010) == 0b0000_0010;
                bus.cpu.irq_dis = (p & 0b0000_0100) == 0b0000_0100;
                bus.cpu.dec = (p & 0b0000_1000) == 0b0000_1000;
                bus.cpu.b = (p & 0b0001_0000) == 0b0001_0000;
                bus.cpu.overflow = (p & 0b0100_0000) == 0b0100_0000;
                bus.cpu.negative = (p & 0b1000_0000) == 0b1000_0000;
                bus.cpu.pc = (bus.cpu.stack_pop() as u16) | ((bus.cpu.stack_pop() as u16) << 8);
            }
            InstructionName::RTS => {
                bus.cpu.pc = (bus.cpu.stack_pop() as u16) | (bus.cpu.stack_pop() as u16) << 8;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::NOP => {
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::STX => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                bus.data_write(ppu, addr, bus.cpu.x);
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::STY => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                bus.data_write(ppu, addr, bus.cpu.y);
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::INC => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let value = bus.data_read(ppu, addr).wrapping_add(1);
                bus.data_write(ppu, addr, value);
                bus.cpu.zero = value == 0;
                bus.cpu.negative = value & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::INX => {
                bus.cpu.x = bus.cpu.x.wrapping_add(1);
                bus.cpu.negative = bus.cpu.x & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = bus.cpu.x == 0;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::INY => {
                bus.cpu.y = bus.cpu.y.wrapping_add(1);
                bus.cpu.negative = bus.cpu.y & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = bus.cpu.y == 0;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::JMP => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                bus.cpu.pc = addr;
            }
            InstructionName::JSR => {
                bus.cpu.pc += 2;
                bus.cpu.stack_push(((bus.cpu.pc >> 8) & 0xff) as u8);
                bus.cpu.stack_push((bus.cpu.pc & 0xff) as u8);
                bus.cpu.pc -= 2;
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                bus.cpu.pc = addr;
            }
            InstructionName::ROL => {
                let shift_accumulator =
                    matches!(instr.addressing_mode, AddressingMode::Accumulator);
                let mut addr = 0x00;

                let m = if shift_accumulator {
                    bus.cpu.a
                } else {
                    addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                    bus.data_read(ppu, addr)
                };

                let res: u16 = ((m as u16) << 1) | (bus.cpu.carry as u16);

                if shift_accumulator {
                    bus.cpu.a = (res & 0xFF) as u8;
                } else {
                    bus.data_write(ppu, addr, (res & 0xFF) as u8);
                }

                bus.cpu.negative = res & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = res & 0xFF == 0;
                bus.cpu.carry = res & 0x100 == 0x100;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::ROR => {
                let shift_accumulator =
                    matches!(instr.addressing_mode, AddressingMode::Accumulator);
                let mut addr = 0x00;

                let m = if shift_accumulator {
                    bus.cpu.a
                } else {
                    addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                    bus.data_read(ppu, addr)
                };

                let res = (m >> 1) | ((bus.cpu.carry as u8) << 7);

                if shift_accumulator {
                    bus.cpu.a = res;
                } else {
                    bus.data_write(ppu, addr, res);
                }

                bus.cpu.negative = res & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = res == 0;
                bus.cpu.carry = m & 0x1 == 0x1;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::SBC => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let m = bus.data_read(ppu, addr);

                let a_is_positive = bus.cpu.a < 0b1000_0000;
                let m_is_positive = m < 0b1000_0000;
                let a_positive_and_m_negative = a_is_positive && !m_is_positive;
                let a_negative_and_m_positive = !a_is_positive && m_is_positive;

                let (temp, carry1) = bus.cpu.a.overflowing_sub(m);
                let (res, carry2) = temp.overflowing_sub(if bus.cpu.carry { 0 } else { 1 });
                bus.cpu.carry = !(carry1 || carry2);
                bus.cpu.zero = res == 0;
                bus.cpu.negative = (res & 0b1000_0000) == 0b1000_0000;
                bus.cpu.overflow = (a_positive_and_m_negative && res > 0b0111_1111)
                    || (a_negative_and_m_positive && res < 0b1000_0000);
                bus.cpu.a = res as u8;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::SEC => {
                bus.cpu.carry = true;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::SED => {
                bus.cpu.dec = true;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::SEI => {
                bus.cpu.irq_dis = true;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::STA => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                bus.data_write(ppu, addr, bus.cpu.a);
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::TAX => {
                bus.cpu.x = bus.cpu.a;
                bus.cpu.zero = bus.cpu.x == 0;
                bus.cpu.negative = bus.cpu.x & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::TAY => {
                bus.cpu.y = bus.cpu.a;
                bus.cpu.zero = bus.cpu.y == 0;
                bus.cpu.negative = bus.cpu.y & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::TSX => {
                bus.cpu.x = (bus.cpu.sp & 0xff) as u8;
                bus.cpu.zero = bus.cpu.x == 0;
                bus.cpu.negative = bus.cpu.x & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::TXA => {
                bus.cpu.a = bus.cpu.x;
                bus.cpu.zero = bus.cpu.a == 0;
                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::TXS => {
                bus.cpu.sp = 0x100 | (bus.cpu.x as u16);
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::TYA => {
                bus.cpu.a = bus.cpu.y;
                bus.cpu.zero = bus.cpu.a == 0;
                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::ALR => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let m = bus.data_read(ppu, addr);
                bus.cpu.a &= m;
                bus.cpu.carry = bus.cpu.a & 0x1 == 0x1;
                let res = bus.cpu.a >> 1;

                bus.cpu.a = res;

                bus.cpu.negative = false; // The 7th bit cannot be 1 when shifted to the right
                bus.cpu.zero = res == 0;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::ANC => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let m = bus.data_read(ppu, addr);

                // operation
                bus.cpu.a &= m;

                //flags
                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = bus.cpu.a == 0;
                //copy flag n to c
                bus.cpu.carry = bus.cpu.negative;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::ANE => {
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::ARR => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let m = bus.data_read(ppu, addr);

                // operation AND and shift
                bus.cpu.a &= m;

                let res = (bus.cpu.a >> 1) | ((bus.cpu.carry as u8) << 7);
                bus.cpu.a = res;

                bus.cpu.negative = res & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = res == 0;
                bus.cpu.carry = res & 0b0100_0000 == 0b0100_0000;
                bus.cpu.overflow = ((bus.cpu.a << 1) ^ bus.cpu.a) & 0b0100_0000 != 0;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::DCP => {
                // do DEC
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let value = bus.data_read(ppu, addr).wrapping_sub(1);
                let result = bus.cpu.a.wrapping_sub(value);
                bus.data_write(ppu, addr, value);
                // flags
                bus.cpu.negative = result & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = result == 0;
                bus.cpu.carry = value <= bus.cpu.a as u8;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::ISC => {
                // do INC
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let value = bus.data_read(ppu, addr).wrapping_add(1);
                bus.data_write(ppu, addr, value);
                // do SBC
                let m = bus.data_read(ppu, addr);

                let a_is_positive = bus.cpu.a < 0b1000_0000;
                let m_is_positive = m < 0b1000_0000;
                let a_positive_and_m_negative = a_is_positive && !m_is_positive;
                let a_negative_and_m_positive = !a_is_positive && m_is_positive;

                let (temp, carry1) = bus.cpu.a.overflowing_sub(m);
                let (res, carry2) = temp.overflowing_sub(if bus.cpu.carry { 0 } else { 1 });
                bus.cpu.carry = !(carry1 || carry2);
                bus.cpu.zero = res == 0;
                bus.cpu.negative = (res & 0b1000_0000) == 0b1000_0000;
                bus.cpu.overflow = (a_positive_and_m_negative && res > 0b0111_1111)
                    || (a_negative_and_m_positive && res < 0b1000_0000);
                bus.cpu.a = res as u8;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::LAS => {
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::LAX => {
                // do LDA
                //value & a
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                bus.cpu.a = bus.data_read(ppu, addr);
                //ps
                bus.cpu.zero = bus.cpu.a == 0;
                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                // do TAX
                bus.cpu.x = bus.cpu.a;
                //ps
                bus.cpu.zero = bus.cpu.x == 0;
                bus.cpu.negative = bus.cpu.x & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::LXA => {
                // do LDA
                //value & a
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                bus.cpu.a = bus.data_read(ppu, addr);
                //ps
                bus.cpu.zero = bus.cpu.a == 0;
                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                // do TAX
                bus.cpu.x = bus.cpu.a;
                //ps
                bus.cpu.zero = bus.cpu.x == 0;
                bus.cpu.negative = bus.cpu.x & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::RLA => {
                // ROL
                let shift_accumulator =
                    matches!(instr.addressing_mode, AddressingMode::Accumulator);
                let mut addr = 0x00;

                let m = if shift_accumulator {
                    bus.cpu.a
                } else {
                    addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                    bus.data_read(ppu, addr)
                };

                let res: u16 = ((m as u16) << 1) | (bus.cpu.carry as u16);

                if shift_accumulator {
                    bus.cpu.a = (res & 0xFF) as u8;
                } else {
                    bus.data_write(ppu, addr, (res & 0xFF) as u8);
                }
                bus.cpu.carry = res & 0x100 == 0x100;

                // AND
                let m1 = bus.data_read(ppu, addr);
                bus.cpu.a &= m1;

                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = bus.cpu.a == 0;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::RRA => {
                // ROR
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);

                let m = bus.data_read(ppu, addr);

                let res_ror = ((m >> 1) | ((bus.cpu.carry as u8) << 7)) as u16;
                bus.data_write(ppu, addr, res_ror as u8);
                bus.cpu.carry = m & 0b0000_0001 != 0;

                // ADC according to the carry flag
                let mut res = (bus.cpu.a as u16) + res_ror;
                if bus.cpu.carry {
                    res += 1;
                }
                //bus.data_write(ppu, addr, res as u8);

                bus.cpu.overflow =
                    ((bus.cpu.a ^ (res as u8)) & ((res_ror as u8) ^ (res as u8)) & 0x80) == 0x80;
                bus.cpu.a = res as u8;

                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = bus.cpu.a == 0;

                bus.cpu.carry = res > 0xff;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::SAX => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                // operation
                let res = bus.cpu.a & bus.cpu.x;
                bus.data_write(ppu, addr, res);
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::SBX => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let res = bus.cpu.x & bus.cpu.a;
                let m = bus.data_read(ppu, addr);
                bus.cpu.x = res.wrapping_sub(m);
                //set flags as CMP
                bus.cpu.zero = bus.cpu.x == 0;
                bus.cpu.negative = bus.cpu.x & 0b1000_0000 == 0b1000_0000;
                bus.cpu.carry = m <= res;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::SHA => {
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::SHX => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let hh = (addr >> 8).wrapping_add(1);
                let ll = addr & 0xff;
                let res = (bus.cpu.x as u16 & hh) as u8;
                let addrs1 = ((bus.cpu.x as u16 & hh) << 8) | ll;
                bus.data_write(ppu, addrs1, res);
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::SHY => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let hh = (addr >> 8).wrapping_add(1);
                let ll = addr & 0xff;
                let res = (bus.cpu.y as u16 & hh) as u8;
                let addrs1 = ((bus.cpu.y as u16 & hh) << 8) | ll;
                bus.data_write(ppu, addrs1, res);
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::SLO => {
                // ASL
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let m = bus.data_read(ppu, addr);
                // Carry flag
                bus.cpu.carry = m & 0b1000_0000 == 0b1000_0000;
                let res = m << 1;
                bus.data_write(ppu, addr, res);
                // ORA
                bus.cpu.a |= res as u8;
                // Negative and zero flags
                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = bus.cpu.a == 0;

                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::SRE => {
                // LSR
                let shift_accumulator =
                    matches!(instr.addressing_mode, AddressingMode::Accumulator);
                let mut addr = 0x00;

                let m = if shift_accumulator {
                    bus.cpu.a
                } else {
                    addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                    bus.data_read(ppu, addr)
                };

                // Carry flag
                bus.cpu.carry = m & 0x1 == 0x1;

                // operation
                let res = m >> 1;

                // Writeback
                if shift_accumulator {
                    bus.cpu.a = res;
                } else {
                    bus.data_write(ppu, addr, res);
                }

                // EOR
                // operation
                bus.cpu.a ^= res;

                //flags
                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = bus.cpu.a == 0;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::TAS => {
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::USBC => {
                let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                let m = bus.data_read(ppu, addr);

                let a_is_positive = bus.cpu.a < 0b1000_0000;
                let m_is_positive = m < 0b1000_0000;
                let a_positive_and_m_negative = a_is_positive && !m_is_positive;
                let a_negative_and_m_positive = !a_is_positive && m_is_positive;

                let (temp, carry1) = bus.cpu.a.overflowing_sub(m);
                let (res, carry2) = temp.overflowing_sub(if bus.cpu.carry { 0 } else { 1 });
                bus.cpu.carry = !(carry1 || carry2);
                bus.cpu.zero = res == 0;
                bus.cpu.negative = (res & 0b1000_0000) == 0b1000_0000;
                bus.cpu.overflow = (a_positive_and_m_negative && res > 0b0111_1111)
                    || (a_negative_and_m_positive && res < 0b1000_0000);
                bus.cpu.a = res as u8;

                bus.cpu.pc += 1; //Next instruction
            }
            InstructionName::NOPs => {
                if !matches!(instr.addressing_mode, AddressingMode::Implied) {
                    let addr = Self::get_data_address(bus, instr.addressing_mode, ppu);
                    bus.data_read(ppu, addr);
                }
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::JAM => {
                // To kill the program
                bus.jam = true;
            }
        }
    }
}
