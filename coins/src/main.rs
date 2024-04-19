#[derive(Debug)]
enum UsState {
    Alabama, 
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

fn value_in_coin2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("penny");
            1
        }
        other => {
            println!("{:?}", other);
            2

        }
    }
}


fn value(x: Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => i,
    }
}


fn main() {
   // let coin = Coin::Quarter(UsState::Alabama); 
   // value_in_coin2(coin);
   
   //println!("value is {}", value(Some(5)));

   let config_max: Option<i32> = None;
   if let Some(max) = config_max {
        println!("The maximum is {}", max);
   }


}

