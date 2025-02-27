fn test_vector() {
    // 三种初始化vector的方法
    let v: Vec<i32> = Vec::new();
    let v1 = Vec::from([1, 2, 3]);
    let v2 = vec![4, 5, 6];

    for i in v {
        println!("{}", i);
    }
    for i in v1 {
        println!("{}", i);
    }
    for i in v2 {
        println!("{}", i);
    }

    // 定义一个immutable的vector
    let mut v = Vec::from([1, 2, 3]);
    // 引用其中的一个元素
    let first_val = &v[0];
    // v.push(4);
    println!("{}", first_val);

    let second_val= &mut v[1];
    // v.remove(2);
    println!("{}", second_val);
}

fn test_string() {
    let mut s = String::from("hello");
    s.push_str(" world");
    s.push('!');

    let s1 = String::from("hello");
    let s2 = String::from(" world!");
    let s = s1 + &s2; // + 展开为 fn add(self, s: &str) -> String {
    // let s = &s1 + &s2; // s1 不能使用 &
    // println!("{}, {}", s1, s2); // add 拿走了 s1 self 的所有权
    println!("{}", s);
}

fn main() {
    test_vector();
    test_string();
}
