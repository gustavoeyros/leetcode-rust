fn roman_to_int(s: String) -> i32 {
    let mut result = 0;
    let mut prev_value = 0;

    for c in s.chars() {
        let current_value = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        if current_value > prev_value {
            result += current_value - 2 * prev_value;
        } else {
            result += current_value;
        }

        prev_value = current_value;
    }

    result
}

fn main() {
    let s1 = String::from("IV");
    let s2 = String::from("LVIII");
    let s3 = String::from("MCMXCIV");

    println!("IV = {}", roman_to_int(s1));
    println!("LVIII = {}", roman_to_int(s2));
    println!("MCMXCIV = {}", roman_to_int(s3));
}
