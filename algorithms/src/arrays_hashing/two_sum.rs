#![allow(dead_code)]

use std::collections::HashMap;

pub(crate) fn two_sum(array: &[u32], target: u32) -> Option<(usize, usize)> {
    let mut hash_table: HashMap<u32, usize> = HashMap::new();
    for (current_index, current_element) in array.iter().enumerate() {
        // checked_sub used to skip the insertion if the element is bigger than target

        if let Some(complement) = target.checked_sub(*current_element) {
            if let Some(prev_index) = hash_table.get(&complement) {
                return Some((*prev_index, current_index));
            }
            hash_table.insert(*current_element, current_index);
        }
    }
    None
}

pub(crate) fn two_sum_slow(array: &[u32], target: u32) -> Option<(usize, usize)> {
    for (idx, element) in array.iter().enumerate() {
        for (offset, element2) in array[idx + 1..].iter().enumerate() {
            if element.checked_add(*element2) == Some(target) {
                return Some((idx, idx + 1 + offset));
            }
        }
    }
    None
}

#[cfg(test)]
mod test {

    use crate::arrays_hashing::two_sum::{two_sum, two_sum_slow};

    #[test]
    fn success_two_sum() {
        // if many pair exist in array there may be different idices whose value is equal to target
        let array = [2, 1, 4, 3];
        let (i, j) = two_sum(&array, 5).unwrap();
        assert_eq!(array[i] + array[j], 5);
        let (i, j) = two_sum_slow(&array, 5).unwrap();
        assert_eq!(array[i] + array[j], 5);
    }

    #[test]
    fn fail_two_sum() {
        assert_eq!(two_sum(&[1, 2, 4], 10), None);
        assert_eq!(two_sum_slow(&[1, 2, 4], 10), None);
    }

    #[test]
    fn empty_array_test() {
        assert_eq!(two_sum(&[], 1), None);
        assert_eq!(two_sum_slow(&[], 1), None)
    }

    #[test]
    fn one_element_test() {
        assert_eq!(two_sum(&[1], 1), None);
        assert_eq!(two_sum_slow(&[1], 1), None)
    }

    #[test]
    fn duplicate_same_element_twice_test() {
        assert_eq!(two_sum(&[3, 3], 6), Some((0, 1)));
        assert_eq!(two_sum_slow(&[3, 3], 6), Some((0, 1)));
    }

    #[test]
    fn zero_test() {
        assert_eq!(two_sum(&[0, 5], 5), Some((0, 1)));
        assert_eq!(two_sum_slow(&[0, 5], 5), Some((0, 1)));
    }
}
