use crate::{get_mapped_address, MyCpu};
use tudelft_nes_ppu::Ppu;

impl MyCpu {
    pub fn data_write(&mut self, _ppu: &Option<&mut Ppu>, addr: u16, data: u8) {
        if addr >= 0x8000 {
            // TODO NROM does not have a write option but other do so add that here
        } else {
            self.cpu.mem[addr as usize] = data;
        }
    }

    pub fn data_read(&mut self, _ppu: &Option<&mut Ppu>, addr: u16) -> u8 {
        if addr >= 0x8000 {
            self.cpu.mem[get_mapped_address(
                self.cartridge.mapper_number,
                addr,
                self.cartridge.prg_rom_size_in_16kb,
            ) as usize]
        } else {
            self.cpu.mem[addr as usize]
        }
    }
    pub fn data_read_borrow(&mut self, _ppu: &Option<&mut Ppu>, addr: &u16) -> u8 {
        if *addr >= 0x8000 {
            self.cpu.mem[get_mapped_address(
                self.cartridge.mapper_number,
                *addr,
                self.cartridge.prg_rom_size_in_16kb,
            ) as usize]
        } else {
            self.cpu.mem[*addr as usize]
        }
    }
}
