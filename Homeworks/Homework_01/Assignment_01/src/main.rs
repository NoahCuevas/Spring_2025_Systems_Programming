const FREEZE_POINT_F:f64 = 32.0;

fn fahrenheit_to_celcius(f:f64) -> f64 {
    return (f - FREEZE_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c:f64) -> f64 {
    return (c * 9.0 / 5.0) + FREEZE_POINT_F
}

fn main() {
    let temp_f = 95.0;
    let temp_c = 0.0;

    println!("Fahrenheit: {} -> Celcius: {}", temp_f, fahrenheit_to_celcius(temp_f));
    
    for i in 1..=5 {
        let new_temp_f = temp_f + i as f64;
        println!("Fahrenheit: {} -> Celcius: {}", new_temp_f, fahrenheit_to_celcius(new_temp_f));
    }


    println!("Celsius: {} -> Fahrenheit: {}", temp_c, celsius_to_fahrenheit(temp_c));

    for i in 1..=5 {
        let new_temp_c = temp_c + i as f64;
        println!("Celsius: {} -> Fahrenheit: {}", new_temp_c, celsius_to_fahrenheit(new_temp_c));
    }
}