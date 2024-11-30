fn main() {
    // Test 1: Basic unsorted array
    let mut arr1 = [3, 2, 1, 5, 6, 4];
    test_sort(&mut arr1, "Basic unsorted");

    // Test 2: Already sorted array
    let mut arr2 = [1, 2, 3, 4, 5, 6];
    test_sort(&mut arr2, "Already sorted");

    // Test 3: Reverse sorted array
    let mut arr3 = [6, 5, 4, 3, 2, 1];
    test_sort(&mut arr3, "Reverse sorted");

    // Test 4: Array with duplicates
    let mut arr4 = [4, 2, 4, 1, 2, 4];
    test_sort(&mut arr4, "With duplicates");

    // Test 5: Small array
    let mut arr5 = [2, 1];
    test_sort(&mut arr5, "Small array");

    // Test 6: Single element
    let mut arr6 = [1];
    test_sort(&mut arr6, "Single element");

    // Test 7: Larger array
    let mut arr7 = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    test_sort(&mut arr7, "Larger array");

    // Test 8: Array with negative numbers
    let mut arr8 = [-5, 2, -8, 10, -3, 6];
    test_sort(&mut arr8, "With negative numbers");
}

fn test_sort(arr: &mut [i32], test_name: &str) {
    println!("\nTesting {}", test_name);
    println!("Before: {:?}", arr);
    let len = arr.len();
    quicksort(arr, 0, len - 1);
    println!("After:  {:?}", arr);
    
    // Verify the array is sorted
    let is_sorted = arr.windows(2).all(|w| w[0] <= w[1]);
    println!("Correctly sorted: {}", is_sorted);
}

// choose a pivot
// partition the array: smaller than the pivot to the left, larger to the right
// recursively apply steps 1 and two to the smaller arrays

fn quicksort(a: &mut[i32], start: usize, end: usize) {
    if start < end {
        let pivot = partition(a, start, end);
        if pivot > 0 {
            quicksort(a, start, pivot - 1);
        }
        quicksort(a, pivot + 1, end);
    }
}

fn partition(a: &mut[i32], start: usize, end: usize) -> usize {
    let pivot = a[end];
    let mut i = start;
    for j in start..end {
        if a[j] < pivot {
            let temp = a[i];
            a[i] = a[j];
            a[j] = temp;
            i += 1;
        }
    }
    let temp = a[i];
    a[i] = a[end];
    a[end] = temp;
    return i;
}
