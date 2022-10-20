mod mapper;

use log::LevelFilter;
use std::error::Error;
use tudelft_nes_ppu::{run_cpu, Cpu, Mirroring, Ppu};
use tudelft_nes_test::TestableCpu;

pub struct MyCpu {}

/// See docs of `Cpu` for explanations of each function
impl Cpu for MyCpu {
    fn tick(&mut self, _ppu: &mut Ppu) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn ppu_read_chr_rom(&self, _offset: u16) -> u8 {
        todo!()
    }

    fn non_maskable_interrupt(&mut self) {
        todo!()
    }
}

/// Implementing this trait allows automated tests to be run on your cpu.
/// The crate `tudelft-nes-test` contains all kinds of small and large scale
/// tests to find bugs in your cpu.
impl TestableCpu for MyCpu {
    fn get_cpu(_rom: &[u8]) -> Result<Self, Box<dyn Error>> {
        todo!()
    }

    fn set_program_counter(&mut self, _value: u16) {
        todo!()
    }

    fn memory_read(&self, _address: u16) -> u8 {
        todo!()
    }
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let cpu = MyCpu {};

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

        if let Err(e) = run_tests::<MyCpu>(TestSelector::DEFAULT) {
            log::error!("TEST FAILED: {e}");
            assert!(false);
        }
    }
}
