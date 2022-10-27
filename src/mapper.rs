use crate::Cartridge;

#[derive(Debug, PartialEq, Eq)]
pub enum MapperType {
    Nrom {
        prg_rom_size_in_16kb: u8,
    },
    MMC1 {
        // Settings
        mirroring: u8,
        prg_rom_bank_mode: u8,
        chr_rom_bank_mode: bool,
        chr_bank0: u8,
        chr_bank1: u8,
        prg_bank: u8,
        mmc1b: bool,

        shift_register: u8,
        amount_shifted: u8,
    },
}
impl Default for MapperType {
    fn default() -> Self {
        MapperType::Nrom {
            prg_rom_size_in_16kb: 1,
        }
    }
}
impl MapperType {
    pub fn get_mapper(mapper_number: u8, cartridge: Cartridge) -> MapperType {
        match mapper_number {
            0 => MapperType::Nrom {
                prg_rom_size_in_16kb: cartridge.prg_rom_size_in_16kb,
            },
            1 => MapperType::MMC1 {
                mirroring: 0,
                prg_rom_bank_mode: 0,
                chr_rom_bank_mode: false,
                chr_bank0: 0,
                chr_bank1: 0,
                prg_bank: 0,
                mmc1b: false,
                // Stored values
                shift_register: 0,
                amount_shifted: 0,
            },
            _ => panic!("Mapper not implemented/known"),
        }
    }
    pub fn get_mapper_address(&self, addr: u16) -> u16 {
        match self {
            MapperType::Nrom {
                prg_rom_size_in_16kb,
            } => {
                if addr < 0x8000 {
                    return addr;
                }
                if *prg_rom_size_in_16kb > 2 {
                    panic!("Program ROM is too big for NROM")
                }
                if *prg_rom_size_in_16kb == 2 {
                    return addr;
                }
                if addr < 0xc000 {
                    addr
                } else {
                    addr - 16384
                }
            }
            MapperType::MMC1 { .. } => addr,
        }
    }
    pub fn write_mapper(
        &mut self,
        addr: u16,
        data: u8,
        mem: &mut [u8; 0xffff],
        cart: &mut Cartridge,
    ) {
        //You can't write to read-only memory
        match self {
            MapperType::Nrom { .. } => {}
            MapperType::MMC1 {
                ref mut mirroring,
                ref mut prg_rom_bank_mode,
                ref mut chr_rom_bank_mode,
                ref mut chr_bank0,
                ref mut chr_bank1,
                ref mut prg_bank,
                ref mut mmc1b,
                // Stored values
                ref mut shift_register,
                ref mut amount_shifted,
            } => {
                let mut prg_bank_changed = false;
                let mut chr_bank_changed = false;
                *shift_register >>= 1;
                *shift_register |= (data & 0x1) << 4;
                *amount_shifted += 1;
                if data & 0x80 == 0x80 {
                    *shift_register = 0;
                    *amount_shifted = 0;
                    *prg_rom_bank_mode = 0b11;
                    //return;
                }
                if *amount_shifted == 5 {
                    if (0x8000..=0x9fff).contains(&addr) {
                        *mirroring = *shift_register & 0b11;
                        *prg_rom_bank_mode = (*shift_register & 0b1100) >> 2;
                        *chr_rom_bank_mode = (*shift_register & 0b1_0000) == 0b1_0000;
                        prg_bank_changed = true;
                        chr_bank_changed = true;
                    } else if (0xa000..=0xbfff).contains(&addr) {
                        *chr_bank0 = *shift_register & 0b1_1111;
                        chr_bank_changed = true;
                    } else if (0xc000..=0xdfff).contains(&addr) {
                        *chr_bank1 = *shift_register & 0b1_1111;
                        chr_bank_changed = true;
                    } else if (0xe000..=0xffff).contains(&addr) {
                        *prg_bank = *shift_register & 0b1111;
                        *mmc1b = *shift_register & 0b1_0000 == 0b1_0000;
                        prg_bank_changed = true;
                    }
                    *shift_register = 0;
                    *amount_shifted = 0;
                    if prg_bank_changed {
                        //for i in 0x8000..0xffff {
                        for (i, mem_ref) in mem.iter_mut().enumerate().take(0xffff).skip(0x8000) {
                            if *prg_rom_bank_mode == 0 {
                                if i < 0xc000 {
                                    *mem_ref = cart.prg_rom_data[(i - 0x8000
                                        + 16384 * (*prg_bank & 0b1110) as usize)
                                        as usize];
                                } else {
                                    *mem_ref = cart.prg_rom_data[(i - 0x8000
                                        + 16384 * (*prg_bank & 0b1111) as usize)
                                        as usize];
                                }
                                //mem[i] = cart.prg_rom_data[(i-0x8000 + 32768 * (prg_bank & 0b1110) as usize) as usize]; // TODO see if this can work too
                            } else if *prg_rom_bank_mode == 1 {
                                if i < 0xc000 {
                                    *mem_ref = cart.prg_rom_data[i - 0x8000]; // Set to first PRG bank
                                } else {
                                    *mem_ref = cart.prg_rom_data
                                        [(i - 0x8000 + 16384 * *prg_bank as usize) as usize];
                                }
                            } else if i < 0xc000 {
                                *mem_ref = cart.prg_rom_data
                                    [(i - 0x8000 + 16384 * *prg_bank as usize) as usize];
                            } else {
                                *mem_ref = cart.prg_rom_data
                                    [cart.prg_rom_data.len() - 16834 + (i - 0xc000)];
                                // Set to last PRG bank
                            }
                        }
                    }
                    if chr_bank_changed {
                        // TODO this has to be implemented
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod mapper_tests {
    use crate::MapperType::{Nrom, MMC1};
    use crate::{Cartridge, Cpu6502, MapperType, MyCpu};

    #[test]
    fn test_nrom() {
        let cart = Cartridge::default();
        let mapper0 = MapperType::get_mapper(0, cart);
        assert_eq!(
            mapper0,
            Nrom {
                prg_rom_size_in_16kb: 1
            }
        );
        assert_eq!(mapper0.get_mapper_address(0xC000), 0x8000);
        // Make cartridge with 32 kb
        let cart_32kb = Cartridge {
            prg_rom_size_in_16kb: 2,
            chr_rom_size_in_8kb: 1,
            mapper_number: 0,
            mirroring_control: false,
            trainer_available: false,
            battery_backed_prg_ram_available: false,
            mirroring_mode: false,
            nes_2_0_format: false,
            playchoice_10: false,
            vs_unisystem: false,
            prg_ram_size_in_8kb: 1,
            tv_system_mode: false,
            prg_ram_present: false,
            bus_conflicts: false,
            prg_rom_data: Vec::new(), // Dont need this
            chr_rom_data: Vec::new(), // Don't need this
        };
        let mapper0_32kb = MapperType::get_mapper(0, cart_32kb);
        assert_eq!(mapper0_32kb.get_mapper_address(0x8000), 0x8000);
        assert_eq!(mapper0_32kb.get_mapper_address(0xC000), 0xC000);
    }

    #[test]
    fn test_mmc1_write_prg() {
        let mut rom_data: [u8; 0xffff] = [0; 0xffff];
        for i in 0x8000..0xffff {
            rom_data[i] = (i - 0x8000) as u8;
        }

        let test_cpu = Cpu6502 {
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
        };

        let prg_rom_size_in_16kb = 16;
        let chr_rom_size_in_8kb = 1;
        let mut prg_data: Vec<u8> = Vec::new();
        let mut chr_data: Vec<u8> = Vec::new();
        // Fill cartridge and rom with same data
        for i in 0..16384 * prg_rom_size_in_16kb as u32 {
            prg_data.push((i % 8) as u8);
        }
        for i in 0..8192 * (chr_rom_size_in_8kb as u16) {
            chr_data.push((i % 8) as u8);
        }

        let cart = Cartridge {
            prg_rom_size_in_16kb,
            chr_rom_size_in_8kb,

            mapper_number: 1,
            mirroring_control: false,
            battery_backed_prg_ram_available: false,
            trainer_available: false,
            mirroring_mode: false,

            vs_unisystem: false,
            playchoice_10: false,
            nes_2_0_format: false,

            prg_ram_size_in_8kb: 1,

            tv_system_mode: false, // Not used for now

            prg_ram_present: false,
            bus_conflicts: false,

            prg_rom_data: prg_data,
            chr_rom_data: chr_data,
        };

        let mut mycpu = MyCpu {
            cpu: test_cpu,
            cartridge: cart,
            cycle: 0,
            mapper: MMC1 {
                mirroring: 0,
                prg_rom_bank_mode: 0,
                chr_rom_bank_mode: true,
                chr_bank0: 0,
                chr_bank1: 0,
                prg_bank: 0,
                mmc1b: false,
                // Stored values
                shift_register: 0,
                amount_shifted: 0,
            },
        };
        mycpu
            .mapper
            .write_mapper(0xa000, 0b0, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        mycpu
            .mapper
            .write_mapper(0xa000, 0b0, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        match mycpu.mapper {
            MapperType::MMC1 {
                mirroring: _,
                prg_rom_bank_mode: _,
                chr_rom_bank_mode: _,
                chr_bank0: _,
                chr_bank1: _,
                prg_bank,
                mmc1b,
                // Stored values
                shift_register,
                amount_shifted,
            } => {
                assert_eq!(amount_shifted, 2);
                assert_eq!(shift_register, 0b0_0000);
                assert_eq!(prg_bank, 0);
                assert_eq!(mmc1b, false);
            }
            _ => panic!("This is a mmc1 test"),
        }
        mycpu
            .mapper
            .write_mapper(0xa000, 0b1, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        mycpu
            .mapper
            .write_mapper(0xa000, 0b1, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        mycpu
            .mapper
            .write_mapper(0xe000, 0b1, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        match mycpu.mapper {
            MapperType::MMC1 {
                mirroring: _,
                prg_rom_bank_mode: _,
                chr_rom_bank_mode: _,
                chr_bank0: _,
                chr_bank1: _,
                prg_bank,
                mmc1b,
                // Stored values
                shift_register,
                amount_shifted,
            } => {
                assert_eq!(amount_shifted, 0);
                assert_eq!(shift_register, 0b0);
                assert_eq!(prg_bank, 0b1100);
                assert_eq!(mmc1b, true);
            }
            _ => panic!("This is a mmc1 test"),
        }
    }
    #[test]
    fn test_mmc1_write_chr_bank1() {
        let mut rom_data: [u8; 0xffff] = [0; 0xffff];
        for i in 0x8000..0xffff {
            rom_data[i] = (i - 0x8000) as u8;
        }

        let test_cpu = Cpu6502 {
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
        };

        let prg_rom_size_in_16kb = 16;
        let chr_rom_size_in_8kb = 1;
        let mut prg_data: Vec<u8> = Vec::new();
        let mut chr_data: Vec<u8> = Vec::new();
        // Fill cartridge and rom with same data
        for i in 0..16384 * (prg_rom_size_in_16kb as u32) {
            prg_data.push(i as u8);
        }
        for i in 0..8192 * (chr_rom_size_in_8kb as u16) {
            chr_data.push(i as u8);
        }

        let cart = Cartridge {
            prg_rom_size_in_16kb,
            chr_rom_size_in_8kb,

            mapper_number: 1,
            mirroring_control: false,
            battery_backed_prg_ram_available: false,
            trainer_available: false,
            mirroring_mode: false,

            vs_unisystem: false,
            playchoice_10: false,
            nes_2_0_format: false,

            prg_ram_size_in_8kb: 1,

            tv_system_mode: false, // Not used for now

            prg_ram_present: false,
            bus_conflicts: false,

            prg_rom_data: prg_data,
            chr_rom_data: chr_data,
        };

        let mut mycpu = MyCpu {
            cpu: test_cpu,
            cartridge: cart,
            cycle: 0,
            mapper: MMC1 {
                mirroring: 0,
                prg_rom_bank_mode: 0,
                chr_rom_bank_mode: true,
                chr_bank0: 0,
                chr_bank1: 0,
                prg_bank: 0,
                mmc1b: false,
                // Stored values
                shift_register: 0,
                amount_shifted: 0,
            },
        };
        mycpu
            .mapper
            .write_mapper(0xa000, 0b0, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        mycpu
            .mapper
            .write_mapper(0xa000, 0b0, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        match mycpu.mapper {
            MapperType::MMC1 {
                mirroring,
                prg_rom_bank_mode,
                chr_rom_bank_mode,
                chr_bank0: _,
                chr_bank1: _,
                prg_bank: _,
                mmc1b: _,
                // Stored values
                shift_register,
                amount_shifted,
            } => {
                assert_eq!(amount_shifted, 2);
                assert_eq!(shift_register, 0b0_0000);
                assert_eq!(mirroring, 0);
                assert_eq!(prg_rom_bank_mode, 0);
                assert_eq!(chr_rom_bank_mode, true);
            }
            _ => panic!("This is a mmc1 test"),
        }
        mycpu
            .mapper
            .write_mapper(0xa000, 0b1, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        mycpu
            .mapper
            .write_mapper(0xa000, 0b1, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        mycpu
            .mapper
            .write_mapper(0xa000, 0b1, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        match mycpu.mapper {
            MapperType::MMC1 {
                mirroring: _,
                prg_rom_bank_mode: _,
                chr_rom_bank_mode: _,
                chr_bank0,
                chr_bank1: _,
                prg_bank: _,
                mmc1b: _,
                // Stored values
                shift_register,
                amount_shifted,
            } => {
                assert_eq!(amount_shifted, 0);
                assert_eq!(shift_register, 0b0);
                assert_eq!(chr_bank0, 0b11100);
            }
            _ => panic!("This is a mmc1 test"),
        }
    }
    #[test]
    fn test_mmc1_write_chr_bank0() {
        let mut rom_data: [u8; 0xffff] = [0; 0xffff];
        for i in 0x8000..0xffff {
            rom_data[i] = (i - 0x8000) as u8;
        }

        let test_cpu = Cpu6502 {
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
        };

        let prg_rom_size_in_16kb = 1;
        let chr_rom_size_in_8kb = 1;
        let mut prg_data: Vec<u8> = Vec::new();
        let mut chr_data: Vec<u8> = Vec::new();
        // Fill cartridge and rom with same data
        for i in 0..16384 * (prg_rom_size_in_16kb as u16) {
            prg_data.push(i as u8);
        }
        for i in 0..8192 * (chr_rom_size_in_8kb as u16) {
            chr_data.push(i as u8);
        }

        let cart = Cartridge {
            prg_rom_size_in_16kb,
            chr_rom_size_in_8kb,

            mapper_number: 1,
            mirroring_control: false,
            battery_backed_prg_ram_available: false,
            trainer_available: false,
            mirroring_mode: false,

            vs_unisystem: false,
            playchoice_10: false,
            nes_2_0_format: false,

            prg_ram_size_in_8kb: 1,

            tv_system_mode: false, // Not used for now

            prg_ram_present: false,
            bus_conflicts: false,

            prg_rom_data: prg_data,
            chr_rom_data: chr_data,
        };

        let mut mycpu = MyCpu {
            cpu: test_cpu,
            cartridge: cart,
            cycle: 0,
            mapper: MMC1 {
                mirroring: 0,
                prg_rom_bank_mode: 0,
                chr_rom_bank_mode: true,
                chr_bank0: 0,
                chr_bank1: 0,
                prg_bank: 0,
                mmc1b: false,
                // Stored values
                shift_register: 0,
                amount_shifted: 0,
            },
        };
        mycpu
            .mapper
            .write_mapper(0xa000, 0b0, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        mycpu
            .mapper
            .write_mapper(0xa000, 0b0, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        match mycpu.mapper {
            MapperType::MMC1 {
                mirroring,
                prg_rom_bank_mode,
                chr_rom_bank_mode,
                chr_bank0: _,
                chr_bank1: _,
                prg_bank: _,
                mmc1b: _,
                // Stored values
                shift_register,
                amount_shifted,
            } => {
                assert_eq!(amount_shifted, 2);
                assert_eq!(shift_register, 0b0_0000);
                assert_eq!(mirroring, 0);
                assert_eq!(prg_rom_bank_mode, 0);
                assert_eq!(chr_rom_bank_mode, true);
            }
            _ => panic!("This is a mmc1 test"),
        }
        mycpu
            .mapper
            .write_mapper(0xa000, 0b1, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        mycpu
            .mapper
            .write_mapper(0xa000, 0b1, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        mycpu
            .mapper
            .write_mapper(0xa000, 0b1, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        match mycpu.mapper {
            MapperType::MMC1 {
                mirroring: _,
                prg_rom_bank_mode: _,
                chr_rom_bank_mode: _,
                chr_bank0,
                chr_bank1: _,
                prg_bank: _,
                mmc1b: _,
                // Stored values
                shift_register,
                amount_shifted,
            } => {
                assert_eq!(amount_shifted, 0);
                assert_eq!(shift_register, 0b0);
                assert_eq!(chr_bank0, 0b11100);
            }
            _ => panic!("This is a mmc1 test"),
        }
    }
    #[test]
    fn test_mmc1_write_control() {
        let mut rom_data: [u8; 0xffff] = [0; 0xffff];
        for i in 0x8000..0xffff {
            rom_data[i] = (i - 0x8000) as u8;
        }

        let test_cpu = Cpu6502 {
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
        };

        let prg_rom_size_in_16kb = 2;
        let chr_rom_size_in_8kb = 1;
        let mut prg_data: Vec<u8> = Vec::new();
        let mut chr_data: Vec<u8> = Vec::new();
        // Fill cartridge and rom with same data
        for i in 0..16384 * (prg_rom_size_in_16kb as u16) {
            prg_data.push(i as u8);
        }
        for i in 0..8192 * (chr_rom_size_in_8kb as u16) {
            chr_data.push(i as u8);
        }

        let cart = Cartridge {
            prg_rom_size_in_16kb,
            chr_rom_size_in_8kb,

            mapper_number: 1,
            mirroring_control: false,
            battery_backed_prg_ram_available: false,
            trainer_available: false,
            mirroring_mode: false,

            vs_unisystem: false,
            playchoice_10: false,
            nes_2_0_format: false,

            prg_ram_size_in_8kb: 1,

            tv_system_mode: false, // Not used for now

            prg_ram_present: false,
            bus_conflicts: false,

            prg_rom_data: prg_data,
            chr_rom_data: chr_data,
        };

        let mut mycpu = MyCpu {
            cpu: test_cpu,
            cartridge: cart,
            cycle: 0,
            mapper: MMC1 {
                mirroring: 0,
                prg_rom_bank_mode: 0,
                chr_rom_bank_mode: true,
                chr_bank0: 0,
                chr_bank1: 0,
                prg_bank: 0,
                mmc1b: false,
                // Stored values
                shift_register: 0,
                amount_shifted: 0,
            },
        };
        mycpu
            .mapper
            .write_mapper(0x8000, 0b1111, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        mycpu
            .mapper
            .write_mapper(0x8000, 0b0111, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        match mycpu.mapper {
            MapperType::MMC1 {
                mirroring,
                prg_rom_bank_mode,
                chr_rom_bank_mode,
                chr_bank0: _,
                chr_bank1: _,
                prg_bank: _,
                mmc1b: _,
                // Stored values
                shift_register,
                amount_shifted,
            } => {
                assert_eq!(amount_shifted, 2);
                assert_eq!(shift_register, 0b1_1000);
                assert_eq!(mirroring, 0);
                assert_eq!(prg_rom_bank_mode, 0);
                assert_eq!(chr_rom_bank_mode, true);
            }
            _ => panic!("This is a mmc1 test"),
        }
        mycpu
            .mapper
            .write_mapper(0x8000, 0b1, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        mycpu
            .mapper
            .write_mapper(0x8000, 0b1, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        mycpu
            .mapper
            .write_mapper(0x8000, 0b0, &mut mycpu.cpu.mem, &mut mycpu.cartridge);
        match mycpu.mapper {
            MapperType::MMC1 {
                mirroring,
                prg_rom_bank_mode,
                chr_rom_bank_mode,
                chr_bank0: _,
                chr_bank1: _,
                prg_bank: _,
                mmc1b: _,
                // Stored values
                shift_register,
                amount_shifted,
            } => {
                assert_eq!(amount_shifted, 0);
                assert_eq!(shift_register, 0b0);
                assert_eq!(mirroring, 0b11);
                assert_eq!(prg_rom_bank_mode, 0b11);
                assert_eq!(chr_rom_bank_mode, false);
            }
            _ => panic!("This is a mmc1 test"),
        }
    }
}
