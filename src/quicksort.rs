use crate::Sorter;

pub struct QuickSort;

fn quicksort<T>(slice: &mut [T])
where
    T: Ord + std::fmt::Debug,
{
    match slice.len() {
        0 | 1 => return,
        _ => {}
    }

    println!("slice: {:?}", slice);
    let (pivot, rest) = slice.split_first_mut().expect("slice is empty");
    let mut left = 0;
    let mut right = rest.len() - 1;

    while left <= right {
        if rest[left] < *pivot {
            left += 1;
        } else if rest[right] > *pivot {
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            //println!("left: {}", left);
            //println!("right: {}", right);
            rest.swap(left, right);
            //println!("after SWAP: {:?}", rest);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }
    // place the pivot between the two sub-slices
    left = left + 1;
    slice.swap(0, left - 1);
    //println!("slice after placing the pivot: {:?}", slice);

    let (left, right) = slice.split_at_mut(left);
    quicksort(left);
    quicksort(right);
}

impl Sorter for QuickSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + std::fmt::Debug,
    {
        quicksort(slice)
    }
}

#[cfg(test)]
mod tests {
    use crate::{quicksort::QuickSort, Sorter};

    #[test]
    fn quicksort_works() {
        let mut v = vec![-4, 3, 1, 10, 5];
        QuickSort::sort(&mut v);
        assert_eq!(v, &[-4, 1, 3, 5, 10]);
    }
}
