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
        //A bitboard that goes through the first rank, then moved by column
        let rank_mask = 0xff << rank * 8;
        Bitboard(rank_mask)
    }

    pub fn get_column_mask(column: i32) -> Bitboard {
        //^The other way around: a bitboard going through the first column, then nudged left
        let column_mask = 0x101010101010101 << column;
        Bitboard(column_mask)
    }

    // "/"-direction
    pub fn get_diagonal_mask_asc(column: i32, rank: i32) -> Bitboard {
        let diff = column - rank;

        let initial_mask: u64 = 0x8040201008040201;

        let shifted_mask: u64;
        let cover_mask: u64;

        if diff < 0 {
            cover_mask = u64::MAX << (-diff * 8);
            shifted_mask = initial_mask >> -diff;
        }
        else {
            cover_mask = u64::MAX >> (diff * 8);
            shifted_mask = initial_mask << diff;
        }

        let asc_mask: u64 = shifted_mask & cover_mask;

        Bitboard(asc_mask)
    }

    pub fn get_diagonal_mask_des(column: i32, rank: i32) -> Bitboard {
        let sum = column + rank;

        let initial_mask: u64 = 0x102040810204080;

        let shifted_mask: u64;
        let cover_mask: u64;

        if sum < 7 {
            cover_mask = u64::MAX >> ((7-sum) * 9);
            shifted_mask = initial_mask >> (7-sum);
        }
        else {
            cover_mask = u64::MAX << ((sum - 7) * 9);
            shifted_mask = initial_mask << (sum - 7);
        }

        let des_mask: u64 = shifted_mask & cover_mask;

        Bitboard(des_mask)
    }

    // "\" -direction
    // pub fn get_diagonal_mask_des (column: i32, rank: i32) -> Bitboard {
    //     //Starts with a bitboard going from a7 to h1 diagonally
    //     //This should be returned when col + rank == 7
    //     let mut des_mask: u64 = 0x102040810204080;

    //     if rank + column > 7 {
    //         for _ in 0..(rank + column - 7) {
    //             des_mask &= des_mask - 1;
    //             des_mask <<= 1;
    //         }
    //     } else if rank + column < 7 {
    //         for _ in 0..(7 - rank + column) {
    //             des_mask ^= 1 << (63 - des_mask.leading_zeros());
    //             des_mask >>= 1;
    //         }
    //     }

        // Bitboard(des_mask)
    // }
}
