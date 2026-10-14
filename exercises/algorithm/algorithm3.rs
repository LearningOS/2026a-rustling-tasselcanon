/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
use std::cmp::Ordering;

fn sort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    let last = array.len() - 1;
    array.swap(array.len() / 2, last);

    let mut lt = 0;
    let mut i = 0;
    let mut gt = last;

    while i < gt {
        match array[i].cmp(&array[last]) {
            Ordering::Less => {
                array.swap(i, lt);
                lt += 1;
                i += 1;
            }
            Ordering::Equal => {
                i + 1;
            }
            Ordering::Greater => {
                gt -= 1;
                array.swap(i, gt);
            }
        }
    }
    array.swap(gt, last);
    let (left_and_equal, right) = array.split_at_mut(gt + 1);
    let (left, _) = left_and_equal.split_at_mut(lt);

    sort(left);
    sort(right);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
