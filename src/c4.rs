use std::{fs::File, io::Read};

use sha2::{Digest, Sha512};

pub fn c4_file(path: &str) -> String {
    // open file
    let mut file = File::open(path).unwrap();

    // read data
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer);

    // compute SHA-512
    let mut hasher = Sha512::new();
    hasher.update(buffer);
    let result = hasher.finalize();

    // compute base58
    let mut base58 = bs58::encode(result).into_string();
    base58 = format!("{:1>88}", base58);

    // add c4 to beginning
    format!("c4{}", base58)
}

