use std::io;

fn main() {
    println!("Input temperature: ");
    let mut input = String::new();
    let number: f64;
    let mut sign: String;
    loop {
        io::stdin().read_line(&mut input).expect(
            "Failed to read line",
        );
        let tup = input.split_at(input.len() - 2);
        sign = tup.1.trim().to_uppercase();
        let potential_number = tup.0.parse();
        if potential_number.is_ok() && (sign == "F" || sign == "C") {
            number = potential_number.unwrap();
            break;
        }
        println!("Invalid input. Give input in the form of: 36C or 72F");
    }

    let result: f64 = if sign == "F" {
        from_farenheit(number)
    } else {
        from_celcius(number)
    };

    let new_sign: char = if sign == "F" { 'C' } else { 'F' };

    println!("Temp: {}{}", result, new_sign);
}

fn from_farenheit(input: f64) -> f64 {
    input - 32.0 * 5.0 / 9.0
}

fn from_celcius(input: f64) -> f64 {
    input * 9.0 / 5.0 + 32.0
}
