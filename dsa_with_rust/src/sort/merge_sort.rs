#[allow(dead_code)]
pub fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
    let n: usize = arr.len();

    if n == 1 {
        return arr;
    }

    let mid = n / 2;
    let left = get_vec(&arr[..mid]);
    let right = get_vec(&arr[mid..]);
    // println!("from merge_sort: left:{:?} right:{:?}", left, right);

    let _left_sorted = merge_sort(left);
    let _right_sorted = merge_sort(right);

    let temp = merge_arr(_left_sorted, _right_sorted);
    // println!("from merge_srot:Arr:{:?} Res:{:?}", arr, temp);
    temp
}

#[allow(dead_code)]
fn merge_arr(mut _left_arr: Vec<i32>, mut _right_arr: Vec<i32>) -> Vec<i32> {
    let mut new_arr: Vec<i32> = Vec::new();
    while !_left_arr.is_empty() && !_right_arr.is_empty() {
        if _left_arr.get(0) < _right_arr.get(0) {
            new_arr.push(_left_arr.remove(0))
        } else {
            new_arr.push(_right_arr.remove(0))
        }
    }

    while !_left_arr.is_empty() {
        new_arr.push(_left_arr.remove(0))
    }

    while !_right_arr.is_empty() {
        new_arr.push(_right_arr.remove(0))
    }

    new_arr
}

#[allow(dead_code)]
fn get_vec(ref_arr: &[i32]) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for val in ref_arr {
        let new_val = val.to_owned();
        new_vec.push(new_val);
    }
    new_vec
}
