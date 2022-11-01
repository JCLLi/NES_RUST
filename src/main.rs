mod bus;
mod cartridge;
mod cpu;
mod instructions;
mod instructions_test;
mod mapper;

use crate::cartridge::Cartridge;
use crate::cpu::Cpu6502;
use instructions::Instruction;
use mapper::MapperType;

use crate::bus::Bus;
use log::LevelFilter;
use tudelft_nes_ppu::{run_cpu_headless_for, Mirroring};
use tudelft_nes_test::TestableCpu;

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    pub const FILE: &[u8] = include_bytes!("../test_roms/nestest.nes");

    let mut cpu = Bus::get_cpu(FILE).expect("In main error");

    run_cpu_headless_for(&mut cpu, Mirroring::Horizontal, 100000).expect("File reading failed");
}

#[cfg(test)]
mod tests {
    use crate::Bus;
    use log::LevelFilter;
    use tudelft_nes_test::{run_tests, TestSelector};

    #[test]
    fn test_all() {
        env_logger::builder().filter_level(LevelFilter::Info).init();

        if let Err(e) = run_tests::<Bus>(TestSelector::OFFICIAL_INSTRS) {
            log::error!("TEST FAILED: {e}");
            assert!(false);
        }
    }
}
