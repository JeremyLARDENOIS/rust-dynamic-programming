use std::i32;

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
    weight: i32,
}

fn bellman_ford(edges: &[Edge], num_vertices: usize, start: usize) -> Vec<i32> {
    let mut distances = vec![i32::MAX; num_vertices];
    distances[start] = 0;

    println!("distances: {:?}", distances);

    for _ in 0..num_vertices - 1 {
        let mut updated = false;
        for edge in edges {
            if distances[edge.from] != i32::MAX && distances[edge.from] + edge.weight < distances[edge.to] {
                distances[edge.to] = distances[edge.from] + edge.weight;
                updated = true;
                println!("distances: {:?}", distances);
            }
        }
        if !updated {
            break;
        }
    }

    for edge in edges {
        if distances[edge.from] != i32::MAX && distances[edge.from] + edge.weight < distances[edge.to] {
            println!("Le graphe contient un cycle négatif.");
            break;
        }
    }

    distances
}

fn main() {
    // Bellman-Ford est un algorithme qui permet de trouver les plus courts chemins depuis un sommet.
    // Il fonctionne sur des graphes pondérés et orientés ou non.
    // Ici, on cherchera les plus courts chemins depuis le sommet 0.
    let edges = vec![
        Edge { from: 0, to: 1, weight: -1 },
        Edge { from: 0, to: 2, weight: 4 },
        Edge { from: 1, to: 2, weight: 3 },
        Edge { from: 1, to: 3, weight: 2 },
        Edge { from: 1, to: 4, weight: 2 },
        Edge { from: 3, to: 2, weight: 5 },
        Edge { from: 3, to: 1, weight: 1 },
        Edge { from: 4, to: 3, weight: -3 },
    ];

    let num_vertices = 5;
    let start_vertex = 0;

    let distances = bellman_ford(&edges, num_vertices, start_vertex);

    println!("Distances depuis le sommet {:?} :", start_vertex);
    for (index, distance) in distances.iter().enumerate() {
        println!("{} -> {}: {:?}", start_vertex, index, distance);
    }
}
