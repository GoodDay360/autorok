use sha2::{Sha256, Digest};

const ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn new(input: &str, len: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();

    let mut n = u64::from_be_bytes(hash[0..8].try_into().unwrap());

    let mut out = Vec::with_capacity(len);
    for _ in 0..len {
        out.push(ALPHABET[(n % 62) as usize]);
        n /= 62;
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}