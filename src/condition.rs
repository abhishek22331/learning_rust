pub fn con()->bool{
    let a=8;
    if 1<a{
      true
    }else{
        false
    }
}

pub fn con2()->u32{
    let a:u32=9;
    // if a{ //we can only check from boolean  
        println!("{a} present");
        a
    // }
}

pub fn looping(){
    // let mut z:u32=0;
    // let mut temp:bool=false;
    // loop {
    //     println!("runing...");
    //     z+=1;
    //     temp=true;
    //     if temp{
    //         break;
    //     }
    //     // if z==5{
    //     //     break;
    //     // }
    // }
    // let mut name:bool=false;
    // 'loopname:loop{
    //     name=true;
    //     if name{
    //         println!("nameloop222222");
    //         break;
    //     }
    // }
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         println!("------------{count}");
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");

    let arr: [i32; 6]=[2,3,4,5,6,7];
    let mut len: usize=0;
    loop{
        println!("using loop loop 😂{}",arr[len]);
        len+=1;
        if len==arr.len(){
            break;
        }
    }
    for ele in arr{
        println!("looping for{}",ele);
    }

    //another great loop here below
 
        for number in (1..7) { // it will iterates till 6 
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    
}