// Define a simple TrafficLight enum
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(u32), 
}

pub fn learnEnum() {
    // let red_light: TrafficLight = TrafficLight::Red;
    let red_light: TrafficLight = TrafficLight::Yellow;
    // let red_light: TrafficLight = TrafficLight::Green;
    let coin: Coin = Coin::Quarter(25);
    // let coin: Coin = Coin::Penny;
    // let coin: Coin = Coin::Nickel;
    // let coin: Coin = Coin::Dime;


    match red_light { //it match the value
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Slow down."),
        TrafficLight::Green => println!("Go!"),
    }

    match coin {
        Coin::Penny => println!("It's a penny."),
        Coin::Nickel => println!("It's a nickel."),
        Coin::Dime => println!("It's a dime."),
        Coin::Quarter(cents) => println!("It's a quarter with {} cents.", cents),
    }
}
