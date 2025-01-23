#[derive(Clone, Copy)]

pub struct Bitboard(pub u64);

//Bitwise operation reminder: |= offset -> place a piece, &= !offset -> remove a piece

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

    pub fn move_bit(&mut self, from_offset: u32, to_offset: u32) {
        //The basis of moving a bit: checks if there is a 1 there, makes it a 0, and makes another field a 1.
        if self.check_bit(from_offset) {
            let rmv_bitmask = 1 << from_offset;
            self.0 &= !rmv_bitmask;

            //Thinking ahead, there could be some logic here to know if it's stepping on a 1?
            let add_bitmask = 1 << to_offset;
            self.0 |= add_bitmask;
        }
    }

    pub fn offset_to_coordinates(bit_offset: u32) -> (i32, i32) {
        let pos = bit_offset.trailing_zeros();
        let x = (pos % 8) as i32;
        let y = (pos / 8) as i32;
        (x, y)
        /* If better to iterate:
        for pos in 0..31 (or amnt of bits) {
        if (bit_offset << pos) & 1 == 1 {
        let x = i % 8;
        let y = pos / 8;
        }
        } */
    }
}
