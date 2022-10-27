#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Cpu6502 {
    // TODO make these not all public
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u16,

    // Program status register
    pub carry: bool,
    pub zero: bool,
    pub irq_dis: bool,
    pub dec: bool,
    pub b: bool,
    pub overflow: bool,
    pub negative: bool,

    pub mem: [u8; 0xffff],
}

impl Cpu6502 {
    pub fn generate_from_rom(rom: &[u8]) -> Cpu6502 {
        // TODO this only works for NROM! Make some mapper init function.
        let train_avail = (rom[6] & 0b100) == 0b100;
        let data_offset: usize = if train_avail { 16 + 512 } else { 16 }; //Start of prg_rom
        let mut rom_data: [u8; 0xffff] = [0; 0xffff];
        let end = if rom[4] == 1 { 0xBFFF } else { 0xFFFF };
        for i in 0x8000..end {
            rom_data[i] = rom[i + data_offset - 0x8000];
        }

        if train_avail {
            for i in 0x7000..=0x71ff {
                rom_data[i] = rom[i - 0x7000 + 16];
            }
        }
        let reset_offset = if rom[4] == 1 { 16384 } else { 0 };
        let mut pc = ((rom_data[0xfffd - reset_offset] as u16) << 8)
            | rom_data[0xfffc - reset_offset] as u16; // Reset vector
        if pc >= 0xc000 && rom[4] == 1 {
            pc -= 16384;
        }
        Cpu6502 {
            a: 0,
            x: 0,
            y: 0,
            pc,
            sp: 0x01FF,

            carry: false,
            zero: false,
            irq_dis: false,
            dec: false,
            b: false,
            overflow: false,
            negative: false,

            mem: rom_data,
        }
    }

    pub fn stack_push(&mut self, value: u8) {
        self.mem[self.sp as usize] = value;
        self.sp -= 1;
        self.sp &= 0xff;
        self.sp |= 0x100;
    }

    pub fn stack_pop(&mut self) -> u8 {
        self.sp += 1;
        self.sp &= 0xff;
        self.sp |= 0x100;
        self.mem[self.sp as usize]
    }
}

impl Default for Cpu6502 {
    fn default() -> Cpu6502 {
        Cpu6502 {
            a: 0,
            x: 0,
            y: 0,
            pc: 0x8000,
            sp: 0x01FF,

            carry: false,
            zero: false,
            irq_dis: false,
            dec: false,
            b: false,
            overflow: false,
            negative: false,

            mem: [0; 0xffff],
        }
    }
}

#[cfg(test)]
mod cpu_tests {
    use crate::cpu::Cpu6502;

    #[test]
    fn cpu_init_test() {
        let mut rom: [u8; 0xffff] = [0; 0xffff]; // NOTE this ROM is only 32kb

        let prg_rom_size_in_16kb = 1;
        let chr_rom_size_in_8kb = 1;
        let flag6 = 0x00;
        let flag7 = 0x00;
        let flag8 = 0x01; // Set RAM to 8kb
        let flag9 = 0x00;
        let flag10 = 0x00;
        let header_rom: [u8; 16] = [
            b'N',
            b'E',
            b'S',
            0x1a,
            prg_rom_size_in_16kb,
            chr_rom_size_in_8kb,
            flag6,
            flag7,
            flag8,
            flag9,
            flag10,
            0x9,
            0x3E,
            0x20,
            0x00,
            0x00,
        ];
        for i in 0..16 {
            rom[i] = header_rom[i];
        }
        // Fill cartridge and rom with same data
        for i in 0..16384 * (prg_rom_size_in_16kb as u16) - 4 {
            let test_data = (i % 8).try_into().unwrap();
            rom[(16 + i) as usize] = test_data;
        }
        // Reset vector
        rom[(16 + 16380) as usize] = 0x00;
        rom[(16 + 16381) as usize] = 0x80;
        // IRQ vector
        rom[(16382 + 16) as usize] = 0x80;
        rom[(16383 + 16) as usize] = 0x80;
        for i in 0..8192 * (chr_rom_size_in_8kb as u16) {
            let test_data = (i % 8).try_into().unwrap();
            rom[(16 + 16384 * (prg_rom_size_in_16kb as u16) + i) as usize] = test_data;
        }

        let train_avail = (rom[6] & 0b100) == 0b100;
        let data_offset: usize = if train_avail { 16 + 512 } else { 16 }; //Start of prg_rom
        let mut rom_data: [u8; 0xffff] = [0; 0xffff];
        let end = if rom[4] == 1 { 0xBFFF } else { 0xFFFF };
        for i in 0x8000..end {
            rom_data[i] = rom[i + data_offset - 0x8000];
        }

        assert_eq!(
            Cpu6502::generate_from_rom(&rom),
            Cpu6502 {
                a: 0,
                x: 0,
                y: 0,
                pc: 0x8000,
                sp: 0x01FF,

                carry: false,
                zero: false,
                irq_dis: false,
                dec: false,
                b: false,
                overflow: false,
                negative: false,

                mem: rom_data,
            }
        );
    }

    #[test]
    fn stack_test() {
        let mut test_cpu = Cpu6502::default();
        test_cpu.stack_push(0x01);
        assert_eq!(test_cpu.sp, 0x01fe);
        assert_eq!(test_cpu.stack_pop(), 0x01);
        assert_eq!(test_cpu.sp, 0x01ff);

        // Overflow stack
        for _i in 0..256 {
            test_cpu.stack_push(0x01);
        }
        assert_eq!(test_cpu.sp, 0x01ff);
    }
}
