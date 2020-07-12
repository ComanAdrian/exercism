pub fn square_of_sum(n: u32) -> u32 {
    let mut result: u32 = 0;
    for i in 0..n + 1 {
        result += i;
    }
    result.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut result: u32 = 0;
    for i in 0..n + 1 {
        result += i.pow(2);
    }
    result
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
