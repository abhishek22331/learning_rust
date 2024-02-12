pub fn learnString() {
    println!("Learn string");
    let mut first_string = String::new();
    first_string = String::from("i don't know");
    println!("{}", first_string);
    let mut mutalString = String::from("value1");
    mutalString.push_str("value2"); //push will only push char for pushing string use push_str
    println!("{}",mutalString);
}
