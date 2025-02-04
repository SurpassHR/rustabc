fn test_shadowing()
{
    let var = 5;
    let var = var + 1;
    {
        let var = var * 2;
        println!("the value of var is {var}.");
    }
    println!("the value of var is {var}.");

    let spaces = "    ";
    println!("the variable spaces is {spaces}");
    let spaces = spaces.len();
    println!("the variable spaces is {spaces}");

    // let mut mut_spaces = "    ";
    // mut_spaces = mut_spaces.len(); mismatched types expected `&str`, found `usize`
}

fn test_copy()
{
    let tup1 = (1, 2, (), "Hello World");
    let tup2 = tup1;

    println!("tup1: {:?}, tup2: {:?}", tup1, tup2);
}

fn test_close()
{
    let tup1 = (1, 2, (), "Hello World".to_string());
    let tup2 = tup1.clone();

    println!("tup1: {:?}, tup2: {:?}", tup1, tup2);
}

fn main() {
    let var = 5;
    println!("the value of var is {var}.");
    let mut mut_var = 6;
    mut_var += 1;
    println!("the value of mut_var is {mut_var}.");

    test_shadowing();

    test_copy();
    test_close();
}
