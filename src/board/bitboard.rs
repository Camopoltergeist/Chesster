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

    pub fn bit_offset_to_coordinates(bit_offset: i32) -> (i32, i32) {
        let column = bit_offset % 8;
        let rank = bit_offset / 8;

        return (column, rank);
    }

    pub fn get_rank_mask(rank: i32) -> Bitboard {
        let rank_mask = 0xff << rank * 8;
        Bitboard(rank_mask)
    }

    pub fn get_column_mask(column: i32) -> Bitboard {
        let column_mask = 0x101010101010101 << column;
        Bitboard(column_mask)
    }

    // "/"-direction
    pub fn get_diagonal_mask_asc(column: i32, rank: i32) -> Bitboard {

        //Makes a bitboard from a1 to h8, going through the board diagonally
        //This sould be returned when rank == column
        let mut asc_mask: u64 = 0x8040201008040201;

        if rank > column {
            //Removes the least significant bit and shifts the board to the right (because bb is mirrored)
            for _ in 0..rank - column {
                asc_mask &= asc_mask - 1;
                asc_mask >>= 1;
        }
        } else {
            //Does the same to MSB and the other way around
            for _ in 0..column - rank {
                asc_mask ^= 1 << (63 - asc_mask.leading_zeros());
                asc_mask <<= 1;
            };
        };

        Bitboard(asc_mask)
    }
}
