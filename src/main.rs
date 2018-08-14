use std::io;

fn main() {
    // We want to have a prompt now, instead of just falling directly to the test cases.
    println!("Fahrenheit/Celsius converter");
    println!("============================");
    println!("Operations:");
    println!("F - Convert Fahrenheit to Celsius");
    println!("C - Convert Celsius to Fahrenheit");
    println!("T - Run a set of basic self-test computations.");
    println!("X - Exit.");

    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to get command!");
    command = command.trim().to_string(); // trim() returns a `&str` instead of a `String`.
    println!("Received command: {}.", command);

    interactive_from_fahrenheit();
    interactive_from_celsius();
    run_test();
}

fn interactive_from_fahrenheit() {
    println!("Enter a temperature in Fahrenheit:");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to get temperature!");
    let temp : f64 = temp.trim().parse().expect("Enter a number.");
    println!("{}°F is {}°C.", temp, fahrenheit_to_celsius(temp));
}

fn interactive_from_celsius() {
    println!("Enter a temperature in Celsius:");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to get temperature!");
    let temp : f64 = temp.trim().parse().expect("Enter a number.");

    println!("{}°C is {}°F.", temp, celsius_to_fahrenheit(temp));
}

fn run_test() {
    println!("This mode runs a few basic tests of the conversion logic.");
    compare_values(0.0, 32.0);
    compare_values(-40.0, -40.0);
    compare_values(37.0, 98.6);
}

fn compare_values ( celsius_value : f64, fahrenheit_value : f64 ) {
    // Compares a celsius value and fahrenheit value that should match each other and convert into each other.
    println!("Expected converted value for {}°F is {}°C. The actual value is: {}°C.", fahrenheit_value, celsius_value, fahrenheit_to_celsius(fahrenheit_value));
    println!("Expected converted value for {}°C is {}°F. The actual value is: {}°F.", celsius_value, fahrenheit_value, celsius_to_fahrenheit(celsius_value));
}

fn celsius_to_fahrenheit( celsius_temp : f64 ) -> f64 {
    // F = ( 1.8 * C ) + 32
    1.8 * celsius_temp + 32.0
}

fn fahrenheit_to_celsius( fahrenheit_temp : f64 ) -> f64 {
    // C = (5 / 9) * ( F - 32 )
    (5.0 / 9.0) * (fahrenheit_temp - 32.0)
}