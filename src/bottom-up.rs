// bottom-up
fn fib(n: u32) -> u32 {
    let mut memo = vec![0; (n+1) as usize];

    memo[0] = 0;
    memo[1] = 1;

    println!("On initialise le tableau memo avec les valeurs 0 et 1");
    println!("memo = {:?}", memo);
    println!("On remplit le tableau memo avec les valeurs suivantes :");
    for i in 2..=n {
        memo[i as usize] = memo[(i-1) as usize] + memo[(i-2) as usize];
    }
    println!("memo = {:?}", memo);

    println!("On retourne la valeur de la case memo[{}]", n);
    return memo[n as usize];
}

fn main() {
    let n = 10;

    let result = fib(n);

    println!("Le {}ème nombre de la suite de Fibonacci est {}", n, result);
}
