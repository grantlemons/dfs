use std::{collections::HashSet, hash::Hash};

pub fn can_reach<N, FN, IN, FS>(start: N, successors: FN, success: FS) -> bool
where
    N: Clone + Eq + Hash,
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: Fn(&N) -> bool,
{
    let mut visited: HashSet<N> = HashSet::new();
    success(&start) || traverse(&start, &successors, &success, &mut visited)
}

fn traverse<N, FN, IN, FS>(
    start: &N,
    successors: &FN,
    success: &FS,
    visited: &mut HashSet<N>,
) -> bool
where
    N: Clone + Eq + Hash,
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: Fn(&N) -> bool,
{
    visited.insert(start.clone());
    successors(start).into_iter().any(|n| {
        success(&n) || (!visited.contains(&n) && traverse(&n, successors, success, visited))
    })
}

#[cfg(test)]
mod tests {
    use crate::can_reach;

    #[test]
    fn example() {
        assert!(can_reach(
            1,
            |&n| vec![n + 1, n * n].into_iter().filter(|&x| x <= 17),
            |&n| n == 17
        ));
        assert!(!can_reach(
            2,
            |&n| vec![n * n].into_iter().filter(|&x| x <= 18),
            |&n| n == 18
        ));
        assert!(can_reach(
            2,
            |&n| vec![n * n].into_iter().filter(|&x| x <= 16),
            |&n| n == 16
        ));
    }

    #[test]
    fn circular() {
        assert!(can_reach(
            5,
            |&n| vec![n - 1, n + 1]
                .into_iter()
                .filter(|&x| x <= 10 && x >= 0),
            |&n| n == 10
        ));
    }
}
