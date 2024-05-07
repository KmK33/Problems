

fn longes_common_prefix(vec:Vec<String>) -> String{
    if vec.is_empty(){
        return String::new();
    };

    let mut prefix = vec[0].clone();

    for s in &vec[1..]{
        while !s.starts_with(&prefix) {
            if prefix.is_empty(){
                return prefix;
            };
            prefix.pop();
        }
    };

    prefix
    
}



fn main() {
    

    let vec: Vec<String> = vec![
        String::from("Heello"),
        String::from("Heey"),
        String::from("Heell"),
    ];
    let result = longes_common_prefix(vec);
    println!("{result}")
}
