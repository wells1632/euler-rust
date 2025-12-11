/*
Euler discovered the remarkable quadratic formula:

n^2 + n + 41

It turns out that the formula will produce 40 primes for the consecutive integer values
0 <= n <= 39. However, when n = 40, 40^2 + 40 + 41 = 40(40+1) + 41 is divisible by 41,
and certainly when n = 41, 41^2 + 41 + 41 is clearly divisible by 41.

The incredible formula n^2 - 79n + 1601 was discovered, which produces 80 primes for the
consecutive values 0 <= n <= 79. The product of the coefficients, -79 and 1601, is -126479.

Considering quadratics of the form:

n^2 + an + b, where |b| <= 1000 and |a| < 1000

where |n| is the modulus/absolute value of n

e.g. |11| = 11 and |-4| = 4

Find the product of the coefficients, a and b, for the quadratic expression that produces
the maximum number of primes for consecutive values of n, starting with n = 0.

*/

use euler::check_prime_i32;

fn main() {
    let mut count = 0;
    let mut primeval = 0;
    for a in 1..1000 {
	for b in 1..=1000 {
	    let mut n = 0;
	    while check_prime_i32(n*n+a*n+b) {
		println!("a:{} b:{} n:{}", a, b, n);
		n+=1;
	    }
	    if n > count {
		primeval = a * b;
		count=n;
	    }
	}
    }
    

    println!("{primeval}");
//    println!("{:#?}", primes);
}
