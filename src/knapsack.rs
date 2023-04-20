fn knapsack(w: i32, weights: &[i32], values: &[i32], n: usize) -> i32 {
    let mut dp = vec![vec![0; (w + 1) as usize]; n + 1];

    println!("On initialise le tableau dp avec des 0");
    println!("dp est un tableau de taille {}x{}", n + 1, w + 1);

    for i in 1..=n {
        for j in 1..=(w as usize) {
            if weights[i - 1] <= j as i32 {
                dp[i][j] = (values[i - 1] + dp[i - 1][(j - weights[i - 1] as usize)]).max(dp[i - 1][j]);
                println!("Objet {} (poids: {}, valeur: {}) peut être inclus. dp[{}][{}] = {}", i, weights[i - 1], values[i - 1], i, j, dp[i][j]);
            } else {
                dp[i][j] = dp[i - 1][j];
                println!("Objet {} (poids: {}, valeur: {}) ne peut pas être inclus. dp[{}][{}] = {}", i, weights[i - 1], values[i - 1], i, j, dp[i][j]);
            }
        }
    }
    println!("dp: {:?}", dp);

    dp[n][w as usize]
}

fn main() {
    // Le problème du sac à dos est un problème d'optimisation combinatoire dans lequel on cherche à
    // maximiser la valeur des objets mis dans un sac à dos, sachant que le sac à dos a une capacité
    // maximale.
    let w = 50; // capacité maximale du sac à dos
    let weights = vec![10, 20, 30]; // poids des objets
    let values = vec![60, 100, 120]; // valeur des objets
    // Notez que l'on aurait pu utiliser une HashMap pour stocker les objets et des structures pour
    // stocker les poids et les valeurs.

    let n = weights.len(); // nombre d'objets

    let result = knapsack(w, &weights, &values, n);

    println!("La valeur maximale pouvant être mise dans le sac à dos est {}", result);
}
