//! This module provides the bus, which connects the CPU, the Cartridge and the mapper.

use crate::{Cartridge, Cpu6502, Instruction, MapperType};
use std::error::Error;
use tudelft_nes_ppu::{Cpu, Mirroring, Ppu, PpuRegister};
use tudelft_nes_test::TestableCpu;

#[derive(Default)]
/// This struct triggers the CPU to execute the next instruction and it reads and writes data to the ROM by using the mapper struct.
pub struct Bus {
    /// CPU which is connected to the bus.
    pub cpu: Cpu6502,
    /// Cartridge which is connected to the bus.
    pub cartridge: Cartridge,
    /// The current number of remaining cycles for an instruction.
    pub cycle: u8,
    /// Mapper used for accessing memory.
    pub mapper: MapperType,
    /// Flag to stop the program
    pub jam: bool,
}

impl Bus {
    /// Writes data to the dedicated memory. Depending on the address, either the CPU's memory or the PPU's memory is written.
    ///
    /// # Arguments
    ///
    /// * `ppu` - Borrowed instance of PPU.
    /// * `addr` - The address to which the address shall be written.
    /// * `data` - The data itself that shall be written.
    ///
    /// Nothing is returned.
    pub fn data_write(&mut self, ppu: &mut Ppu, addr: u16, data: u8) {
        if addr >= 0x8000 {
            self.mapper
                .write_mapper(addr, data, &mut self.cpu.mem, &mut self.cartridge);
        } else if (0x2000..=0x2007).contains(&addr) {
            //ppu register
            match addr {
                0x2000 => ppu.write_ppu_register(PpuRegister::Controller, data),
                0x2001 => ppu.write_ppu_register(PpuRegister::Mask, data),
                0x2002 => ppu.write_ppu_register(PpuRegister::Status, data),
                0x2003 => ppu.write_ppu_register(PpuRegister::OamAddress, data),
                0x2004 => ppu.write_ppu_register(PpuRegister::OamData, data),
                0x2005 => ppu.write_ppu_register(PpuRegister::Scroll, data),
                0x2006 => ppu.write_ppu_register(PpuRegister::Address, data),
                0x2007 => ppu.write_ppu_register(PpuRegister::Data, data),
                _ => panic!("Out of ppu map bound"),
            }
        } else if (0x2008..=0x3fff).contains(&addr) {
            //ppu register mapping
            let remainder = (addr - 0x2000) % 8;
            match remainder {
                0 => ppu.write_ppu_register(PpuRegister::Controller, data),
                1 => ppu.write_ppu_register(PpuRegister::Mask, data),
                2 => ppu.write_ppu_register(PpuRegister::Status, data),
                3 => ppu.write_ppu_register(PpuRegister::OamAddress, data),
                4 => ppu.write_ppu_register(PpuRegister::OamData, data),
                5 => ppu.write_ppu_register(PpuRegister::Scroll, data),
                6 => ppu.write_ppu_register(PpuRegister::Address, data),
                7 => ppu.write_ppu_register(PpuRegister::Data, data),
                _ => panic!("Out of ppu map bound"),
            }
        } else if addr == 0x4014 {
            let page_start: u16 = (data as u16) << 8;
            if page_start >= 0x2000 {
                panic!("Page address is out of CPU RAM bound");
            }
            let mut oam_data: [u8; 256] = [0; 256];
            for i in page_start..=page_start + 255 {
                oam_data[(i - page_start) as usize] = self.data_read(ppu, i);
            }
            ppu.write_oam_dma(oam_data);
        } else if (0x0000..0x2000).contains(&addr) {
            let remainder = addr % 0x0800; // Mirror RAM address
            self.cpu.mem[remainder as usize] = data;
        } else {
            self.cpu.mem[addr as usize] = data; // NOTE this should not be called
        }
    }

    /// Reads data from the dedicated memory. Depending on the address, either the CPU's memory or the PPU's memory is read.
    ///
    /// # Arguments
    ///
    /// * `ppu` - Borrowed instance of PPU.
    /// * `addr` - The address to which the address shall be written.
    ///
    /// # Return
    /// * `u8` - read data byte of address.
    pub fn data_read(&self, ppu: &mut Ppu, addr: u16) -> u8 {
        if addr >= 0x8000 {
            self.cpu.mem[self.mapper.get_mapper_address(addr) as usize]
        } else if (0x2000..=0x2007).contains(&addr) {
            //ppu register
            match addr {
                0x2000 => ppu.read_ppu_register(PpuRegister::Controller, self),
                0x2001 => ppu.read_ppu_register(PpuRegister::Mask, self),
                0x2002 => ppu.read_ppu_register(PpuRegister::Status, self),
                0x2003 => ppu.read_ppu_register(PpuRegister::OamAddress, self),
                0x2004 => ppu.read_ppu_register(PpuRegister::OamData, self),
                0x2005 => ppu.read_ppu_register(PpuRegister::Scroll, self),
                0x2006 => ppu.read_ppu_register(PpuRegister::Address, self),
                0x2007 => ppu.read_ppu_register(PpuRegister::Data, self),
                _ => panic!("Out of ppu map bound"),
            }
        } else if (0x2008..=0x3fff).contains(&addr) {
            //ppu register mapping
            let remainder = (addr - 0x2000) % 8;
            match remainder {
                0 => ppu.read_ppu_register(PpuRegister::Controller, self),
                1 => ppu.read_ppu_register(PpuRegister::Mask, self),
                2 => ppu.read_ppu_register(PpuRegister::Status, self),
                3 => ppu.read_ppu_register(PpuRegister::OamAddress, self),
                4 => ppu.read_ppu_register(PpuRegister::OamData, self),
                5 => ppu.read_ppu_register(PpuRegister::Scroll, self),
                6 => ppu.read_ppu_register(PpuRegister::Address, self),
                7 => ppu.read_ppu_register(PpuRegister::Data, self),
                _ => panic!("Out of ppu map bound"),
            }
        } else if (0x0000..0x2000).contains(&addr) {
            let remainder = addr % 0x0800; // Mirror RAM address
            self.cpu.mem[remainder as usize]
        } else {
            self.cpu.mem[addr as usize] // NOTE this should not be called
        }
    }
}

