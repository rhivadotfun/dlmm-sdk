use anyhow::{ensure, Result};
use bytemuck::AnyBitPattern;

/// Decode an anchor account from raw account data bytes.
/// Strips the 8-byte discriminator and reads exactly `size_of::<T>()` bytes.
pub fn pod_read_unaligned_skip_disc<T: AnyBitPattern>(account_data: &[u8]) -> Result<T> {
    let size = std::mem::size_of::<T>();
    ensure!(
        account_data.len() >= 8 + size,
        "account data too short: expected at least {} bytes, got {}",
        8 + size,
        account_data.len()
    );
    Ok(bytemuck::pod_read_unaligned(&account_data[8..8 + size]))
}

pub mod quote;
pub use quote::*;

pub mod rpc_client_extension;

pub mod account_filters;
pub use account_filters::*;

pub mod token_2022;
pub use token_2022::*;

pub mod cluster;
pub use cluster::*;

pub use dlmm;
pub use dlmm::pda::*;
pub use dlmm::{
    BinArrayBitmapExtExtension, BinArrayExtension, BinExtension, LbPairExtension,
    LimitOrderExtension, PositionExtension,
};

