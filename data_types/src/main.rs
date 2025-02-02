fn main() {
    let data : u32 = "42".parse().unwrap();
    println!("the value of data is {data}.");

    let tup : (i32, f64, u8) = (500, 6.4, 1);
    println!("the 1/2/3 th member of variable tup is {0}, {1}, {2}", tup.0, tup.1, tup.2);
    let (a, b, c) : (i32, f64, u8) = tup;
    println!("the values of a, b, c are {a}, {b}, {c}.");
}
