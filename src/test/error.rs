use std::fs::File;
use std::io::ErrorKind;

pub fn all_error(){
   //  let greeting_file_result = File::open("hello.txt").expect("hello.txt should be included in this project"); //insted of using panic we can use expect which will give error message "hello.txt should be included in this project" when hello.txt not present
    let greeting_file_result: Result<File, std::io::Error> = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}