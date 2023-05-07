use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde::{Deserialize, Serialize};
use std::ptr::hash;

pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    bincode::serialize(value).unwrap()
}

pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    bincode::deserialize(bytes).unwrap()
}

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn coder_work() {
        let p = Point { x: 1, y: 2 };
        let se = my_serialize(&p);
        let de: Point = my_deserialize(&se);
        assert_eq!(p, de);
    }
}
