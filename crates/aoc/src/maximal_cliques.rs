#![allow(clippy::many_single_char_names)]

// SPDX-License-Identifier: MIT OR Apache-2.0

// From https://github.com/petgraph/petgraph/pull/662

use petgraph::visit::{GetAdjacencyMatrix, IntoNeighbors, IntoNodeIdentifiers};

use std::collections::HashSet;
use std::hash::Hash;

/// Finds maximal cliques containing all the vertices in r, some of the
/// vertices in p, and none of the vertices in x.
fn bron_kerbosch_pivot<G>(
    g: G,
    adj_mat: &G::AdjMatrix,
    r: HashSet<G::NodeId>,
    mut p: HashSet<G::NodeId>,
    mut x: HashSet<G::NodeId>,
) -> Vec<HashSet<G::NodeId>>
where
    G: GetAdjacencyMatrix + IntoNeighbors,
    G::NodeId: Eq + Hash,
{
    let mut cliques = Vec::with_capacity(1);
    if p.is_empty() {
        if x.is_empty() {
            cliques.push(r);
        }
        return cliques;
    }
    // pick the pivot u to be the vertex with max degree
    let u = p.iter().max_by_key(|&v| g.neighbors(*v).count()).unwrap();
    let mut todo = p
        .iter()
        .filter(|&v| *u == *v || !g.is_adjacent(adj_mat, *u, *v) || !g.is_adjacent(adj_mat, *v, *u)) //skip neighbors of pivot
        .copied()
        .collect::<Vec<G::NodeId>>();
    while let Some(v) = todo.pop() {
        let neighbors: HashSet<_> = g.neighbors(v).collect();
        p.remove(&v);
        let mut next_r = r.clone();
        next_r.insert(v);

        let next_p = p
            .intersection(&neighbors)
            .copied()
            .collect::<HashSet<G::NodeId>>();
        let next_x = x
            .intersection(&neighbors)
            .copied()
            .collect::<HashSet<G::NodeId>>();

        cliques.extend(bron_kerbosch_pivot(g, adj_mat, next_r, next_p, next_x));

        x.insert(v);
    }

    cliques
}

pub fn maximal_cliques<G>(g: G) -> Vec<HashSet<G::NodeId>>
where
    G: GetAdjacencyMatrix + IntoNodeIdentifiers + IntoNeighbors,
    G::NodeId: Eq + Hash,
{
    let adj_mat = g.adjacency_matrix();
    let r = HashSet::new();
    let p = g.node_identifiers().collect::<HashSet<G::NodeId>>();
    let x = HashSet::new();
    bron_kerbosch_pivot(g, &adj_mat, r, p, x)
}
