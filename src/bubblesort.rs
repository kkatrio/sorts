use crate::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        let mut n = slice.len() - 1;
        while swapped {
            swapped = false;
            for i in 0..n {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
            n -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bubblesort::BubbleSort, Sorter};

    #[test]
    fn buublesort_works() {
        let mut v = vec![-4, 3, 1, 10, 5];
        BubbleSort::sort(&mut v);
        assert_eq!(v, &[-4, 1, 3, 5, 10]);
    }
}
