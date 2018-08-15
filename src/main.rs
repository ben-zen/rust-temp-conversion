// Implementation of a Fahrenheit-Celsius converter in Rust, at the behest
// of the No Starch Press Rust book.
// Copyright Ben Lewis, 2018.
// Licensed under the MIT license.

use std::io;

// Move the command handling into these tuples, and make the match below into a
// search and call operation.
const OPERATIONS : [(&str, &str); 5] = [ ( "F" , "Convert Fahrenheit to Celsius."),
      		     	   	                 ( "C" , "Convert Celsius to Fahrenheit."),
		     		                     ( "T" , "Run a set of basic self-test computations."),
                                         ( "?" , "Print this message."),
		     		                     ( "X" , "Exit." ) ];

fn print_operations() {
    println!("Operations:");
    for operation in OPERATIONS.iter() {
        println!("{} - {}", operation.0, operation.1);
    }
}

fn main() {
    // We want to have a prompt now, instead of just falling directly to the test cases.
    println!("Fahrenheit/Celsius converter");
    println!("============================");
    print_operations();

    loop {
        println!("");
        println!("Enter an operation.");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to get command!");
        let command = command.trim(); // trim() returns a `&str` instead of a `String`.
        println!("Received command: {}.", command);
        match command {
            "C" => interactive_from_celsius(),
            "F" => interactive_from_fahrenheit(),
            "T" => run_test(),
            "H" | "?" => print_operations(),
            "X" => {
                println!("Goodbye.");
                break
            },
            _   => println!("Please enter a recognized command.")
        }
    }
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