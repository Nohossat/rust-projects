fn main() {
    println!("Hello, world!");
    another_function(6, 10);

    let y = {
        let x = 3;
        x + 1
    };

    println!("{}", y);

    let x = five();

    println!("the value of x is : {}", x);

    let z = plus_one(7);

    println!("the value of z is : {}", z);

}

fn another_function(x : i32, y : i32){
    println!("the value of x is : {}", x);
    println!("thevqlue of y is : {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x : u8) -> u8 {
    x + 1
}