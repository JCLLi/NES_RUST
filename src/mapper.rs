pub fn get_mapped_address(mapper_number: u8, addr: u16, prg_rom_size_in_16kb: u8) -> u16 {
    match mapper_number {
        0 => {
            // NROM
            if addr < 0x8000 {
                return addr;
            }
            if prg_rom_size_in_16kb == 2 {
                addr
            } else {
                if addr < 0xc000 {
                    addr
                } else {
                    addr - 16384
                }
            }
        }
        _ => panic!("Mapper type not implemented yet"),
    }
}

#[cfg(test)]
mod mapper_tests {
    use crate::mapper::get_mapped_address;

    #[test]
    fn test_mapper() {
        assert_eq!(get_mapped_address(0, 0x8000, 1), 0x8000);
        assert_eq!(get_mapped_address(0, 0xC000, 1), 0x8000);
        assert_eq!(get_mapped_address(0, 0xC000, 2), 0xC000);
    }
}
