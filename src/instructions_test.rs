#[cfg(test)]
mod instruction_tests {
    use crate::instructions::Instruction;
    use crate::mapper::get_mapped_address;
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
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
        assert_eq!(
            test_cpu.cpu.pc,
            get_mapped_address(
                test_cpu.cartridge.mapper_number,
                0x0766,
                test_cpu.cartridge.prg_rom_size_in_16kb
            ) - 1
        );
        assert_eq!(test_cpu.cpu.mem[test_cpu.cpu.sp as usize + 3], 0x00);
        assert_eq!(test_cpu.cpu.mem[test_cpu.cpu.sp as usize + 2], 0x80);
        assert_eq!(test_cpu.cpu.mem[test_cpu.cpu.sp as usize + 1], 0x22); // NOTE 6th bit is alwasy set to 1

        test_cpu.cpu.mem[0x0766] = 0x40; // RTI Implied
        test_cpu.cpu.pc += 1;
        test_cpu.cpu.zero = false; // Set a status flag
        Instruction::do_instruction(&mut test_cpu, &mut ppu);
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
