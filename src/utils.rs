// Some utility functions
use anyhow::Result;
use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;
use hex;

pub fn deterministic_hash(array: &[u8], digest_size: usize) -> Result<u128> {
    let mut hasher = Blake2bVar::new(digest_size)?;
    hasher.update(array);

    let mut output = vec![0u8; digest_size];
    hasher.finalize_variable(&mut output)?;
    let output = u128::from_str_radix(&hex::encode(output), 16)?;
    Ok(output)
}

pub fn deterministic_checksum(coord: &[i32]) -> u128 {
    let mut hash = 0u128;
    for i in coord {
        if *i < 0 {
            hash |= (2i32 * (-*i) + 1) as u128;
        } else {
            hash |= (2i32 * *i) as u128;
        }
        hash <<= 16;
    }
    hash
}
