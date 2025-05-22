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
    let base58 = bs58::encode(result).into_string();

    // add c4 to beginning
    format!("c4{}", base58)
}

#[test]
fn c4_file_test() {
    let real = "c45ofc7UjV8ZVECmYD1tZbHXWafRaX7hg56bV9LqEpe73g4HedomAU1niYBcmcTA1sqxV8FLxhbUHFtutvfszL6JWM";
    let file = "tests/sample.mhl";
    let result = c4_file(file);
    assert_eq!(result, real);
}
