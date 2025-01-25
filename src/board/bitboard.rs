use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, Shr};

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
        let bitmask: u64 = 1 << bit_offset;
        (*self & bitmask) != 0
    }

    pub fn set_bit(&mut self, bit_offset: u32) {
        let mask = 1 << bit_offset;

        self.0 |= mask;
    }

    pub fn unset_bit(&mut self, bit_offset: u32) {
        let mask = !(1 << bit_offset);

        self.0 &= mask;
    }

    pub fn move_bit(&mut self, from_offset: u32, to_offset: u32) {
        //The basis of moving a bit: checks if there is a 1 there, makes it a 0, and makes another field a 1.
        if self.check_bit(from_offset) {
            let rmv_bitmask = 1 << from_offset;
            *self &= !rmv_bitmask;

            //Thinking ahead, there could be some logic here to know if it's stepping on a 1?
            let add_bitmask = 1 << to_offset;
            *self |= add_bitmask;
        }
    }

    pub fn bit_offset_to_coordinates(bit_offset: u32) -> (u32, u32) {
        let column = bit_offset % 8;
        let rank = bit_offset / 8;

        return (column, rank);
    }

    pub fn coordinates_to_bit_offset(column: u32, rank: u32) -> u32 {
        column + rank * 8
    }

    /// Converts a tile &str to coordinates. For example b1 => (1, 0)
    /// Used for debugging purposes only. This function is kinda slow.
    pub fn tile_str_to_coordinates(tile: &str) -> Result<(u32, u32), ()> {
        if tile.len() != 2 {
            return Err(());
        };

        let tile = tile.to_ascii_lowercase();

        let column_char = tile.chars().nth(0).unwrap();
        let rank_char = tile.chars().nth(1).unwrap();

        let column = match column_char {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return Err(())
        };

        let rank = match rank_char {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return Err(())
        };

        Ok((column, rank))
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
        } else {
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
            cover_mask = u64::MAX >> ((7 - sum) * 9);
            shifted_mask = initial_mask >> (7 - sum);
        } else {
            cover_mask = u64::MAX << ((sum - 7) * 9);
            shifted_mask = initial_mask << (sum - 7);
        }

        let des_mask: u64 = shifted_mask & cover_mask;

        Bitboard(des_mask)
    }

    pub fn get_knight_mask(column: i32, rank: i32) -> Bitboard {
        // All possible knight directions from its place
        let moves: [(i32, i32); 8] = [
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
        ];

        let mut knight_mask: u64 = 0;

        // Iterate through directions
        for (x, y) in moves {
            let new_column = column + x;
            let new_rank = rank + y;

            // Check if the move is within the board
            if (0..8).contains(&new_column) && (0..8).contains(&new_rank) {
                // Calculate the bit index for the new position if it is, and add it to the mask
                let offset = new_rank * 8 + new_column;
                knight_mask |= 1 << offset;
            }
        }

        Bitboard(knight_mask)
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Bitboard(value)
    }
}

impl From<Bitboard> for u64 {
    fn from(value: Bitboard) -> Self {
        value.0
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAnd<u64> for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: u64) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0)
    }
}

impl BitAndAssign<u64> for Bitboard {
    fn bitand_assign(&mut self, rhs: u64) {
        *self = Self(self.0 & rhs)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOr<u64> for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: u64) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0)
    }
}

impl BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        *self = Self(self.0 | rhs)
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXor<u64> for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: u64) -> Self::Output {
        Self(self.0 ^ rhs)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign<u64> for Bitboard {
    fn bitxor_assign(&mut self, rhs: u64) {
        *self = Self(self.0 ^ rhs)
    }
}

impl Shl<u64> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u64) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<u64> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u64) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl PartialEq<u64> for Bitboard {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }

    fn ne(&self, other: &u64) -> bool {
        self.0 != *other
    }
}
