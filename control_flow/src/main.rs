fn main() {
    let number = 3;

    if number < 6 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is : {}", number);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        println!("{}", counter);

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let a : [i32; 5] = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is : {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!");

    let celsius = 36;
    let far = celsius_to_fahrenheit(celsius);
    println!("{} degrees Celsius are {} degrees Fahrenheit", celsius, far);

    let result_fibo = fibo(9);

    println!("the fibo value for 3 is : {}", result_fibo);
}
 
fn fibo(nb : i32) -> i32 {
    if nb == 0 || nb == 1 {
        nb
    } else {
        fibo(nb - 1) + fibo(nb - 2)
    }
}

fn celsius_to_fahrenheit(celsius : u32) -> u32 {
    celsius * 9 / 5 + 32
}
