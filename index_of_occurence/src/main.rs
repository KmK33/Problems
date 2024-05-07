
fn first_occurence(vec:&Vec<i32>,target:i32) -> i32{
    vec.binary_search(&target).map(|x| x as i32).unwrap_or_else(|_x| {
        print!("element not found\n");
        -1 
    }) as i32
}



fn main() {
    let vec = vec![1,2,3,4];
    let index = first_occurence(&vec, 0);
    println!("{index}")
}
