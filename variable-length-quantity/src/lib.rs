#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let raw = format!("{:x}", values.get(0).unwrap());
    println!("{:b}", values.get(0).unwrap());

    let dec = u32::from_str_radix(&raw, 16);
    println!("{:?}", &dec);

    // let x = format!("{:b}", dec.unwrap());
    // println!("{:?}", x);
    unimplemented!("Convert the values {:?} to a list of bytes", values)
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    unimplemented!("Convert the list of bytes {:?} to a list of numbers", bytes)
}

fn to_hex(val: &str, len: usize) -> String {
    let n: u32 = u32::from_str_radix(val, 2).unwrap();
    format!("{:01$x}", n, len * 2)
}



https://exercism.io/tracks/rust/exercises/variable-length-quantity/solutions/49c76befb33e43949ec9e90a97b7bd2a