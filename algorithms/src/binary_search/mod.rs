#![allow(dead_code)]
pub(crate) fn binary_search(array: &[u32], target: u32) -> Option<usize> {
    binary_search_helper(array, target, 0, array.len())
}

fn binary_search_helper(array: &[u32], target: u32, low: usize, high: usize) -> Option<usize> {
    if low >= high {
        return None;
    }
    let mid = low + (high - low) / 2;
    if array[mid] == target {
        Some(mid)
    } else if target < array[mid] {
        binary_search_helper(array, target, low, mid)
    } else {
        binary_search_helper(array, target, mid + 1, high)
    }
}

#[cfg(test)]
mod test {
    use crate::binary_search::binary_search;

    #[test]
    fn empty_array_test() {
        assert_eq!(binary_search(&[], 1), None);
    }

    #[test]
    fn single_element_test() {
        assert_eq!(binary_search(&[2], 2), Some(0));
    }

    #[test]
    fn find_element_test() {
        let array: [u32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&array, 3), Some(2));
        assert_eq!(binary_search(&array, 2), Some(1));
        assert_eq!(binary_search(&array, 1), Some(0));
        assert_eq!(binary_search(&array, 4), Some(3));
        assert_eq!(binary_search(&array, 5), Some(4));
    }

    #[test]
    fn not_find_element_test() {
        assert_eq!(binary_search(&[1, 2, 3], 4), None);
    }

    #[test]
    fn smaller_then_all_item_test() {
        assert_eq!(binary_search(&[4, 5, 6, 7, 8], 1), None);
    }

    #[test]
    fn duplicate_element_test() {
        let array = [4, 5, 6, 7, 7, 8];
        let idx = binary_search(&array, 7).expect("element not found");
        assert_eq!(array[idx], 7);
    }

    #[test]
    fn even_length_array() {
        assert_eq!(binary_search(&[1, 2, 3, 4], 4), Some(3));
    }

    #[test]
    fn gap_in_the_middle_test() {
        assert_eq!(binary_search(&[1, 3, 5], 2), None);
    }
}
