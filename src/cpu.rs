//! This module provides the CPU, which stores the states of the registers, manages the stack memory, and holds the program memory.
use crate::Cartridge;
use crate::MapperType;

/// A struct representing the CPU, which stores the states of the registers, manages the stack memory, and holds the program memory.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Cpu6502 {
    /// Value in accumulator register
    pub a: u8,
    /// Value in register for index X
    pub x: u8,
    /// Value in register for index Y
    pub y: u8,
    /// Value of program counter
    pub pc: u16,
    /// Value of stack pointer
    pub sp: u16,

    // Program status register
    /// Indicates if carry flag is set
    pub carry: bool,
    /// Indicates if zero flag is set
    pub zero: bool,
    /// Indicates if interrupts are disabled
    pub irq_dis: bool,
    /// Indicates whether decimal mode is used
    pub dec: bool,
    /// Indicates if break "flag" is set
    pub b: bool,
    /// Indicates if overflow flag is set
    pub overflow: bool,
    /// Indicates if negative flag is set
    pub negative: bool,

    /// ROM currently held in the CPU
    pub mem: [u8; 0xffff + 1],
}

impl Cpu6502 {
    /// Write only on the CPU memory (without PPU or other peripherals).
    ///
    /// # Arguments
    ///
    /// * `cartridge` - Cartridge instance, needed for the mapper.
    /// * `mapper` - Mapper instance to know the correct mapper to use
    ///
    /// Nothing is returned.
    pub fn memory_write(
        &mut self,
        cartridge: &Cartridge,
        mapper: &mut MapperType,
        addr: u16,
        data: u8,
    ) {
        if addr >= 0x8000 {
            mapper.write_mapper(addr, data, &mut self.mem, cartridge);
        } else if (0x0000..0x2000).contains(&addr) {
            let remainder = addr % 0x0800; // Mirror RAM address
            self.mem[remainder as usize] = data;
        } else {
            self.mem[addr as usize] = data;
        }
    }
    /// Read registers only on the CPU memory (without PPU or other peripherals).
    ///
    /// # Arguments
    ///
    /// * `mapper` - Mapper instance to know the correct mapper to use
    ///
    /// # Return
    /// * `u8` - read data byte of address.
    pub fn memory_read(&self, mapper: &MapperType, addr: u16) -> u8 {
        if addr >= 0x8000 {
            self.mem[mapper.get_mapper_address(addr) as usize]
        } else if (0x0000..0x2000).contains(&addr) {
            let remainder = addr % 0x0800; // Mirror RAM address
            self.mem[remainder as usize]
        } else {
            self.mem[addr as usize]
        }
    }
    /// Creates and returns a Cpu6502 instance from an NES file provided as a vector of bytes.
    ///
    /// # Arguments
    ///
    /// * `rom` - A byte slice that contains the input .nes file.
    ///
    /// # Return
    /// * `Cpu6502` - a Cpu6502 instance containing the data of `rom`.
    pub fn generate_from_rom(rom: &[u8]) -> Cpu6502 {
        if rom[0] != b'N' || rom[1] != b'E' || rom[2] != b'S' || rom[3] != 0x1a {
            panic!("Not iNES format")
        }
        let train_avail = (rom[6] & 0b100) == 0b100;
        let data_offset: usize = if train_avail { 16 + 512 } else { 16 }; //Start of prg_rom
        let mut rom_data: [u8; 0xffff + 1] = [0; 0xffff + 1];
        let end = if rom[4] == 1 { 0xBFFF } else { 0xFFFF };
        for i in 0x8000..=end {
            rom_data[i] = rom[i + data_offset - 0x8000];
        }

        if train_avail {
            for i in 0x7000..=0x71ff {
                rom_data[i] = rom[i - 0x7000 + 16];
            }
        }
        let reset_offset = if rom[4] == 1 { 16384 } else { 0 };
        let pc = ((rom_data[0xfffd - reset_offset] as u16) << 8)
            | rom_data[0xfffc - reset_offset] as u16; // Reset vector
        Cpu6502 {
            a: 0,
            x: 0,
            y: 0,
            pc,
            sp: 0x01fd,

            carry: false,
            zero: false,
            irq_dis: true,
            dec: false,
            b: false,
            overflow: false,
            negative: false,

            mem: rom_data,
        }
    }

    /// Pushes given value onto the CPU's stack.
    ///
    /// # Arguments
    ///
    /// * `self` - CPU's own instance needed to access the stack pointer.
    /// * `value` - The value that shall be pushed onto the stack.
    ///
    /// No return value.
    pub fn stack_push(&mut self, value: u8) {
        self.mem[self.sp as usize] = value;
        self.sp -= 1;
        self.sp &= 0xff;
        self.sp |= 0x100;
    }

    /// Pops the top value from the CPU's stack.
    ///
    /// # Arguments
    ///
    /// * `self` - CPU's own instance needed to access the stack pointer.
    ///
    /// # Return
    /// * `u8` - The value that has been popped from the stack.
    pub fn stack_pop(&mut self) -> u8 {
        self.sp += 1;
        self.sp &= 0xff;
        self.sp |= 0x100;
        self.mem[self.sp as usize]
    }
}

impl Default for Cpu6502 {
    /// Implements the trait `Default` for Cpu6502 which returns a default instance of itself. All registers are set to false, except for pc and sp, which are also put their default state. The memory is filled with zeros.
    ///
    /// # Return
    /// * `Cpu6502` - Instance of the struct in the default state.
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

            mem: [0; 0xffff + 1],
        }
    }
}

#[cfg(test)]
mod cpu_tests {
    use crate::cpu::Cpu6502;

    #[test]
    fn cpu_init_test() {
        let mut rom: [u8; 0xffff + 1] = [0; 0xffff + 1]; // NOTE this ROM is only 32kb

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
        let mut rom_data: [u8; 0xffff + 1] = [0; 0xffff + 1];
        let end = if rom[4] == 1 { 0xBFFF } else { 0xFFFF };
        for i in 0x8000..=end {
            rom_data[i] = rom[i + data_offset - 0x8000];
        }

        assert_eq!(
            Cpu6502::generate_from_rom(&rom),
            Cpu6502 {
                a: 0,
                x: 0,
                y: 0,
                pc: 0x8000,
                sp: 0x01fd,

                carry: false,
                zero: false,
                irq_dis: true,
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
