// fn main() {
//     println!("Hello nainasweetheart");
//     let ans: u32 = sum(4, 5);
//     println!("{}", ans);
//     println!("{}", is_even(33));
//     let name = "nainasweetheart"; // when you know name is immutable
//     println!("{}", name);

//     //second way

//     let gf: String = String::from("Hazel");
//     println!("my Ex girlfriend was : {}", gf);

//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     println!("{:?}", arr.len());

//     let vector = vec![1, 2, 3];
//     println!("{:?}", vector);
// }

// fn sum(a: u32, b: u32) -> u32 {
//     return a + b;
// }

// // u32 , u64 , usize , u128  , i32 , i64 , i128

// fn is_even(a: u32) -> bool {
//     return a % 2 == 0;
// }

// // let name = naina --> in case of js  in case of string there are two ways

// loops in rust

// fn main() {
//     for i in 0..100 {
//         println!("{}", i);
//     }
// }

// fn main() {
//     let mut x = 25;
//     x = 23;
//     println!("{}", x)
// }

fn main() {
    let mut name: String = String::from("subh");
    name.push_str(" naina");
    println!("{}", name)
}
