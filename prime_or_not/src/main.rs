
fn is_prime(num:i32) -> bool{
    if num <= 1{
        return false;
    };

    for i in 2..num/2+1 as i32{
        if num % i == 0 {
            return false;
        }
    };

    true
}


fn main() {
    let num  = 17;
    let result = is_prime(num);
    println!("{result}")

}
