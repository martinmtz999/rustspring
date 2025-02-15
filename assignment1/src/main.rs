fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0/9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

fn main() {
    const FREEZE: u32 = 32;

    let mut celci = 36.0;
    let mut fari  = 98.0;

    println!("The Fahrenheit is: {}", celsius_to_fahrenheit(celci));
    println!("The Celsius is: {}", fahrenheit_to_celsius(fari));

    let mut counter = 0;
    loop{
        println!("For the celcius temp {},The Fahrenheit is: {}", celci,celsius_to_fahrenheit(celci));
        celci += 5.0;

        counter += 1;

        if counter == 5 {
            break;
        }
    }
}