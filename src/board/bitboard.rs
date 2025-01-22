#[derive(Clone, Copy)]

pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn print_bitboard(&self) {
        let bin_str: String = format!("{:064b}", self.value()).chars().rev().collect();
        let out_str = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            &bin_str[56..64],
            &bin_str[48..56],
            &bin_str[40..48],
            &bin_str[32..40],
            &bin_str[24..32],
            &bin_str[16..24],
            &bin_str[8..16],
            &bin_str[0..8],
        );

        println!("{}", out_str);
    }

    pub fn check_bit(&self, bit_offset: u32) -> bool {
        let bitmask = 1 << bit_offset;
        (self.0 & bitmask) != 0
    }

    pub fn bit_offset_to_coordinates(bit_offset: i32) -> (i32, i32) {
        let column = bit_offset % 8;
        let rank = 7 - bit_offset / 8;

        return (column, rank);
    }
}
