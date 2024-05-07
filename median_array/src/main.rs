fn median(vec:Vec<i32>) -> i32{
    let len = vec.len();
    let  med;
    if len % 2 == 0 {
        med = (len+1)/2;
    }else {
        med = (len/2) + ((len/2)+1)/2;
    };

    med as i32
}


fn main() {
    let vec = vec![1,2,3,4,5,6];
    let result  = median(vec);

    println!("{result}")
}
