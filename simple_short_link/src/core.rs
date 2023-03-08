use std::io::Cursor;

use anyhow::{Ok, Result};

fn hash(s: &str) -> Result<u32> {
    let hash = murmur3::murmur3_32(&mut Cursor::new(s), 0)?;
    Ok(hash)
}

fn hash_with_seed(s: &str, seed: u32) -> Result<u32> {
    let hash = murmur3::murmur3_32(&mut Cursor::new(s), seed)?;
    Ok(hash)
}

fn u32_to_base62(hash: u32) -> Result<String> {
    let dict = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut n = hash;
    let mut chars: Vec<char> = vec![];
    while n > 0 {
        let i = (n % 62) as usize;
        let c = dict.chars().nth(i).unwrap();
        chars.push(c);
        n /= 62;
    }
    chars.reverse();
    let s = chars.into_iter().collect::<String>();
    Ok(s)
}

pub fn short_link(url: &str) -> Result<String> {
    let hash = hash(url)?;
    Ok(u32_to_base62(hash)?)
}

pub fn short_link_with_seed(url: &str, seed: u32) -> Result<String> {
    let hash = hash_with_seed(url, seed)?;
    Ok(u32_to_base62(hash)?)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_short_link() {
        assert_eq!(short_link("https://axum.rs").unwrap(), "3PjdTF".to_string());
    }
}
