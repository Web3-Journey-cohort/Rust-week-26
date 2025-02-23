fn main() {
    println!("Hello nainasweetheart");
    let ans: u32 = sum(4, 5);
    println!("{}", ans);
    println!("{}", is_even(33))
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

// u32 , u64 , usize , u128  , i32 , i64 , i128

fn is_even(a: u32) -> bool {
    return a % 2 == 0;
}
