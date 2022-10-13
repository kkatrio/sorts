use crate::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + std::fmt::Debug,
    {
        // i j
        // 3 2 5 4 1
        //   i j
        // 1 2 5 4 3
        for i in 0..slice.len() {
            let mut k = i;
            let mut min_found = false;
            for j in i + 1..slice.len() {
                if slice[j] < slice[k] {
                    min_found = true;
                    k = j;
                }
            }
            if min_found {
                slice.swap(i, k);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{selectionsort::SelectionSort, Sorter};

    #[test]
    fn selection_works() {
        let mut v = vec![-4, 3, 1, 10, 5];
        SelectionSort::sort(&mut v);
        assert_eq!(v, &[-4, 1, 3, 5, 10]);
    }
}
