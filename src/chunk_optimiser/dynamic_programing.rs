use std::cmp::Ordering;

/// Chooses the best successor j for index i
fn best_successor(
    i: usize,
    punishments: &[usize],
    dp_cost: &[usize],
    n: usize,
) -> (usize, usize) {
    (i + 1..=i + n)
        .map(|j| (punishments[i] + dp_cost[j], j))
        .min_by(|(cost_a, j_a), (cost_b, j_b)| {
            match cost_a.cmp(cost_b) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => j_b.cmp(j_a), // prefer larger jump
            }
        })
        .unwrap()
}

/// Builds dp_cost and next arrays
fn build_dp_tables(
    punishments: &[usize],
    n: usize,
) -> (Vec<usize>, Vec<Option<usize>>) {
    let len = punishments.len();
    let inf = usize::MAX;
    let mut dp_cost = vec![inf; len];
    let mut next = vec![None; len];

    for i in (0..len).rev() {
        if i + n >= len {
            dp_cost[i] = punishments[i];
            next[i] = None;
        } else {
            let (cost, j) = best_successor(i, punishments, &dp_cost, n);
            dp_cost[i] = cost;
            next[i] = Some(j);
        }
    }

    (dp_cost, next)
}

/// create the optimal chunk boundaries
fn reconstruct_path(next: &[Option<usize>]) -> Vec<usize> {
    let mut path = Vec::new();
    let mut i = 0;

    loop {
        path.push(i);
        match next[i] {
            Some(j) => i = j,
            None => break,
        }
    }

    path
}

/// find the optimal chunk indices given the punishments and the max chunk size
pub fn cheapest_path_indices(
    punishments: &[usize],
    n: usize,
) -> Vec<usize> {
    assert!(!punishments.is_empty());
    assert!(n > 0);

    let (_dp_cost, next) = build_dp_tables(punishments, n);
    reconstruct_path(&next)
}
