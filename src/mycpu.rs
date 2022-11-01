use crate::MyCpu;
use tudelft_nes_ppu::{Ppu, PpuRegister};

impl MyCpu {
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

    pub fn data_read(&mut self, ppu: &mut Ppu, addr: u16) -> u8 {
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
#[cfg(test)]
mod mycpu_tests {
    use crate::MyCpu;
    use tudelft_nes_ppu::{Mirroring, Ppu};

    #[test]
    fn test_ram_mirror() {
        let mut test_cpu = MyCpu::default();
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
        let mut test_cpu = MyCpu::default();
        let mut ppu = Ppu::new(Mirroring::Vertical);
        test_cpu.data_write(&mut ppu, 0x2000, 0x69);
        assert_eq!(test_cpu.data_read(&mut ppu, 0x2008), 0x69); // Test mirrored address
        test_cpu.data_write(&mut ppu, 0x2004, 0x63);
        test_cpu.data_read(&mut ppu, 0x2000); // Dummy read
        assert_eq!(test_cpu.data_read(&mut ppu, 0x2004), 0x63);
    }
}
