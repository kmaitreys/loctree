use anyhow::Result;
use loctree::morton;
use loctree::utils;

fn main() -> Result<()> {
    let array = [1, 2, 3, 4];
    let hash = utils::deterministic_hash(&array, 5)?;

    println!("{}", hash);

    let level = morton::find_level(345243);
    println!("{}", level);
    Ok(())
}
