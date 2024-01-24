struct User {
    name: String,
    roll_no: u32,
    email: String,
    DOB: String,
    fatherName: String,
}

impl User {
  
    fn update_email(&mut self, new_email: String) {
        self.email = new_email;
    }
}

pub fn learnStruct() {
    let mut person: User = User {
        name: String::from("amit"),
        roll_no: 1,
        email: String::from("amit@gmail.com"),
        DOB: String::from("2000"),
        fatherName: String::from("amit"),
    };

    println!("Initial Email: {}", person.email);

    person.update_email(String::from("amit_updated@gmail.com"));

    println!("Updated Email: {}", person.email);
}


