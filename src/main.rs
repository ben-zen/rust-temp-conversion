fn main() {
    run_test();
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