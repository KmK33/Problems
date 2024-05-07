fn merge_sorted_arrays(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }

   
    if i < a.len() {
        result.extend(&a[i..]);
    }

   
    if j < b.len() {
        result.extend(&b[j..]);
    }

    result
}

fn main() {
    let a = vec![1, 3, 5];
    let b = vec![2, 4, 6];
    let merged = merge_sorted_arrays(&a, &b);
    println!("{:?}", merged); // output [1, 2, 3, 4, 5, 6]
}

