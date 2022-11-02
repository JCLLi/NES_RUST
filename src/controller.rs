//! This module provides the standard controller 
use tudelft_nes_ppu::{Buttons, Ppu};
#[derive(PartialEq, Default, Eq, Debug)]
/// This struct handles the user input controller
pub struct Controller {
    /// Register index to know which button is being retrieved.
    pub shift_register_index: u8,
    /// Strobe bit to continuously reset the shift register (returning only button 'a').
    pub strobe: bool,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            shift_register_index: 0,
            strobe: false,
        }
    }
    /// Set the strobe bit by inserting one byte.
    /// # Arguments
    ///
    /// * `data` - data byte written to the controller memory address.
    ///
    /// Nothing is returned.
    pub fn set_strobe(&mut self, data: u8) {
        self.strobe = data & 1 == 1;
    }
    /// Retrieve the button state and shift to the next button if strobe is low
    /// # Arguments
    ///
    /// * `ppu` - Borrowed instance of PPU.
    ///
    /// # Return
    /// * `u8` - Button state on LSB.
    pub fn get_controller_byte(&mut self, ppu: &mut Ppu) -> u8 {
        let button: Buttons = ppu.get_joypad_state();
        let mut ret: bool = button.get_by_index(self.shift_register_index);

        if self.strobe {
            ret = button.get_by_index(0); // Return 'a'
        }
        self.shift_register_index += 1;
        if self.shift_register_index == 8 {
            self.shift_register_index = 0;
        }
        ret as u8
    }
}