/// See docs of `Cpu` for explanations of each function
impl Cpu for Bus {
    fn tick(&mut self, ppu: &mut Ppu) -> Result<(), Box<dyn Error>> {
        if !self.jam {
            if self.cycle != 0 {
                self.cycle -= 1;
                return Ok(());
            }

            Instruction::do_instruction(self, ppu);
            self.cycle -= 1;
        }
        Result::Ok(())
    }

    fn ppu_read_chr_rom(&self, offset: u16) -> u8 {
        self.cartridge.prg_rom_data[offset as usize]
    }

    fn non_maskable_interrupt(&mut self) {
        let mut dummy_ppu = Ppu::new(Mirroring::Horizontal); // Not used as nmi vector never in ppu range
        if !self.cpu.irq_dis {
            if self.cycle != 0 {
                self.cycle = 0;
            }

            let p = self.cpu.carry as u8 |
                (self.cpu.zero as u8) << 1 |
                (self.cpu.irq_dis as u8) << 2 |
                (self.cpu.dec as u8) << 3 |
                (self.cpu.b as u8) << 4 |
                0b0010_0000 | //ignore_flag
                (self.cpu.overflow as u8) << 6 |
                (self.cpu.negative as u8) << 7;

            self.cpu.stack_push((self.cpu.pc & 0xff) as u8);
            self.cpu.stack_push(((self.cpu.pc >> 8) & 0xff) as u8);
            self.cpu.stack_push(p);
            self.cpu.irq_dis = true;
            self.cpu.pc = (self.data_read(&mut dummy_ppu, 0xFFFA) as u16)
                & ((self.data_read(&mut dummy_ppu, 0xFFFB) as u16) << 8);
        }
    }
}

/// Implementing this trait allows automated tests to be run on your cpu.
/// The crate `tudelft-nes-test` contains all kinds of small and large scale
/// tests to find bugs in your cpu.
impl TestableCpu for Bus {
    fn get_cpu(rom: &[u8]) -> Result<Self, Box<dyn Error>> {
        let cartridge = Cartridge::generate_from_rom(rom);
        Ok(Bus {
            cpu: Cpu6502::generate_from_rom(rom),
            cartridge: Cartridge::generate_from_rom(rom),
            cycle: 0,
            mapper: MapperType::get_mapper(cartridge.mapper_number, cartridge),
            jam: false,
        })
    }

    fn set_program_counter(&mut self, value: u16) {
        self.cpu.pc = value;
    }

    fn memory_read(&self, address: u16) -> u8 {
        let mut dummy_ppu = Ppu::new(Mirroring::Horizontal);
        self.data_read(&mut dummy_ppu, address)
    }
}

#[cfg(test)]
mod mycpu_tests {
    use crate::Bus;
    use tudelft_nes_ppu::{Mirroring, Ppu};

    #[test]
    fn test_ram_mirror() {
        let mut test_cpu = Bus::default();
        let mut ppu = Ppu::new(Mirroring::Vertical);
        test_cpu.data_write(&mut ppu, 0x800, 0x69);
        test_cpu.data_write(&mut ppu, 0x801, 0x67);
        test_cpu.data_write(&mut ppu, 0x7ff, 0x66);
        assert_eq!(test_cpu.data_read(&mut ppu, 0x0000), 0x69); // Test mirrored address
        assert_eq!(test_cpu.data_read(&mut ppu, 0x0800), 0x69);
        assert_eq!(test_cpu.data_read(&mut ppu, 0x1000), 0x69); // Test mirrored address
        assert_eq!(test_cpu.data_read(&mut ppu, 0x0001), 0x67); // Test mirrored address
        assert_eq!(test_cpu.data_read(&mut ppu, 0x0801), 0x67);
        assert_eq!(test_cpu.data_read(&mut ppu, 0x1001), 0x67); // Test mirrored address
        assert_eq!(test_cpu.data_read(&mut ppu, 0x0fff), 0x66); // Test mirrored address
        assert_eq!(test_cpu.data_read(&mut ppu, 0x07ff), 0x66);
        assert_eq!(test_cpu.data_read(&mut ppu, 0x1fff), 0x66); // Test mirrored address
    }
    #[test]
    fn ppu_test() {
        let mut test_cpu = Bus::default();
        let mut ppu = Ppu::new(Mirroring::Vertical);
        test_cpu.data_write(&mut ppu, 0x2000, 0x69);
        assert_eq!(test_cpu.data_read(&mut ppu, 0x2008), 0x69); // Test mirrored address
        test_cpu.data_write(&mut ppu, 0x2004, 0x63);
        test_cpu.data_read(&mut ppu, 0x2000); // Dummy read
        assert_eq!(test_cpu.data_read(&mut ppu, 0x2004), 0x63);
    }
}
