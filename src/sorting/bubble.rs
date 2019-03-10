pub fn bubble_sort(unsorted: Vec<u32>) -> Vec<u32> {
    let mut sorted = unsorted.clone();
    let vector_length = sorted.len();
    let mut swapped = false;

    // The worst case complexity is O(n*n) where the vector is in reverse
    // order and we have to iterate over it n times.
    for iteration_number in 0..vector_length - 1 {
        // Every iteration, the highest value is moved to the end of the vector,
        // so we can ignore that value during next iteration.
        for i in 0..vector_length - 1 - iteration_number {
            if sorted[i] > sorted[i + 1] {
                sorted.swap(i, i + 1);

                swapped = true;
            }
        }

        // If no values were swapped, the vector is already sorted and we can
        // break early. The best case complexity is O(n).
        if !swapped {
            break;
        }
    }

    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_unsorted_vector() {
        assert_eq!(bubble_sort(vec![3, 4, 1, 5, 2]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_reversed_vector() {
        assert_eq!(bubble_sort(vec![5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_sorted_vector() {
        assert_eq!(bubble_sort(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }
}