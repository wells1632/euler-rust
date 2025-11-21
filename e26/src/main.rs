/*
A unit fraction contains 1 in the numerator. The decimal representation
of the unit fractions with denominators 2 to 10 are given:

1/2 = 0.5
1/3 = 0.(3)
1/4 = 0.25
1/5 = 0.2
1/6 = 0.1(6)
1/7 = 0.(142857)
1/8 = 0.125
1/9 = 0.(1)
1/10 = 0.1

Where 0.1(6) means 0.16666666..., and has a 1-digit recurring cycle. It
can be seen that 1/7 has a 6-digit recurring cycle.

Find the value of d<1000 for which 1/d contains the longest recurring
cycle in its decimal fraction part.
*/

use euler::check_prime_i32;

fn main() {
    let mut max = 0;
    let mut max_p = 0;
    for i in 2..1000 {
	if check_prime_i32(i) {
	    let tmp = cycle_length(i);
	}
	if max < tmp {
	    max_p = i;
	    max = tmp;
	}
    }
    println!("{}", max_p);
}

fn cycle_length(n:i32) -> i32 {
    let mut a = 1.0;
    let mut t = 0;
    loop {
	a = (a * 10.0) % n as f32;
	println!("{}:{}", n, a);
	t+=1;
	if a as i32 == 1 {
	    break;
	}
    }
    return t;
}
