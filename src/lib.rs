trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + std::fmt::Debug;
}

#[allow(dead_code)]
fn sort<T, S>(slice: &mut [T])
where
    T: Ord + std::fmt::Debug,
    S: Sorter,
{
    S::sort(slice);
}

mod bubblesort;
mod insertionsort;
mod quicksort;
mod selectionsort;

#[cfg(test)]
mod tests {

    use super::*;
    struct StdSort;
    impl Sorter for StdSort {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }

    #[test]
    fn std_works() {
        let mut v = vec![4, 3, 1, 5];
        sort::<_, StdSort>(&mut v);
        assert_eq!(v, &[1, 3, 4, 5]);
    }
}
