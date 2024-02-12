use std::collections::HashMap;
pub fn learnString() {
    println!("Learn string");
    let mut first_string = String::new();
    first_string = String::from("i don't know");
    println!("{}", first_string);
    let mut mutalString = String::from("value1");
    mutalString.push_str("value2"); //push will only push char for pushing string use push_str
    println!("{}", mutalString);
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", &s3[0..2]);
    println!("{s3}");
    for c in s3.chars() {
        println!("cccccccccccccc {c}");
    }
    ////HashMap
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let field_name1 = String::from("Favorite color2");
    let field_value1 = String::from("Blue2");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    map.insert(field_name1, field_value1);
    for (key, value) in map {
        println!("{} is {}", key, value);
    }
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue4")).or_insert(50);

    println!("{:?}", scores);
    let text = "hello world hello";

    let mut map3 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map3.entry(word).or_insert(0);
        *count += 1; //count will store in number of occurance for a word
    }

    println!("{:?}", map3);
}
