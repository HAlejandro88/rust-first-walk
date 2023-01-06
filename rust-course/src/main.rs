fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
    let y = plus_one(x);

    println!("The value of x is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}



