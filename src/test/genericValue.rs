use dotenv::dotenv;
use std::env;
fn largest<T: PartialOrd + std::fmt::Display>(list: &[T]) -> &T {
    // Display is printing items if List
    let mut larg = &list[0];

    for item in list {
        println!("item {}", item);
        if item > larg {
            larg = item;
        }
    }

    larg
}

pub fn calling() {
    dotenv().ok();
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    
    match env::var("ENV_VARIABLE_NAME"){
        Ok(val) => println!("The value of ENV_VARIABLE_NAME is: {}", val),
        Err(e) => println!("Error reading the environment variable: {}", e),
    }
    /*  
    below code will read 2 env var at a same time
    // match (env::var("ENV_VARIABLE_NAME"), env::var("ENV_VARIABLE_NAME1")) {
    //     (Ok(val1), Ok(val2)) => {
    //         println!("The value of ENV_VARIABLE_NAME is: {}", val1);
    //         println!("The value of ENV_VARIABLE_NAME1 is: {}", val2);
    //     }
    //     (Err(e), _) => println!("Error reading the environment variable ENV_VARIABLE_NAME: {}", e),
    //     (_, Err(e)) => println!("Error reading the environment variable ENV_VARIABLE_NAME1: {}", e),
    // }
    */
}
