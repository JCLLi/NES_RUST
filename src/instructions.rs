use crate::Bus;
use tudelft_nes_ppu::Ppu;

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
            _ => panic!("Invalid Opcode: {:08x}", opcode),
        }
    }
    pub fn do_instruction(bus: &mut Bus, ppu: &mut Ppu) {
        let opcode: u8 = bus.data_read(ppu, bus.cpu.pc);

        let instr = Instruction::get_instruction(opcode);
        bus.cycle = instr.cycle;

        match instr.instruction_name {
            InstructionName::ADC => {
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                    addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                    let addr = get_data_address(bus, instr.addressing_mode, ppu);
                    bus.cpu.pc = addr;
                } else {
                    bus.cpu.pc += 2;
                }
            }
            InstructionName::BCS => {
                if bus.cpu.carry {
                    let addr = get_data_address(bus, instr.addressing_mode, ppu);
                    bus.cpu.pc = addr;
                } else {
                    bus.cpu.pc += 2;
                }
            }
            InstructionName::BEQ => {
                if bus.cpu.zero {
                    let addr = get_data_address(bus, instr.addressing_mode, ppu);
                    bus.cpu.pc = addr;
                } else {
                    bus.cpu.pc += 2; // Next instruction
                }
            }
            InstructionName::BIT => {
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
                let data = bus.data_read(ppu, addr);

                bus.cpu.zero = bus.cpu.a & data == 0x00;
                bus.cpu.negative = (data & 0b1000_0000) == 0b1000_0000;
                bus.cpu.overflow = (data & 0b0100_0000) == 0b0100_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::BMI => {
                if bus.cpu.negative {
                    let addr = get_data_address(bus, instr.addressing_mode, ppu);
                    bus.cpu.pc = addr;
                } else {
                    bus.cpu.pc += 2;
                }
            }
            InstructionName::BNE => {
                if !bus.cpu.zero {
                    let addr = get_data_address(bus, instr.addressing_mode, ppu);
                    bus.cpu.pc = addr;
                } else {
                    bus.cpu.pc += 2;
                }
            }
            InstructionName::BPL => {
                if !bus.cpu.negative {
                    let addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                    let addr = get_data_address(bus, instr.addressing_mode, ppu);
                    bus.cpu.pc = addr;
                } else {
                    bus.cpu.pc += 2; // Next instruction
                }
            }
            InstructionName::BVS => {
                if bus.cpu.overflow {
                    let addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
                let m = bus.data_read(ppu, addr);

                bus.cpu.a ^= m;

                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                bus.cpu.zero = bus.cpu.a == 0;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::LDA => {
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
                bus.cpu.a = bus.data_read(ppu, addr);

                bus.cpu.zero = bus.cpu.a == 0;
                bus.cpu.negative = bus.cpu.a & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::LDX => {
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
                bus.cpu.x = bus.data_read(ppu, addr);
                bus.cpu.zero = bus.cpu.x == 0;
                bus.cpu.negative = bus.cpu.x & 0b1000_0000 == 0b1000_0000;
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::LDY => {
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                    addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
                bus.data_write(ppu, addr, bus.cpu.x);
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::STY => {
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
                bus.data_write(ppu, addr, bus.cpu.y);
                bus.cpu.pc += 1; // Next instruction
            }
            InstructionName::INC => {
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
                bus.cpu.pc = addr;
            }
            InstructionName::JSR => {
                bus.cpu.pc += 2;
                bus.cpu.stack_push(((bus.cpu.pc >> 8) & 0xff) as u8);
                bus.cpu.stack_push((bus.cpu.pc & 0xff) as u8);
                bus.cpu.pc -= 2;
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
                bus.cpu.pc = addr;
            }
            InstructionName::ROL => {
                let shift_accumulator =
                    matches!(instr.addressing_mode, AddressingMode::Accumulator);
                let mut addr = 0x00;

                let m = if shift_accumulator {
                    bus.cpu.a
                } else {
                    addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                    addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
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
                let addr = get_data_address(bus, instr.addressing_mode, ppu);
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
        }
    }
}

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
                .wrapping_add(bus.cpu.x as u16) as u16) as u16;
            bus.cpu.pc += 2;
            ret_addr
        }
        AddressingMode::AbsoluteY => {
            let ret_addr: u16 = ((((bus.data_read(ppu, bus.cpu.pc + 2) as u16) << 8)
                | (bus.data_read(ppu, bus.cpu.pc + 1) as u16))
                .wrapping_add(bus.cpu.y as u16) as u16) as u16;
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
            let ret_addr_low_addr =
                ((bus.data_read(ppu, bus.cpu.pc + 1) as u16).wrapping_add(bus.cpu.x as u16)) as u16;
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
            let ret_addr_high_addr = (bus.data_read(ppu, bus.cpu.pc + 1).wrapping_add(1)) as u16;
            let ret_addr_high: u16 = (bus.data_read(ppu, ret_addr_high_addr)) as u16;
            let ret_addr = ((ret_addr_high << 8) | ret_addr_low).wrapping_add(bus.cpu.y as u16);
            bus.cpu.pc += 1;
            ret_addr
        }
        AddressingMode::Accumulator => panic!("Accumulator does not have an address!"),
    }
}
