fn main() {
    let mut total_letters = 0;

    for n in 1..=1000 {
	total_letters += count_letters(&number_to_words(n));
    }

    println!("Total letters used: {}", total_letters);
}

fn number_to_words(n: u32) -> String {
    if n == 1000 {
	return "onethousand".to_string();
    }

    let mut result = String::new();

    let hundreds = n / 100;
    let remainder = n % 100;

    if hundreds > 0 {
	result.push_str(digit_to_word(hundreds));
	result.push_str("hundred");
	if remainder > 0 {
	    result.push_str("and");
	}
    }

    if remainder >= 20 {
	let tens = remainder / 10;
	let ones = remainder % 10;
	result.push_str(tens_to_word(tens));
	if ones > 0 {
	    result.push_str(digit_to_word(ones));
	}
    } else if remainder >= 10 {
	result.push_str(teen_to_word(remainder));
    } else if remainder > 0 {
	result.push_str(digit_to_word(remainder));
    }

    result
}

fn digit_to_word(n: u32) -> &'static str {
    match n {
	1 => "one",
	2 => "two",
	3 => "three",
	4 => "four",
	5 => "five",
	6 => "six",
	7 => "seven",
	8 => "eight",
	9 => "nine",
	_ => "",
    }
}

fn teen_to_word(n: u32) -> &'static str {
    match n {
	10 => "ten",
	11 => "eleven",
	12 => "twelve",
	13 => "thirteen",
	14 => "fourteen",
	15 => "fifteen",
	16 => "sixteen",
	17 => "seventeen",
	18 => "eighteen",
	19 => "nineteen",
	_ => "",
    }
}

fn tens_to_word(n: u32) -> &'static str {
    match n {
	2 => "twenty",
	3 => "thirty",
	4 => "forty",
	5 => "fifty",
	6 => "sixty",
	7 => "seventy",
	8 => "eighty",
	9 => "ninety",
	_ => "",
    }
}

fn count_letters(s: &str) -> usize {
    s.len()
}
