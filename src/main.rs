mod rsa;
mod math;

use std::env;
use std::error::Error;
use crate::rsa::KeySet;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    dbg!(KeySet::new());
    Ok(())
}
