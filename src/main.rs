extern crate core;

mod cartridge;
mod cpu;
mod instructions;
mod instructions_test;
mod mapper;
mod mycpu;

use crate::cartridge::Cartridge;
use crate::cpu::Cpu6502;
use instructions::Instruction;

use crate::mapper::MapperType;
use log::LevelFilter;
use std::error::Error;
use tudelft_nes_ppu::{run_cpu, Cpu, Mirroring, Ppu};
use tudelft_nes_test::TestableCpu;

#[derive(Default)] // TODO delete this after files can be read (this is not useful for any implementation)
pub struct MyCpu {
    cpu: Cpu6502,
    cartridge: Cartridge,
    cycle: u8, // Cycles of instruction
    mapper: MapperType,
}

/// See docs of `Cpu` for explanations of each function
impl Cpu for MyCpu {
    fn tick(&mut self, ppu: &mut Ppu) -> Result<(), Box<dyn Error>> {
        if self.cycle != 0 {
            self.cycle -= 1;
            return Ok(());
        }

        Instruction::do_instruction(self, Some(ppu));
        self.cycle -= 1;
        Result::Ok(())
    }

    fn ppu_read_chr_rom(&self, offset: u16) -> u8 {
        self.cartridge.chr_rom_data[offset as usize]
    }

    fn non_maskable_interrupt(&mut self) {
        todo!()
    }
}

/// Implementing this trait allows automated tests to be run on your cpu.
/// The crate `tudelft-nes-test` contains all kinds of small and large scale
/// tests to find bugs in your cpu.
impl TestableCpu for MyCpu {
    fn get_cpu(rom: &[u8]) -> Result<Self, Box<dyn Error>> {
        let cartridge = Cartridge::generate_from_rom(rom);
        Ok(MyCpu {
            cpu: Cpu6502::new(rom),
            cartridge: Cartridge::generate_from_rom(rom),
            cycle: 0,
            mapper: MapperType::get_mapper(cartridge.mapper_number, cartridge),
        })
    }

    fn set_program_counter(&mut self, value: u16) {
        self.cpu.pc = value;
    }

    fn memory_read(&self, address: u16) -> u8 {
        // TODO make this a lot cleaner (went wrong due to borrowing issues with vectors)
        let mut prg_data: Vec<u8> = Vec::new();
        let mut chr_data: Vec<u8> = Vec::new();
        let mut prg_data2: Vec<u8> = Vec::new();
        let mut chr_data2: Vec<u8> = Vec::new();
        for i in 0..self.cartridge.prg_rom_data.len() {
            prg_data.push(self.cartridge.prg_rom_data[i]);
            prg_data2.push(self.cartridge.prg_rom_data[i]);
        }
        for i in 0..self.cartridge.chr_rom_data.len() {
            chr_data.push(self.cartridge.chr_rom_data[i]);
            chr_data2.push(self.cartridge.chr_rom_data[i]);
        }
        let cartridge = Cartridge {
            prg_rom_size_in_16kb: self.cartridge.prg_rom_size_in_16kb,
            chr_rom_size_in_8kb: self.cartridge.chr_rom_size_in_8kb,
            mapper_number: self.cartridge.mapper_number,
            mirroring_control: self.cartridge.mirroring_control,
            trainer_available: self.cartridge.trainer_available,
            battery_backed_prg_ram_available: self.cartridge.battery_backed_prg_ram_available,
            mirroring_mode: self.cartridge.mirroring_mode,
            nes_2_0_format: self.cartridge.nes_2_0_format,
            playchoice_10: self.cartridge.playchoice_10,
            vs_unisystem: self.cartridge.vs_unisystem,
            prg_ram_size_in_8kb: self.cartridge.prg_ram_size_in_8kb,
            tv_system_mode: self.cartridge.tv_system_mode,
            prg_ram_present: self.cartridge.prg_ram_present,
            bus_conflicts: self.cartridge.bus_conflicts,
            prg_rom_data: prg_data,
            chr_rom_data: chr_data,
        };
        let mut cpu: MyCpu = MyCpu {
            cpu: self.cpu,
            cartridge: Cartridge {
                prg_rom_size_in_16kb: self.cartridge.prg_rom_size_in_16kb,
                chr_rom_size_in_8kb: self.cartridge.chr_rom_size_in_8kb,
                mapper_number: self.cartridge.mapper_number,
                mirroring_control: self.cartridge.mirroring_control,
                trainer_available: self.cartridge.trainer_available,
                battery_backed_prg_ram_available: self.cartridge.battery_backed_prg_ram_available,
                mirroring_mode: self.cartridge.mirroring_mode,
                nes_2_0_format: self.cartridge.nes_2_0_format,
                playchoice_10: self.cartridge.playchoice_10,
                vs_unisystem: self.cartridge.vs_unisystem,
                prg_ram_size_in_8kb: self.cartridge.prg_ram_size_in_8kb,
                tv_system_mode: self.cartridge.tv_system_mode,
                prg_ram_present: self.cartridge.prg_ram_present,
                bus_conflicts: self.cartridge.bus_conflicts,
                prg_rom_data: prg_data2,
                chr_rom_data: chr_data2,
            },
            cycle: 0,
            mapper: MapperType::get_mapper(self.cartridge.mapper_number, cartridge),
        };
        cpu.data_read(None, address)
    }
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let cpu = MyCpu::default(); // TODO replace with get_cpu("filename")

    log::info!("running cpu");
    run_cpu(cpu, Mirroring::Horizontal);
}

#[cfg(test)]
mod tests {
    use crate::MyCpu;
    use log::LevelFilter;
    use tudelft_nes_test::{run_tests, TestSelector};

    /// This test fails in the template, since you didn't implement the cpu yet.
    #[test]
    fn test_all() {
        env_logger::builder().filter_level(LevelFilter::Info).init();

        if let Err(e) = run_tests::<MyCpu>(TestSelector::NROM_TEST) {
            log::error!("TEST FAILED: {e}");
            assert!(false);
        }
    }
}
