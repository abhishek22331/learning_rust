mod fun;
mod condition;
mod own;
mod learnStruct;
mod learnEnum;
fn main() {
    let mut a: i32=5;
    println!("{a}");
    a=9;
    println!("{a}");
    {
        let a: i32=18;
        println!("aaaaaaaaaaaaa:{a}");
    }
    println!("{a}");
    let str: &str="a ";
    let l: usize=str.len();
    println!("{}",str.len());
    let guess: u32 = "8".parse().expect("Not a number!");
    println!("{}",guess);
    let z:char='d';
    println!("{z}");
    let tup:(i32,u8)=(302,6);
    println!("{},{}",tup.0,tup.1);
    let (x, _y) = tup;
    println!("{}", x);
    let b: [i32; 5] = [1, 2, 3, 4,9];
    println!("{}",b[4]);
    let mut firstStr=String::new();
    firstStr.push_str("testing");
    println!("eeeeeeee {}",firstStr);
    let mut my_string = String::from("Hello, World!");
    my_string.push_str(" amit");
    println!("{}",my_string);
    let hello: &str = "Hello ,";
    let world: &str = "World!";
    let hw=hello.to_owned()+world;
    println!("{},{}",hw,hello);
   
    let original_string = String::from("Hello, World!");
    let owned_string = original_string.clone();

    println!("Original String: {}", original_string);
    println!("Owned String: {}", owned_string);
    fun::sum();
    condition::looping();
   let ans: bool= condition::con();
   let ans2: u32= condition::con2();
   println!("{} {}",ans,ans2);
   own::ownership();
   learnStruct::learnStruct();
   learnEnum::learnEnum()
}
