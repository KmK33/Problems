
fn kth_smallest(mut vec:Vec<i32>,k:usize) -> i32 {
    if vec.len() < k || k == 0{
        !panic!("K out of  length of vector ");
    };

    vec.sort();
    vec[k-1]


}



fn main() {
    let numbers = vec![1,2,3,4];
    let k = 1;
    let value = kth_smallest(numbers, k);
    println!(" smallest element  {value}");
}
