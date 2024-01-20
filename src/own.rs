pub fn ownership(){
    let str = String::from("abhishek");
    // let str2= str; // when it will run it give str value to str2 and then str will not own any value
    // pritnln!("{}",str) //so it will give error
    
    // do this
    let str2=str.clone(); //it will clone str property to str2
    println!("{}",str2);
    println!("{}",owner(str2));
}
fn owner(str:String)->String{
    let mut ans=String::from ("something ");
    ans=ans+&str;
    ans
}