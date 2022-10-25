use crate::MyCpu;
use tudelft_nes_ppu::Ppu;

impl MyCpu {
    pub fn data_write(&mut self, _ppu: Option<&mut Ppu>, addr: u16, data: u8) {
        if addr >= 0x8000 {
            self.mapper
                .write_mapper(self.cartridge.prg_rom_size_in_16kb, addr, data);
        } else {
            self.cpu.mem[addr as usize] = data;
        }
    }

    pub fn data_read(&mut self, _ppu: Option<&mut Ppu>, addr: u16) -> u8 {
        if addr >= 0x8000 {
            self.cpu.mem[self.mapper.get_mapper_address(addr) as usize]
        } else {
            self.cpu.mem[addr as usize]
        }
    }
}
