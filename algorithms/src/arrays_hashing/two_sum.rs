#![allow(dead_code)]

use std::collections::HashMap;

pub(crate) fn two_sum(array: &[u32], target: u32) -> Option<(usize, usize)> {
    let mut hash_table: HashMap<u32, usize> = HashMap::new();
    for (current_index, current_element) in array.iter().enumerate() {
        if let Some(complement) = target.checked_sub(*current_element) {
            if let Some(prev_index) = hash_table.get(&complement) {
                return Some((*prev_index, current_index));
            }
            hash_table.insert(*current_element, current_index);
        }
    }
    None
}

#[cfg(test)]
mod test {

    use std::assert_matches;

    use crate::arrays_hashing::two_sum::two_sum;

    #[test]
    fn success_two_sum() {
        let array = [1, 2, 3, 4, 5];
        assert_eq!(two_sum(&array, 8), Some((2, 4)));
    }

    #[test]
    fn fail_two_sum() {
        let array = [1, 2, 3, 4, 5];
        assert_matches!(two_sum(&array, 10), None);
    }
}
