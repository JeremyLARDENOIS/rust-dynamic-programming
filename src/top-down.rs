// Top-down dynamic programming - memoization
fn fib(n: u32, memo: &mut Vec<u32>) -> u32 {
    println!("Searching for fib({})", n);
    if n == 0 || n == 1 {
        println!("We found fib({}) = {}", n, n);
        return n;
    }

    if memo[n as usize] == 0 {
        println!("We didn't find fib({}) in memo, so we calculate it", n);
        memo[n as usize] = fib(n-1, memo) + fib(n-2, memo);
        println!("We know now that fib({}) = {}, so we store it in memo", n, memo[n as usize]);
    }

    println!("We already did fib({}) = {} before, so we return it", n, memo[n as usize]);
    return memo[n as usize];
}

fn main() {
    let n = 10; // On veut le 10ème nombre de la suite de Fibonacci
    let mut memo = vec![0; (n+1) as usize]; // initialiser le tableau pour stocker les résultats précédents

    let result = fib(n, &mut memo);

    println!("Le {}ème nombre de la suite de Fibonacci est {}", n, result);
}
