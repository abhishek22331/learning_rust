use std::io::{self, Read};
pub fn learnVector() {
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
    for i in &v {
        println!("this is vector value {}", i);
    }
    let v2 = vec![9, 4, 5, 6, 7];
    for j in &v2 {
        println!("this is vector2 {}", j);
    }
    // let mut v3: Vec<i32> = Vec::new(); // here we can add value in new vector
    let mut v3 = vec![9, 4, 5, 6, 7]; //when we add integer in vector

    println!("Please enter an integer for the vector:");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let value: i32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter an integer only");
            return;
        }
    };

    v3.push(value);
    for k in &v3 {
        println!("this is vector333333333333333 {}", k);
    }
}
