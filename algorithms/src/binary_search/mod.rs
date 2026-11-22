#![allow(dead_code)]
pub(crate) fn binary_search(array: &[u32], target: u32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    if array.len() == 1 && array[0] == target {
        return Some(0);
    }
    if array.len() == 1 && array[0] != target {
        return None;
    }

    binary_search_helper(array, target, 0, array.len() - 1)
}

fn binary_search_helper(array: &[u32], target: u32, low: usize, heigh: usize) -> Option<usize> {
    if heigh < low || heigh >= array.len() {
        return None;
    }
    let mid = low + (heigh - low) / 2;
    if array[mid] == target {
        Some(mid)
    } else if target < array[mid] {
        if mid == 0 {
            return None;
        }
        binary_search_helper(array, target, low, mid - 1)
    } else {
        binary_search_helper(array, target, mid + 1, heigh)
    }
}

#[cfg(test)]
mod test {
    use crate::binary_search::binary_search;
    use std::assert_matches;

    #[test]
    fn empty_array_test() {
        let array: [u32; 0] = [];
        assert_matches!(binary_search(&array, 1), None);
    }

    #[test]
    fn single_element_test() {
        let array: [u32; 1] = [2];
        assert_matches!(binary_search(&array, 2), Some(0));
    }

    #[test]
    fn find_element_test() {
        let array: [u32; 5] = [1, 2, 3, 4, 5];
        assert_matches!(binary_search(&array, 3), Some(2));
        assert_matches!(binary_search(&array, 2), Some(1));
        assert_matches!(binary_search(&array, 1), Some(0));
        assert_matches!(binary_search(&array, 4), Some(3));
        assert_matches!(binary_search(&array, 5), Some(4));
    }

    #[test]
    fn not_find_element_test() {
        let array = [1, 2, 3];
        assert_matches!(binary_search(&array, 4), None);
    }
}
