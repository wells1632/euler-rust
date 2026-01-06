fn count_ways(amount: u32, coins: &[u32], index: usize) -> u64 {
    // Base cases
    if amount == 0 {
	return 1;
    }
    if index >= coins.len() {
	return 0;
    }

    // Check if we can use the current coin before subtracting
    let use_coin = if amount >= coins[index] {
	count_ways(amount - coins[index], coins, index)
    } else {
	0
    };
    let skip_coin = count_ways(amount, coins, index + 1);

    use_coin + skip_coin
}

fn main() {
    // UK coins in pence: 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), £2 (200p)
    let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
    let amount = 200; // 2 pounds = 200 pence

    let ways = count_ways(amount, &coins, 0);
    println!("Number of ways to make £2: {}", ways);
}
