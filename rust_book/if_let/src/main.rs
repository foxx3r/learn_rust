#[derive(Debug)]

#[allow(dead_code)]
enum State {
    Alabama,
    Alaska,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dimme,
    Quarter(State)
}

fn main() {
    let mut count = 0;
    let coin = Coin::Quarter(State::Alabama);

    if let Coin::Quarter(state) = coin {
        println!("{:?} state quarter!", state);
    } else {
        count += 3;
    }

    println!("{}", count);
}
