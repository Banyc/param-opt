use std::{num::NonZeroUsize, ops::Range};

use itertools::{Itertools, MultiProduct};

#[derive(Debug, Clone)]
pub struct GridSearch {
    cartesian_product: MultiProduct<Range<usize>>,
}
impl GridSearch {
    pub fn new(parameter_spaces: impl Iterator<Item = NonZeroUsize>) -> Self {
        let mut iterators = vec![];
        for space in parameter_spaces {
            let range = 0..space.get();
            iterators.push(range);
        }
        let cartesian_product = iterators.into_iter().multi_cartesian_product();

        Self { cartesian_product }
    }
}
impl Iterator for GridSearch {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        self.cartesian_product.next()
    }
    fn count(self) -> usize {
        self.cartesian_product.count()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.cartesian_product.size_hint()
    }
    fn last(self) -> Option<Self::Item> {
        self.cartesian_product.last()
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    use super::*;

    #[test]
    fn test_grid_search() {
        let spaces = [1, 2, 3];
        let spaces = spaces.into_iter().map(|x| NonZeroUsize::new(x).unwrap());
        let mut grid_search = GridSearch::new(spaces);
        assert_eq!(grid_search.next(), Some(vec![0, 0, 0]));
        assert_eq!(grid_search.next(), Some(vec![0, 0, 1]));
        assert_eq!(grid_search.next(), Some(vec![0, 0, 2]));
        assert_eq!(grid_search.next(), Some(vec![0, 1, 0]));
        assert_eq!(grid_search.next(), Some(vec![0, 1, 1]));
        assert_eq!(grid_search.next(), Some(vec![0, 1, 2]));
        assert_eq!(grid_search.next(), None);
    }
}
