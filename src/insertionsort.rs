use crate::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + std::fmt::Debug,
    {
        // [ 5 2 7 3 4 ]
        //       i
        //       j
        // [ 5 | 2 7 3 4 ]
        // [ 2 5 | 7 3 4 ]
        // [ 2 5 7 | 3 4 ]
        // [ 2 3 5 7 | 4 ]
        // [ 2 3 4 5 7 ]
        for i in 1..slice.len() {
            let mut j = i;
            while j > 0 && slice[j - 1] > slice[j] {
                slice.swap(j - 1, j);
                j -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{insertionsort::InsertionSort, Sorter};

    #[test]
    fn insertion_works() {
        let mut v = vec![-4, 3, 1, 10, 5];
        InsertionSort::sort(&mut v);
        assert_eq!(v, &[-4, 1, 3, 5, 10]);
    }
}
