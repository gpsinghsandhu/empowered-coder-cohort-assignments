use std::vec;

// assumes sorted array
fn find_median(arr: &Vec<i32>) -> f32 {
    let size = arr.len();
    assert_ne!(size, 0);
    if size % 2 == 0 {
        let mid1 = *arr.get(size / 2).unwrap();
        let mid2 = *arr.get(size / 2 - 1).unwrap();
        return ((mid1 + mid2) as f32) / 2f32;
    }
    (*arr.get(size / 2).expect("some")) as f32
}

fn combine_and_find_median(arr1: &Vec<i32>, arr2: &Vec<i32>) -> f32 {
    if arr1.len() == 0 && arr2.len() == 0 {
        panic!("No median possible for both empty arrays");
    }
    let mut arr1_clone = arr1.clone();
    let mut arr2_clone = arr2.clone();
    arr1_clone.append(&mut arr2_clone);
    arr1_clone.sort();

    println!("Combined: {:?}", arr1_clone);
    find_median(&arr1_clone)
}


fn test_median(arr1: &Vec<i32>, arr2: &Vec<i32>) {
    println!("Median of combined {:?} and {:?} is {:?}", arr1, arr2, combine_and_find_median(&arr1, &arr2));
}

fn main() {
    let arr1 = vec![1,2,3];
    let arr2 = vec![4,5,6,7];
    test_median(&arr1, &arr2);

    let arr3 = vec![1,2,3];
    let arr4 = vec![4,5,6];
    test_median(&arr3, &arr4);

    // the following will fail
    // test_median(&vec![], &vec![])

    test_median(&vec![], &vec![1, 2, 3]);
    test_median(&vec![], &vec![1, 2]);
    test_median(&vec![], &vec![3]);
    test_median(&vec![2], &vec![3]);
    test_median(&vec![5, 4, 2, 3, 1], &vec![1, 2, 6, 4]);
}
