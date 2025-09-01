//! Definitions for the native ATLAS token and its fractional lamports.

#![allow(clippy::arithmetic_side_effects)]

/// There are 10^9 lamports in one ATLAS
pub const LAMPORTS_PER_ATLAS: u64 = 1_000_000_000;
const ATLAS_DECIMALS: usize = 9;

/// Convert native tokens (ATLAS) into fractional native tokens (lamports)
pub fn atlas_str_to_lamports(atlas_str: &str) -> Option<u64> {
    if atlas_str == "." {
        None
    } else {
        let (atlas, lamports) = atlas_str.split_once('.').unwrap_or((atlas_str, ""));
        let atlas = if atlas.is_empty() {
            0
        } else {
            atlas.parse::<u64>().ok()?
        };
        let lamports = if lamports.is_empty() {
            0
        } else {
            format!("{lamports:0<9}")[..ATLAS_DECIMALS].parse().ok()?
        };
        LAMPORTS_PER_ATLAS
            .checked_mul(atlas)
            .and_then(|x| x.checked_add(lamports))
    }
}

use std::fmt::{Debug, Display, Formatter, Result};
pub struct Atlas(pub u64);

impl Atlas {
    fn write_in_atlas(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "◎{}.{:09}",
            self.0 / LAMPORTS_PER_ATLAS,
            self.0 % LAMPORTS_PER_ATLAS
        )
    }
}

impl Display for Atlas {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_atlas(f)
    }
}

impl Debug for Atlas {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_atlas(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atlas_str_to_lamports() {
        assert_eq!(0, atlas_str_to_lamports("0.0").unwrap());
        assert_eq!(1, atlas_str_to_lamports("0.000000001").unwrap());
        assert_eq!(10, atlas_str_to_lamports("0.00000001").unwrap());
        assert_eq!(100, atlas_str_to_lamports("0.0000001").unwrap());
        assert_eq!(1000, atlas_str_to_lamports("0.000001").unwrap());
        assert_eq!(10000, atlas_str_to_lamports("0.00001").unwrap());
        assert_eq!(100000, atlas_str_to_lamports("0.0001").unwrap());
        assert_eq!(1000000, atlas_str_to_lamports("0.001").unwrap());
        assert_eq!(10000000, atlas_str_to_lamports("0.01").unwrap());
        assert_eq!(100000000, atlas_str_to_lamports("0.1").unwrap());
        assert_eq!(1000000000, atlas_str_to_lamports("1").unwrap());
        assert_eq!(4_100_000_000, atlas_str_to_lamports("4.1").unwrap());
        assert_eq!(8_200_000_000, atlas_str_to_lamports("8.2").unwrap());
        assert_eq!(8_502_282_880, atlas_str_to_lamports("8.50228288").unwrap());

        assert_eq!(
            u64::MAX,
            atlas_str_to_lamports("18446744073.709551615").unwrap()
        );
        // bigger than u64::MAX, error
        assert_eq!(None, atlas_str_to_lamports("18446744073.709551616"));
        // Negative, error
        assert_eq!(None, atlas_str_to_lamports("-0.000000001"));
        // i64::MIN as string, error
        assert_eq!(None, atlas_str_to_lamports("-9223372036.854775808"));
    }
}
