/**
 * Module for handling a 64-bit chess bitboard.
 *
 * Includes various bit manipulation and mask generation functions,
 * forming the basis of chessboard representation in bitboards.
 */

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

use const_for::const_for;

use crate::player::Player;

use super::{moove::CastleSide, tile_position::TilePosition};

/**
 * Represents a 64-bit bitboard used in the chess engine.
 */
#[derive(Clone, Copy, Hash, PartialEq, Eq)]

pub struct Bitboard(pub u64);

impl Bitboard {
    ///@return the 64-bit binary number representing the Bitboard state
    pub const fn value(&self) -> u64 {
        self.0
    }

    /**
     * Prints the Bitboard as an 8x8 grid.
     * @note This is purely for debugging purposes.
     */
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

    /**
     * Checks if a specific bit is set.
     * @param bit_offset The position of the bit to check.
     * @return True if the bit is set, otherwise false.
     */
    pub const fn check_bit(&self, bit_offset: u32) -> bool {
        let bitmask: u64 = 1 << bit_offset;
        (self.0 & bitmask) != 0
    }

    /**
     * Sets a specific bit.
     * @param bit_offset The position of the bit to set.
     */
    pub fn set_bit(&mut self, bit_offset: u32) {
        let mask = 1 << bit_offset;

        self.0 |= mask;
    }

    /**
     * Unsets a specific bit.
     * @param bit_offset The position of the bit to unset.
     */
    pub fn unset_bit(&mut self, bit_offset: u32) {
        let mask = !(1 << bit_offset);

        self.0 &= mask;
    }

    /**
     * Pops the least significant bit and returns its offset.
     * @return The offset of the least significant bit.
     */
    pub fn pop_lsb(&mut self) -> u32 {
        let lsb_offset = self.0.trailing_zeros();
        self.0 &= self.0 - 1;
        lsb_offset
    }

    /**
     * Checks if the Bitboard is empty.
     * @return True if the Bitboard is empty, otherwise false.
     */
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /**
     * Moves a bit from one offset to another.
     * @param from_offset The source bit position.
     * @param to_offset The target bit position.
     */
    pub fn move_bit(&mut self, from_offset: u32, to_offset: u32) {
        if self.check_bit(from_offset) {
            let rmv_bitmask = 1 << from_offset;
            *self &= !rmv_bitmask;

            let add_bitmask = 1 << to_offset;
            *self |= add_bitmask;
        }
    }

    /**
     * The following functions generate movement masks for different chess pieces on a bitboard.
     * @param rank The rank (0-7).
     * @param column The column (0-7).
     * @return The mask as Bitboard
     */
    pub const fn generate_rank_mask(rank: u32) -> Bitboard {
        debug_assert!(rank <= 7);
        let rank_mask = 0xff << rank * 8;
        Bitboard(rank_mask)
    }

    pub const fn generate_column_mask(column: u32) -> Bitboard {
        debug_assert!(column <= 7);
        let column_mask = 0x101010101010101 << column;
        Bitboard(column_mask)
    }

    // "/"-direction
    pub const fn get_diagonal_mask_asc(column: u32, rank: u32) -> Bitboard {
        let diff = column as i32 - rank as i32;

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

    pub const fn get_diagonal_mask_des(column: u32, rank: u32) -> Bitboard {
        let sum = column as i32 + rank as i32;

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

    pub const fn generate_knight_mask(tile_position: TilePosition) -> Bitboard {
        let column = tile_position.column();
        let rank = tile_position.rank();

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

        const_for!(i in 0..moves.len() => {
            let (x, y) = moves[i];
            let new_column = column as i32 + x;
            let new_rank = rank as i32 + y;

            if new_column < 0 || new_column > 7 {
                continue;
            }

            if new_rank < 0 || new_rank > 7 {
                continue;
            }

            let offset = new_rank * 8 + new_column;
            knight_mask |= 1 << offset;
        });

        Bitboard(knight_mask)
    }

    pub const fn get_king_mask(column: u32, rank: u32) -> Bitboard {
        let mut king_mask: u64 = 0x70507;
        let offset_diff = 9 - (rank as i32 * 8 + column as i32);

        if column == 0 || column == 7 {
            let cover_mask = 0x10101 << column / 3;
            king_mask &= !cover_mask;
        }
        if rank == 0 || rank == 7 {
            let cover_mask = 0b111 << (rank << 2);
            king_mask &= !cover_mask;
        }

        if offset_diff < 0 {
            king_mask <<= -offset_diff;
        } else {
            king_mask >>= offset_diff;
        }

        Bitboard(king_mask)
    }

    pub const fn get_white_pawn_mask(column: u32, rank: u32) -> Bitboard {
        if rank == 7 {
            return Bitboard(0);
        }

        let mut pawn_mask: u64 = 1 << ((rank + 1) * 8 + column);

        if rank == 1 {
            pawn_mask = pawn_mask << 8 | pawn_mask;
        }

        Bitboard(pawn_mask)
    }

    pub const fn get_black_pawn_mask(column: u32, rank: u32) -> Bitboard {
        if rank == 0 {
            return Bitboard(0);
        }

        let mut pawn_mask: u64 = 1 << ((rank - 1) * 8 + column);

        if rank == 6 {
            pawn_mask = pawn_mask >> 8 | pawn_mask;
        }

        Bitboard(pawn_mask)
    }

    /**
     * Generates a mask used in castling checks.
     * @param rank The player color.
     * @param side The side of castling.
     * @return The mask as Bitboard
     */
    pub fn generate_castling_block_mask(player: Player, side: CastleSide) -> Bitboard {
        match (player, side) {
            (Player::White, CastleSide::KingSide) => Bitboard::from(0b01100000),
            (Player::White, CastleSide::QueenSide) => Bitboard::from(0b00001110),
            (Player::Black, CastleSide::KingSide) => Bitboard::from(0b01100000 << (8 * 7)),
            (Player::Black, CastleSide::QueenSide) => Bitboard::from(0b00001110 << (8 * 7))
        }
    }

    pub fn generate_castling_threat_mask(player: Player, side: CastleSide) -> Bitboard {
        match (player, side) {
            (Player::White, CastleSide::KingSide) => Bitboard::from(0b01100000),
            (Player::White, CastleSide::QueenSide) => Bitboard::from(0b00001100),
            (Player::Black, CastleSide::KingSide) => Bitboard::from(0b01100000 << (8 * 7)),
            (Player::Black, CastleSide::QueenSide) => Bitboard::from(0b00001100 << (8 * 7))
        }
    }

    /**
     * Generates a bitboard with a horizontal line of 1's in it.
     * @param length Length of the line.
     * @return A Bitboard with the line in its least significant bits.
     */
    pub const fn generate_horizontal_line(length: u32) -> Bitboard {
        Bitboard((1u64 << length) - 1)
    }
}

/**
 * The following functions implement various bitwise and conversion traits,
 * allowing common bit manipulation operations and seamless conversion between
 * `Bitboard` and `u64`.
 */
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

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)    
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
