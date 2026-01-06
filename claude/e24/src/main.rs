fn main() {
    let mut digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let n = 1_000_000 - 1; // Convert to 0-indexed

    let result = kth_permutation(&mut digits, n);

    println!("The millionth lexicographic permutation is: {:?}", result);
}

fn kth_permutation(digits: &mut Vec<i32>, mut k: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let mut available = digits.clone();
    let n = available.len();

    // Precompute factorials
    let mut factorials = vec![1usize; n];
    for i in 1..n {
	factorials[i] = factorials[i - 1] * i;
    }

    for i in 0..n {
	let factorial = factorials[n - 1 - i];
	let index = k / factorial;
	result.push(available.remove(index));
	k %= factorial;
    }

    result
}
