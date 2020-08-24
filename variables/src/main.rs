const MAX_POINTS : u32 = 100_000;

fn main(){
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);
    println!("{}", MAX_POINTS);

    let x = x + 1;
    println!("The value of x is : {}", x);

    let x = x * 2;
    println!("The value of x is : {}", x);

    let guess : f32 = "40.2".parse().expect("Not a number");

    println!("The value of guess is {}", guess);

    let t = true;

    println!("the value of t is {}", t);

    let heart_eyed_cat = '😻';

    println!("{}", heart_eyed_cat);

    let tup : (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is : {}", y);
}
