#[cfg(test)]
mod instruction_tests {
    use crate::instructions::Instruction;
    use crate::mapper::get_mapped_address;
    use crate::MyCpu;

    #[test]
    fn test_bcc() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.mem[0x8000] = 0x90; // BCC Relative
        test_cpu.cpu.mem[0x8001] = 0x08;
        test_cpu.cpu.carry = true;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8001); // No branch

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x90; // BCC Relative
        test_cpu.cpu.mem[0x8001] = 0x08;
        test_cpu.cpu.carry = false;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x800a);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x90; // BCC Relative
        test_cpu.cpu.mem[0x8001] = 0b1111_1101; // -3
        test_cpu.cpu.carry = false;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x7fff);
    }
    #[test]
    fn test_bcs() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.mem[0x8000] = 0xb0; // BCS Relative
        test_cpu.cpu.mem[0x8001] = 0x08;
        test_cpu.cpu.carry = false;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8001); // No branch

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0xb0; // BCS Relative
        test_cpu.cpu.mem[0x8001] = 0x18;
        test_cpu.cpu.carry = true;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x801a);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0xb0; // BCC Relative
        test_cpu.cpu.mem[0x8001] = 0b1111_1011; // -5
        test_cpu.cpu.carry = true;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x7ffd);
    }
    #[test]
    fn test_beq() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.mem[0x8000] = 0xf0; // BEQ Relative
        test_cpu.cpu.mem[0x8001] = 0x08;
        test_cpu.cpu.zero = false;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8001); // No branch

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0xf0; // BEQ Relative
        test_cpu.cpu.mem[0x8001] = 0x7f;
        test_cpu.cpu.zero = true;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8081);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0xf0; // BEQ Relative
        test_cpu.cpu.mem[0x8001] = 0b1001_1100; // -100
        test_cpu.cpu.zero = true;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x7f9e);
    }
    #[test]
    fn test_bit() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.mem[0x8000] = 0x24; // BIT Zero Page
        test_cpu.cpu.mem[0x8001] = 0xff;
        test_cpu.cpu.mem[0x00ff] = 0b1100_0000;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.overflow, true);

        test_cpu.cpu.negative = false;
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x24; // BIT Zero Page
        test_cpu.cpu.mem[0x8001] = 0x01;
        test_cpu.cpu.mem[0x0001] = 0b1000_0000;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.overflow, false);

        test_cpu.cpu.negative = false;
        test_cpu.cpu.overflow = false;
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x2c; // BIT Absolute
        test_cpu.cpu.mem[0x8001] = 0xff;
        test_cpu.cpu.mem[0x8002] = 0x01;
        test_cpu.cpu.mem[0x01ff] = 0b0100_0000;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.overflow, true);
    }
    #[test]
    fn test_bmi() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.mem[0x8000] = 0x30; // BMI Relative
        test_cpu.cpu.mem[0x8001] = 0x08;
        test_cpu.cpu.negative = false;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8001); // No branch

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x30; // BMI Relative
        test_cpu.cpu.mem[0x8001] = 0x7f;
        test_cpu.cpu.negative = true;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8081);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x30; // BMI Relative
        test_cpu.cpu.mem[0x8001] = 0b1001_1100; // -100
        test_cpu.cpu.negative = true;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x7f9e);
    }
    #[test]
    fn test_bne() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.mem[0x8000] = 0xd0; // BNE Relative
        test_cpu.cpu.mem[0x8001] = 0x08;
        test_cpu.cpu.zero = true;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8001); // No branch

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0xd0; // BNE Relative
        test_cpu.cpu.mem[0x8001] = 0x7f;
        test_cpu.cpu.zero = false;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8081);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0xd0; // BNE Relative
        test_cpu.cpu.mem[0x8001] = 0b1001_1100; // -100
        test_cpu.cpu.zero = false;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x7f9e);
    }
    #[test]
    fn test_bpl() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.mem[0x8000] = 0x10; // BPL Relative
        test_cpu.cpu.mem[0x8001] = 0x08;
        test_cpu.cpu.negative = true;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8001); // No branch

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x10; // BPL Relative
        test_cpu.cpu.mem[0x8001] = 0x7f;
        test_cpu.cpu.negative = false;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8081);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x10; // BPL Relative
        test_cpu.cpu.mem[0x8001] = 0b1001_1100; // -100
        test_cpu.cpu.negative = false;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x7f9e);
    }
    #[test]
    fn test_bvc() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.mem[0x8000] = 0x50; // BVC Relative
        test_cpu.cpu.mem[0x8001] = 0x08;
        test_cpu.cpu.overflow = true;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8001); // No branch

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x50; // BVC Relative
        test_cpu.cpu.mem[0x8001] = 0x7f;
        test_cpu.cpu.overflow = false;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8081);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x50; // BVC Relative
        test_cpu.cpu.mem[0x8001] = 0b1001_1100; // -100
        test_cpu.cpu.overflow = false;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x7f9e);
    }
    #[test]
    fn test_bvs() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.mem[0x8000] = 0x70; // BVS Relative
        test_cpu.cpu.mem[0x8001] = 0x08;
        test_cpu.cpu.overflow = false;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8001); // No branch

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x70; // BVS Relative
        test_cpu.cpu.mem[0x8001] = 0x7f;
        test_cpu.cpu.overflow = true;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8081);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x70; // BVS Relative
        test_cpu.cpu.mem[0x8001] = 0b1001_1100; // -100
        test_cpu.cpu.overflow = true;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x7f9e);
    }
    #[test]
    fn test_inx() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.x = 5;
        test_cpu.cpu.mem[0x8000] = 0xe8; // INX Implied
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.x, 6);
        assert!(!test_cpu.cpu.negative);
        assert!(!test_cpu.cpu.zero);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.x = 0xff;
        test_cpu.cpu.mem[0x8000] = 0xe8; // INX Implied
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.x, 0);
        assert!(test_cpu.cpu.zero);
        assert!(!test_cpu.cpu.negative);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.x = 0b1001_1100;
        test_cpu.cpu.mem[0x8000] = 0xe8; // INX Implied
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.x, 0b1001_1101);
        assert!(!test_cpu.cpu.zero);
        assert!(test_cpu.cpu.negative);
    }
    #[test]
    fn test_iny() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.y = 6;
        test_cpu.cpu.mem[0x8000] = 0xc8; // INY Implied
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.y, 7);
        assert!(!test_cpu.cpu.negative);
        assert!(!test_cpu.cpu.zero);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.y = 0xff;
        test_cpu.cpu.mem[0x8000] = 0xc8; // INY Implied
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.y, 0);
        assert!(!test_cpu.cpu.negative);
        assert!(test_cpu.cpu.zero);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.y = 0b1011_1111;
        test_cpu.cpu.mem[0x8000] = 0xc8; // INY Implied
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.y, 0b1100_0000);
        assert!(test_cpu.cpu.negative);
        assert!(!test_cpu.cpu.zero);
    }

    #[test]
    fn test_jmp() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.mem[0x8000] = 0x4c; // JMP Absolute
        test_cpu.cpu.mem[0x8001] = 0x01;
        test_cpu.cpu.mem[0x8002] = 0x02;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x0201);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x1100] = 0x66;
        test_cpu.cpu.mem[0x1101] = 0x67;
        test_cpu.cpu.mem[0x8000] = 0x6c; // JMP Indirect
        test_cpu.cpu.mem[0x8001] = 0x00;
        test_cpu.cpu.mem[0x8002] = 0x11;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x6667);
    }

    #[test]
    fn test_ldx() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.mem[0x8000] = 0xa2; // LDX Immediate
        test_cpu.cpu.mem[0x8001] = 0x42;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.x, 0x42);
        assert!(!test_cpu.cpu.negative);
        assert!(!test_cpu.cpu.zero);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x0001] = 0xff; // -1
        test_cpu.cpu.mem[0x8000] = 0xa6; // LDX Zero Page
        test_cpu.cpu.mem[0x8001] = 0x01;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.x, 0xff);
        assert!(test_cpu.cpu.negative);
        assert!(!test_cpu.cpu.zero);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.y = 0x2;
        test_cpu.cpu.mem[0x0003] = 0x00;
        test_cpu.cpu.mem[0x8000] = 0xb6; // LDX Zero Page Y
        test_cpu.cpu.mem[0x8001] = 0x01;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.x, 0x00);
        assert!(!test_cpu.cpu.negative);
        assert!(test_cpu.cpu.zero);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x0101] = 0x45;
        test_cpu.cpu.mem[0x8000] = 0xae; // LDX Absolute
        test_cpu.cpu.mem[0x8001] = 0x01;
        test_cpu.cpu.mem[0x8002] = 0x01;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.x, 0x45);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.y = 0x3;
        test_cpu.cpu.mem[0x8000] = 0xbe; // LDX Absolute Y
        test_cpu.cpu.mem[0x0105] = 0x46;
        test_cpu.cpu.mem[0x8001] = 0x02;
        test_cpu.cpu.mem[0x8002] = 0x01;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.x, 0x46);
    }
    #[test]
    fn test_ldy() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.mem[0x8000] = 0xa0; // LDY Immediate
        test_cpu.cpu.mem[0x8001] = 0x42;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.y, 0x42);
        assert!(!test_cpu.cpu.negative);
        assert!(!test_cpu.cpu.zero);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x0001] = 0xfe;
        test_cpu.cpu.mem[0x8000] = 0xa4; // LDY Zero Page
        test_cpu.cpu.mem[0x8001] = 0x01;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.y, 0xfe);
        assert!(test_cpu.cpu.negative);
        assert!(!test_cpu.cpu.zero);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.x = 0x2;
        test_cpu.cpu.mem[0x0003] = 0x00;
        test_cpu.cpu.mem[0x8000] = 0xb4; // LDY Zero Page X
        test_cpu.cpu.mem[0x8001] = 0x01;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.y, 0x00);
        assert!(!test_cpu.cpu.negative);
        assert!(test_cpu.cpu.zero);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x0101] = 45;
        test_cpu.cpu.mem[0x8000] = 0xac; // LDY Absolute
        test_cpu.cpu.mem[0x8001] = 0x01;
        test_cpu.cpu.mem[0x8002] = 0x01;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.y, 45);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.x = 0x3;
        test_cpu.cpu.mem[0x8000] = 0xbc; // LDY Absolute X
        test_cpu.cpu.mem[0x0105] = 0x46;
        test_cpu.cpu.mem[0x8001] = 0x02;
        test_cpu.cpu.mem[0x8002] = 0x01;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.y, 0x46);
    }
    #[test]
    fn test_rti_brk() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.mem[0x8000] = 0x00; // BRK Implied
        test_cpu.cpu.zero = true; // Set a status flag
        test_cpu.cpu.mem[get_mapped_address(
            test_cpu.cartridge.mapper_number,
            0xfffe,
            test_cpu.cartridge.prg_rom_size_in_16kb,
        ) as usize] = 0x66; // Set IRQ vector
        test_cpu.cpu.mem[get_mapped_address(
            test_cpu.cartridge.mapper_number,
            0xffff,
            test_cpu.cartridge.prg_rom_size_in_16kb,
        ) as usize] = 0x07; // Set IRQ vector
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(
            test_cpu.cpu.pc,
            get_mapped_address(
                test_cpu.cartridge.mapper_number,
                0x0766,
                test_cpu.cartridge.prg_rom_size_in_16kb
            )
        );
        assert_eq!(test_cpu.cpu.mem[test_cpu.cpu.sp as usize + 3], 0x01);
        assert_eq!(test_cpu.cpu.mem[test_cpu.cpu.sp as usize + 2], 0x80);
        assert_eq!(test_cpu.cpu.mem[test_cpu.cpu.sp as usize + 1], 0x22); // NOTE 6th bit is alwasy set to 1

        test_cpu.cpu.mem[0x0766] = 0x40; // RTI Implied
        test_cpu.cpu.zero = false; // Set a status flag
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8001 + 1);
        assert_eq!(test_cpu.cpu.zero, true);
    }
    #[test]
    fn test_rts_jsr() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.mem[0x8000] = 0x20; // JSR Absolute
        test_cpu.cpu.mem[0x8001] = 0x01;
        test_cpu.cpu.mem[0x8002] = 0x02;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x0201);
        assert_eq!(test_cpu.cpu.mem[(test_cpu.cpu.sp + 2) as usize], 0x02);
        assert_eq!(test_cpu.cpu.mem[(test_cpu.cpu.sp + 1) as usize], 0x80);

        test_cpu.cpu.mem[0x0201] = 0x60; // RTS Implied
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8003);
    }
    #[test]
    fn test_nop() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0xea; // NOP Implied
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.pc, 0x8001);
    }
    #[test]
    fn test_stx() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.x = 55;
        test_cpu.cpu.mem[0x8000] = 0x86; // STX Zero Page
        test_cpu.cpu.mem[0x8001] = 0x66;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x0066], 55);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.x = 57;
        test_cpu.cpu.y = 0x01;
        test_cpu.cpu.mem[0x8000] = 0x96; // STX Zero Page Y
        test_cpu.cpu.mem[0x8001] = 0x67;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x0068], 57);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.x = 58;
        test_cpu.cpu.mem[0x8000] = 0x8e; // STX Absolute
        test_cpu.cpu.mem[0x8001] = 0x67;
        test_cpu.cpu.mem[0x8002] = 0x68;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x6867], 58);
    }
    #[test]
    fn test_sty() {
        let mut test_cpu = MyCpu::default();
        test_cpu.cpu.y = 100;
        test_cpu.cpu.mem[0x8000] = 0x84; // STY Zero Page
        test_cpu.cpu.mem[0x8001] = 0x66;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x0066], 100);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.y = 101;
        test_cpu.cpu.x = 0x03;
        test_cpu.cpu.mem[0x8000] = 0x94; // STY Zero Page Y
        test_cpu.cpu.mem[0x8001] = 0x67;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x006a], 101);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.y = 200;
        test_cpu.cpu.mem[0x8000] = 0x8c; // STY Absolute
        test_cpu.cpu.mem[0x8001] = 0x67;
        test_cpu.cpu.mem[0x8002] = 0x44;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x4467], 200);
    }
    #[test]
    fn test_adc() {
        let mut test_cpu = MyCpu::default();

        // signed:   5 + 5 = 10 | V -> 0
        // unsigned: 5 + 5 = 10 | C -> 0
        test_cpu.cpu.mem[0x8000] = 0x69; // ADC Immediate
        test_cpu.cpu.mem[0x8001] = 0x05;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.a = 0x05;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0x0a);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, false);

        // signed:   -12 + 14 = 2                 | V -> 0
        // unsigned: 244 + 14 = (1)_0000_0010 | C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x69; // ADC Immediate
        test_cpu.cpu.mem[0x8001] = 0b1111_0100; // -12
        test_cpu.cpu.a = 14;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 2);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, true);
        assert_eq!(test_cpu.cpu.overflow, false);

        // signed:   -12 + 12 = 0             | V -> 0
        // unsigned: 244 + 12 = (1)_0000_0000 | C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x69; // ADC Immediate
        test_cpu.cpu.mem[0x8001] = 0b1111_0100; // -12
        test_cpu.cpu.a = 12;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, true);
        assert_eq!(test_cpu.cpu.carry, true);
        assert_eq!(test_cpu.cpu.overflow, false);

        // signed:   -12 + -12 = -24            | V -> 0
        // unsigned: 244 + 244 = (1)_1110_1000  | C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x69; // ADC Immediate
        test_cpu.cpu.mem[0x8001] = 0b1111_0100; // -12
        test_cpu.cpu.a = 0b1111_0100; // -12
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1110_1000);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, true);
        assert_eq!(test_cpu.cpu.overflow, false);

        // signed:   100 + 50 = -106  | V -> 1
        // unsigned: 100 + 50 = 150   | C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x69; // ADC Immediate
        test_cpu.cpu.mem[0x8001] = 50;
        test_cpu.cpu.a = 100;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 150);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, true);

        // signed:   -1 + -1 = -2               | V -> 0
        // unsigned: 255 + 255 = (1)_1111_1110  | C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x69; // ADC Immediate
        test_cpu.cpu.mem[0x8001] = 0b1111_1111; // -1
        test_cpu.cpu.a = 0b1111_1111; // -1
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1111_1110);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, true);
        assert_eq!(test_cpu.cpu.overflow, false);

        // signed:   0 + 0 = 0  | V -> 0
        // unsigned: 0 + 0 = 0  | C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x69; // ADC Immediate
        test_cpu.cpu.mem[0x8001] = 0; // -1
        test_cpu.cpu.a = 0; // -1
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, true);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, false);

        /* Other addressing modes */

        // signed:   -12 + -12 = -24            | V -> 0
        // unsigned: 244 + 244 = (1)_1110_1000  | C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x65; // ADC Zeropage
        test_cpu.cpu.mem[0x0091] = 0b1111_0100; // -12
        test_cpu.cpu.mem[0x8001] = 0x91; // Address on zeropage
        test_cpu.cpu.a = 0b1111_0100; // -12
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1110_1000);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, true);
        assert_eq!(test_cpu.cpu.overflow, false);

        // signed:   -12 + -12 = -24            | V -> 0
        // unsigned: 244 + 244 = (1)_1110_1000  | C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x75; // ADC ZeropageX
        test_cpu.cpu.mem[0x0096] = 0b1111_0100; // -12
        test_cpu.cpu.mem[0x8001] = 0x91; // Address on zeropage
        test_cpu.cpu.x = 5;
        test_cpu.cpu.a = 0b1111_0100; // -12
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1110_1000);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, true);
        assert_eq!(test_cpu.cpu.overflow, false);

        // // signed:   -12 + -12 = -24            | V -> 0
        // // unsigned: 244 + 244 = (1)_1110_1000  | C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x6D; // ADC Absolute
        test_cpu.cpu.mem[0x9096] = 0b1111_0100; // -12
        test_cpu.cpu.mem[0x8001] = 0x96; // LL
        test_cpu.cpu.mem[0x8002] = 0x90; // HH
        test_cpu.cpu.a = 0b1111_0100; // -12
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1110_1000);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, true);
        assert_eq!(test_cpu.cpu.overflow, false);

        // // signed:   -12 + -12 = -24            | V -> 0
        // // unsigned: 244 + 244 = (1)_1110_1000  | C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x7D; // ADC AbsoluteX
        test_cpu.cpu.mem[0x3093] = 0b1111_0100; // -12
        test_cpu.cpu.mem[0x8001] = 0x91; // LL
        test_cpu.cpu.mem[0x8002] = 0x30; // HH
        test_cpu.cpu.x = 2;
        test_cpu.cpu.a = 0b1111_0100; // -12
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1110_1000);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, true);
        assert_eq!(test_cpu.cpu.overflow, false);

        // signed:   -12 + -12 = -24            | V -> 0
        // unsigned: 244 + 244 = (1)_1110_1000  | C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x79; // ADC AbsoluteY
        test_cpu.cpu.mem[0x5151] = 0b1111_0100; // -12
        test_cpu.cpu.mem[0x8001] = 0x41;
        test_cpu.cpu.mem[0x8002] = 0x51;
        test_cpu.cpu.y = 0x10;
        test_cpu.cpu.a = 0b1111_0100; // -12
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1110_1000);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, true);
        assert_eq!(test_cpu.cpu.overflow, false);

        // signed:   5 + 12 + C = 18  | V -> 0
        // unsigned: 5 + 12 + C = 18  | C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = true;
        test_cpu.cpu.mem[0x8000] = 0x61; // ADC IndirectX
        test_cpu.cpu.mem[0x8001] = 0x04;
        test_cpu.cpu.x = 0x03;

        test_cpu.cpu.mem[0x0007] = 0x13; // Address of m

        test_cpu.cpu.mem[0x0013] = 0x12; // m
        test_cpu.cpu.a = 5;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0x18);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, false);

        // signed:   -12 + -12 = -24            | V -> 0
        // unsigned: 244 + 244 = (1)_1110_1000  | C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x71; // ADC IndirectY
        test_cpu.cpu.mem[0x8001] = 0x33;
        test_cpu.cpu.mem[0x0033] = 0x73;
        test_cpu.cpu.y = 0x4;
        test_cpu.cpu.mem[0x0077] = 0b1111_0100; // m = -12
        test_cpu.cpu.a = 0b1111_0100; // -12
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1110_1000);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, true);
        assert_eq!(test_cpu.cpu.overflow, false);
    }

    #[test]
    fn test_sbc() {
        let mut test_cpu = MyCpu::default();

        // signed:   5 - 5 = 0  | V -> 0
        // unsigned: 5 - 5 = 0 | C -> 0
        test_cpu.cpu.mem[0x8000] = 0xE9; // SBC Immediate
        test_cpu.cpu.mem[0x8001] = 0x05;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.a = 0x05;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0x0);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, true);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, false);

        // signed:   -12 - 14 = -26  | V -> 0
        // unsigned: 244 - 14 = 230  | C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0xE9; // SBC Immediate
        test_cpu.cpu.mem[0x8001] = 14;
        test_cpu.cpu.a = 0b1111_0100; // -12
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1110_0110); // -26 -> 0b1110_0110
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, false);

        // signed:   12 - 12 = 0  | V -> 0
        // unsigned: 12 - 12 = 0  | C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0xE9; // SBC Immediate
        test_cpu.cpu.mem[0x8001] = 12;
        test_cpu.cpu.a = 12;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, true);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, false);

        // signed:   4 - (-12) = 16             | V -> 0
        // unsigned: 4 - 244 -> 260 - 244 = 16  | C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0xE9; // SBC Immediate
        test_cpu.cpu.mem[0x8001] = 0b1111_0100; // -12
        test_cpu.cpu.a = 4;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 16);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, true);
        assert_eq!(test_cpu.cpu.overflow, false);

        // signed:   -100 - 100 = -200 -> 56  (1)0011_1000  | V -> 1
        // unsigned: 156 - 100 = 56                         | C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0xE9; // SBC Immediate
        test_cpu.cpu.mem[0x8001] = 100;
        test_cpu.cpu.a = 0b1001_1100; // -100
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 56);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, true);

        // signed:   0 - 0 = 0  | V -> 0
        // unsigned: 0 - 0 = 0  | C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0xE9; // SBC Immediate
        test_cpu.cpu.mem[0x8001] = 0; // -1
        test_cpu.cpu.a = 0; // -1
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, true);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, false);

        /* Other addressing modes */

        // signed:   -100 - 100 = -200 -> 56  (1)0011_1000  | V -> 1
        // unsigned: 156 - 100 = 56                         | C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0xE5; // SBC Zeropage
        test_cpu.cpu.mem[0x0091] = 100;
        test_cpu.cpu.mem[0x8001] = 0x91; // Address on zeropage
        test_cpu.cpu.a = 0b1001_1100; // -100
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 56);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, true);

        // signed:   -100 - 100 = -200 -> 56  (1)0011_1000  | V -> 1
        // unsigned: 156 - 100 = 56                         | C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0xF5; // SBC ZeropageX
        test_cpu.cpu.mem[0x0096] = 100;
        test_cpu.cpu.mem[0x8001] = 0x91; // Address on zeropage
        test_cpu.cpu.x = 5;
        test_cpu.cpu.a = 0b1001_1100; // -100
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 56);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, true);

        // signed:   -100 - 100 = -200 -> 56  (1)0011_1000  | V -> 1
        // unsigned: 156 - 100 = 56                         | C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0xED; // SBC Absolute
        test_cpu.cpu.mem[0x8896] = 100;
        test_cpu.cpu.mem[0x8001] = 0x96;
        test_cpu.cpu.mem[0x8002] = 0x88;
        test_cpu.cpu.a = 0b1001_1100; // -100
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 56);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, true);

        // signed:   -100 - 100 = -200 -> 56  (1)0011_1000  | V -> 1
        // unsigned: 156 - 100 = 56                         | C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0xFD; // SBC AbsoluteX
        test_cpu.cpu.mem[0x8E93] = 100;
        test_cpu.cpu.mem[0x8001] = 0x91;
        test_cpu.cpu.mem[0x8002] = 0x8E;
        test_cpu.cpu.x = 2;
        test_cpu.cpu.a = 0b1001_1100; // -100
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 56);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, true);

        // signed:   -100 - 100 = -200 -> 56  (1)0011_1000  | V -> 1
        // unsigned: 156 - 100 = 56                         | C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0xF9; // SBC AbsoluteY
        test_cpu.cpu.mem[0xA051] = 100;
        test_cpu.cpu.mem[0x8001] = 0x41;
        test_cpu.cpu.mem[0x8002] = 0xA0;
        test_cpu.cpu.y = 0x10;
        test_cpu.cpu.a = 0b1001_1100; // -100
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 56);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, true);

        // signed:   -100 - 100 = -200 -> 56  (1)0011_1000  | V -> 1
        // unsigned: 156 - 100 = 56                         | C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = true;
        test_cpu.cpu.mem[0x8000] = 0xE1; // SBC IndirectX
        test_cpu.cpu.mem[0x8001] = 0x04;
        test_cpu.cpu.x = 0x03;

        test_cpu.cpu.mem[0x0007] = 0x13; // Address of m

        test_cpu.cpu.mem[0x0013] = 100; // m
        test_cpu.cpu.a = 0b1001_1100;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 56);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, true);

        // signed:   -12 + -12 = -24            | V -> 0
        // unsigned: 244 + 244 = (1)_1110_1000  | C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0xF1; // SBC IndirectY
        test_cpu.cpu.mem[0x8001] = 0x33;
        test_cpu.cpu.mem[0x0033] = 0x73;
        test_cpu.cpu.y = 0x4;
        test_cpu.cpu.mem[0x0077] = 100; // m = 100
        test_cpu.cpu.a = 0b1001_1100; // -100
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 56);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, true);
    }

    #[test]
    fn test_and() {
        let mut test_cpu = MyCpu::default();

        //   0b1111_0100
        // & 0b0011_1010
        // = 0b0011_0000 -> N -> 0, Z -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x29; // AND Immediate
        test_cpu.cpu.mem[0x8001] = 0b1111_0100;
        test_cpu.cpu.a = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b0011_0000);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);

        //   0b1111_1111
        // & 0b0000_0000
        // = 0b0000_0000 -> N -> 0, Z -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x29; // AND Immediate
        test_cpu.cpu.mem[0x8001] = 0b1111_1111;
        test_cpu.cpu.a = 0;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, true);

        //   0b1111_1111
        // & 0b1111_1111
        // = 0b1111_1111 -> N -> 1, Z -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x29; // AND Immediate
        test_cpu.cpu.mem[0x8001] = 0b1111_1111;
        test_cpu.cpu.a = 0b1111_1111;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1111_1111);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        //   0b1100_1011
        // & 0b1001_0111
        // = 0b1000_0011 -> N -> 1, Z -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x29; // AND Immediate
        test_cpu.cpu.mem[0x8001] = 0b1100_1011;
        test_cpu.cpu.a = 0b1001_0111;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1000_0011);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        /* Other addressing modes */

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x25; // AND Zeropage
        test_cpu.cpu.mem[0x0091] = 0b1111_0100;
        test_cpu.cpu.mem[0x8001] = 0x91; // Address on zeropage
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1001_0100);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x35; // AND ZeropageX
        test_cpu.cpu.mem[0x0096] = 0b1111_0100;
        test_cpu.cpu.mem[0x8001] = 0x91; // Address on zeropage
        test_cpu.cpu.x = 5;
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1001_0100);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
        assert_eq!(test_cpu.cpu.overflow, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x2D; // AND Absolute
        test_cpu.cpu.mem[0x0096] = 0b1111_0100;
        test_cpu.cpu.mem[0x8001] = 0x96;
        test_cpu.cpu.mem[0x8002] = 0x00;
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1001_0100);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x3D; // AND AbsoluteX
        test_cpu.cpu.mem[0x6693] = 0b1111_0100;
        test_cpu.cpu.mem[0x8001] = 0x91;
        test_cpu.cpu.mem[0x8002] = 0x66;
        test_cpu.cpu.x = 2;
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1001_0100);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x39; // AND AbsoluteY
        test_cpu.cpu.mem[0x4251] = 0b1111_0100;
        test_cpu.cpu.mem[0x8001] = 0x41;
        test_cpu.cpu.mem[0x8002] = 0x42;
        test_cpu.cpu.y = 0x10;
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1001_0100);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x21; // AND IndirectX
        test_cpu.cpu.mem[0x8001] = 0x04;
        test_cpu.cpu.x = 0x03;

        test_cpu.cpu.mem[0x0007] = 0x13; // Address of m

        test_cpu.cpu.mem[0x0013] = 0b1111_0100; // m
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1001_0100);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x31; // AND IndirectY
        test_cpu.cpu.mem[0x8001] = 0x33;
        test_cpu.cpu.mem[0x0033] = 0x73;
        test_cpu.cpu.y = 0x4;
        test_cpu.cpu.mem[0x0077] = 0b1111_0100; // m
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1001_0100);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
    }

    #[test]
    fn test_eor() {
        let mut test_cpu = MyCpu::default();

        //   0b1111_0100
        // ^ 0b0011_1010
        // = 0b1100_1110 -> N -> 1, Z -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x49; // EOR Immediate
        test_cpu.cpu.mem[0x8001] = 0b1111_0100;
        test_cpu.cpu.a = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1100_1110);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        //   0b1111_1111
        // ^ 0b0000_0000
        // = 0b1111_1111 -> N -> 1, Z -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x49; // EOR Immediate
        test_cpu.cpu.mem[0x8001] = 0b1111_1111;
        test_cpu.cpu.a = 0;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 255);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        //   0b1111_1111
        // & 0b1111_1111
        // = 0b0000_0000 -> N -> 0, Z -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x49; // EOR Immediate
        test_cpu.cpu.mem[0x8001] = 0b1111_1111;
        test_cpu.cpu.a = 0b1111_1111;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, true);

        //   0b1100_1011
        // & 0b1001_0111
        // = 0b0101_1100 -> N -> 0, Z -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x49; // EOR Immediate
        test_cpu.cpu.mem[0x8001] = 0b1100_1011;
        test_cpu.cpu.a = 0b1001_0111;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b0101_1100);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.negative, false);

        /* Other addressing modes */

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x45; // EOR Zeropage
        test_cpu.cpu.mem[0x0091] = 0b1111_0100;
        test_cpu.cpu.mem[0x8001] = 0x91; // Address on zeropage
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b0110_0010);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x55; // EOR ZeropageX
        test_cpu.cpu.mem[0x0096] = 0b1111_0100;
        test_cpu.cpu.mem[0x8001] = 0x91; // Address on zeropage
        test_cpu.cpu.x = 5;
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b0110_0010);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x4D; // EOR Absolute
        test_cpu.cpu.mem[0x4296] = 0b1111_0100;
        test_cpu.cpu.mem[0x8001] = 0x96;
        test_cpu.cpu.mem[0x8002] = 0x42;
        test_cpu.cpu.a = 0b1001_0110; // -12
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b0110_0010);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x5D; // EOR AbsoluteX
        test_cpu.cpu.mem[0x4393] = 0b1111_0100;
        test_cpu.cpu.mem[0x8001] = 0x91;
        test_cpu.cpu.mem[0x8002] = 0x43;
        test_cpu.cpu.x = 2;
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b0110_0010);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x59; // EOR AbsoluteY
        test_cpu.cpu.mem[0x4451] = 0b1111_0100;
        test_cpu.cpu.mem[0x8001] = 0x41;
        test_cpu.cpu.mem[0x8002] = 0x44;
        test_cpu.cpu.y = 0x10;
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b0110_0010);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x41; // EOR IndirectX
        test_cpu.cpu.mem[0x8001] = 0x04;
        test_cpu.cpu.x = 0x03;

        test_cpu.cpu.mem[0x0007] = 0x13; // Address of m

        test_cpu.cpu.mem[0x0013] = 0b1111_0100; // m
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b0110_0010);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x51; // EOR IndirectY
        test_cpu.cpu.mem[0x8001] = 0x33;
        test_cpu.cpu.mem[0x0033] = 0x73;
        test_cpu.cpu.y = 0x4;
        test_cpu.cpu.mem[0x0077] = 0b1111_0100; // m
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b0110_0010);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
    }

    #[test]
    fn test_ora() {
        let mut test_cpu = MyCpu::default();

        //   0b1111_0100
        // | 0b0011_1010
        // = 0b1111_1110 -> N -> 1, Z -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x09; // ORA Immediate
        test_cpu.cpu.mem[0x8001] = 0b1111_0100;
        test_cpu.cpu.a = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1111_1110);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        //   0b1111_1111
        // | 0b0000_0000
        // = 0b1111_1111 -> N -> 1, Z -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x09; // ORA Immediate
        test_cpu.cpu.mem[0x8001] = 0b1111_1111;
        test_cpu.cpu.a = 0;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 255);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        //   0b1111_1111
        // | 0b1111_1111
        // = 0b1111_1111 -> N -> 1, Z -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x09; // ORA Immediate
        test_cpu.cpu.mem[0x8001] = 0b1111_1111;
        test_cpu.cpu.a = 0b1111_1111;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 255);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        //   0b1100_1011
        // | 0b1001_0111
        // = 0b1101_1111 -> N -> 1, Z -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x09; // ORA Immediate
        test_cpu.cpu.mem[0x8001] = 0b1100_1011;
        test_cpu.cpu.a = 0b1001_0111;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1101_1111);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.negative, true);

        /* Other addressing modes */

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x05; // ORA Zeropage
        test_cpu.cpu.mem[0x0091] = 0b1111_0100;
        test_cpu.cpu.mem[0x8001] = 0x91; // Address on zeropage
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1111_0110);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x15; // ORA ZeropageX
        test_cpu.cpu.mem[0x0096] = 0b1111_0100;
        test_cpu.cpu.mem[0x8001] = 0x91; // Address on zeropage
        test_cpu.cpu.x = 5;
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1111_0110);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x0D; // ORA Absolute
        test_cpu.cpu.mem[0x6496] = 0b1111_0100;
        test_cpu.cpu.mem[0x8001] = 0x96;
        test_cpu.cpu.mem[0x8002] = 0x64;
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1111_0110);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x1D; // ORA AbsoluteX
        test_cpu.cpu.mem[0x6593] = 0b1111_0100;
        test_cpu.cpu.mem[0x8001] = 0x91;
        test_cpu.cpu.mem[0x8002] = 0x65;
        test_cpu.cpu.x = 2;
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1111_0110);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x19; // ORA AbsoluteY
        test_cpu.cpu.mem[0x3451] = 0b1111_0100;
        test_cpu.cpu.mem[0x8001] = 0x41;
        test_cpu.cpu.mem[0x8002] = 0x34;
        test_cpu.cpu.y = 0x10;
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1111_0110);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x01; // ORA IndirectX
        test_cpu.cpu.mem[0x8001] = 0x04;
        test_cpu.cpu.x = 0x03;

        test_cpu.cpu.mem[0x0007] = 0x13; // Address of m

        test_cpu.cpu.mem[0x0013] = 0b1111_0100; // m
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1111_0110);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);

        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.mem[0x8000] = 0x11; // ORA IndirectY
        test_cpu.cpu.mem[0x8001] = 0x33;
        test_cpu.cpu.mem[0x0033] = 0x73;
        test_cpu.cpu.y = 0x4;
        test_cpu.cpu.mem[0x0077] = 0b1111_0100; // m
        test_cpu.cpu.a = 0b1001_0110;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1111_0110);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
    }

    #[test]
    fn test_asl() {
        let mut test_cpu = MyCpu::default();

        //  0b00111010
        // = 0b01110100 -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x0A; // ASL accumulator
        test_cpu.cpu.a = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b0111_0100);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);

        //  0b11111111
        // = 0b(1)11111110 -> N -> 1, Z -> 0, C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x0A; // ASL Accumulator
        test_cpu.cpu.a = 0b1111_1111;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1111_1110);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, true);

        //  0b0000_0000
        // = 0b0000_0000 -> N -> 0, Z -> 1, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x0A; // ALS Accumulator
        test_cpu.cpu.a = 0b0000_0000;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, true);
        assert_eq!(test_cpu.cpu.carry, false);

        /* Other addressing modes */

        //  0b01111010
        // = 0b1111_010(0) -> N -> 1, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x06; // ASL zeropage
        test_cpu.cpu.mem[0x8001] = 0x0076;
        test_cpu.cpu.mem[0x0076] = 0b0111_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x0076], 0b1111_0100);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x16; // ASL zeropageX
        test_cpu.cpu.mem[0x8001] = 0x0066;
        test_cpu.cpu.x = 3;
        test_cpu.cpu.mem[0x0069] = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x0069], 0b0111_0100);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x0E; // ASL absolute
        test_cpu.cpu.mem[0x8001] = 0x0060;
        test_cpu.cpu.mem[0x8002] = 0x24;
        test_cpu.cpu.mem[0x2460] = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x2460], 0b0111_0100);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x1E; // ASL absoluteX
        test_cpu.cpu.mem[0x8001] = 0x0060;
        test_cpu.cpu.mem[0x8002] = 0x24;
        test_cpu.cpu.x = 3;
        test_cpu.cpu.mem[0x2463] = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x2463], 0b0111_0100);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
    }
    #[test]
    fn test_lsr() {
        let mut test_cpu = MyCpu::default();

        //    0b00111010
        // = 0b00011101 -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x4A; // LSR accumulator
        test_cpu.cpu.a = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b000_11101);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);

        //  0b11111111
        // = 0b01111111(1) -> N -> 0, Z -> 0, C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x4A; // LSR Accumulator
        test_cpu.cpu.a = 0b1111_1111;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b0111_1111);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, true);

        //  0b0000_0000
        // = 0b0000_0000 -> N -> 0, Z -> 1, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x4A; // LSR Accumulator
        test_cpu.cpu.a = 0b0000_0000;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, true);
        assert_eq!(test_cpu.cpu.carry, false);

        /* Other addressing modes */

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x46; // LSR zeropage
        test_cpu.cpu.mem[0x8001] = 0x0076;
        test_cpu.cpu.mem[0x0076] = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x0076], 0b0001_1101);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x56; // LSR zeropageX
        test_cpu.cpu.mem[0x8001] = 0x0066;
        test_cpu.cpu.x = 3;
        test_cpu.cpu.mem[0x0069] = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x0069], 0b0001_1101);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x4E; // LSR absolute
        test_cpu.cpu.mem[0x8001] = 0x60;
        test_cpu.cpu.mem[0x8002] = 0x60;
        test_cpu.cpu.mem[0x6060] = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x6060], 0b0001_1101);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x5E; // LSR absoluteX
        test_cpu.cpu.mem[0x8001] = 0x60;
        test_cpu.cpu.mem[0x8002] = 0x55;
        test_cpu.cpu.x = 3;
        test_cpu.cpu.mem[0x5563] = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x5563], 0b0001_1101);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
    }

    #[test]
    fn test_ror() {
        let mut test_cpu = MyCpu::default();

        //   0b   0001_1101
        // = 0b(1)000_1110 -> N -> 1, Z -> 0, C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x6A; // ROR accumulator
        test_cpu.cpu.a = 0b0001_1101;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1000_1110);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, true);

        //   0b   11111111
        // = 0b(1)1111111 -> N -> 1, Z -> 0, C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x6A; // ROR Accumulator
        test_cpu.cpu.a = 0b1111_1111;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1111_1111);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, true);

        //   0b   00000000
        // = 0b(0)0000000 -> N -> 0, Z -> 1, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x6A; // LSR Accumulator
        test_cpu.cpu.a = 0b0000_0000;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, true);
        assert_eq!(test_cpu.cpu.carry, false);

        /* Other addressing modes */

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x66; // ROR zeropage
        test_cpu.cpu.mem[0x8001] = 0x0076;
        test_cpu.cpu.mem[0x0076] = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x0076], 0b0001_1101);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x76; // ROR zeropageX
        test_cpu.cpu.mem[0x8001] = 0x0066;
        test_cpu.cpu.x = 3;
        test_cpu.cpu.mem[0x0069] = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x0069], 0b0001_1101);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x6E; // ROR absolute
        test_cpu.cpu.mem[0x8001] = 0x60;
        test_cpu.cpu.mem[0x8002] = 0x22;
        test_cpu.cpu.mem[0x2260] = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x2260], 0b0001_1101);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x7E; // ROR absoluteX
        test_cpu.cpu.mem[0x8001] = 0x60;
        test_cpu.cpu.mem[0x8002] = 0x66;
        test_cpu.cpu.x = 3;
        test_cpu.cpu.mem[0x6663] = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x6663], 0b0001_1101);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
    }

    #[test]
    fn test_rol() {
        let mut test_cpu = MyCpu::default();

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x2A; // ROL accumulator
        test_cpu.cpu.a = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b0111_0100);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);

        //  0b11111111
        // = 0b11111111(1) -> N -> 1, Z -> 0, C -> 1
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x2A; // ROL Accumulator
        test_cpu.cpu.a = 0b1111_1111;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0b1111_1111);
        assert_eq!(test_cpu.cpu.negative, true);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, true);

        //  0b0000_0000
        // = 0b0000_000(0) -> N -> 0, Z -> 1, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x2A; // ROL Accumulator
        test_cpu.cpu.a = 0b0000_0000;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.a, 0);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, true);
        assert_eq!(test_cpu.cpu.carry, false);

        /* Other addressing modes */

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x26; // ROL zeropage
        test_cpu.cpu.mem[0x8001] = 0x0076;
        test_cpu.cpu.mem[0x0076] = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x0076], 0b0111_0100);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x36; // ROL zeropageX
        test_cpu.cpu.mem[0x8001] = 0x0066;
        test_cpu.cpu.x = 3;
        test_cpu.cpu.mem[0x0069] = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x0069], 0b0111_0100);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x2E; // ROL absolute
        test_cpu.cpu.mem[0x8001] = 0x60;
        test_cpu.cpu.mem[0x8002] = 0x77;
        test_cpu.cpu.mem[0x7760] = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x7760], 0b0111_0100);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);

        //  0b00111010
        // = 0b0111_010(0) -> N -> 0, Z -> 0, C -> 0
        test_cpu.cpu.pc = 0x8000;
        test_cpu.cpu.carry = false;
        test_cpu.cpu.mem[0x8000] = 0x3E; // ROL absoluteX
        test_cpu.cpu.mem[0x8001] = 0x60;
        test_cpu.cpu.mem[0x8002] = 0x7F;
        test_cpu.cpu.x = 3;
        test_cpu.cpu.mem[0x7F63] = 0b0011_1010;
        Instruction::do_instruction(&mut test_cpu, None);
        assert_eq!(test_cpu.cpu.mem[0x7F63], 0b0111_0100);
        assert_eq!(test_cpu.cpu.negative, false);
        assert_eq!(test_cpu.cpu.zero, false);
        assert_eq!(test_cpu.cpu.carry, false);
    }
}
