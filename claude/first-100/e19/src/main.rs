fn main() {
    let mut sundays = 0;

    // Start from January 1, 1901
    // January 1, 1900 was a Monday, so January 1, 1901 was a Tuesday (day 2)
    let mut day_of_week = 2; // 0=Sunday, 1=Monday, ..., 6=Saturday

    for year in 1901..=2000 {
	for month in 1..=12 {
	    // Check if the 1st of this month is a Sunday
	    if day_of_week == 0 {
		sundays += 1;
	    }

	    // Add the days in this month to move to next month
	    let days_in_month = get_days_in_month(month, year);
	    day_of_week = (day_of_week + days_in_month) % 7;
	}
    }

    println!("Number of Sundays on the 1st of the month (1901-2000): {}", sundays);
}

fn get_days_in_month(month: u32, year: u32) -> u32 {
    match month {
	1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
	4 | 6 | 9 | 11 => 30,
	2 => {
	    if is_leap_year(year) {
		29
	    } else {
		28
	    }
	}
	_ => 0,
    }
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
