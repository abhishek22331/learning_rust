use std::fmt; //std::fmt is a module that provides facilities for formatting and printing text

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

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User(name: {}, roll_no: {}, email: {}, DOB: {}, fatherName: {})",
            self.name, self.roll_no, self.email, self.DOB, self.fatherName
        )
    }
}

fn checking(email: String, name: String) -> User {
    User {
        name, //: String::from(name) //we can use this also 
        roll_no: 1,
        email: String::from(email),
        DOB: String::from("2000"),
        fatherName: String::from("oooooooooooooo"),
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

    let ad = checking("emam".to_owned(), "ppppppppp".to_owned());
    println!("le bhai bhai {}", ad.name);
}
