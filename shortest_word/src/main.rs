

fn shortest_word(string:String) -> String{
    if string.is_empty(){
        return String::new();
    };

    let vec:Vec<&str> = string.split_whitespace().collect();

    let mut shortest = vec[0];


    for &word in &vec[1..] {
        if word.len() < shortest.len(){
            shortest = word;
        }
    };

    shortest.to_string()




}


fn main() {
    let string: String = String::from("Hello worlds ");
    let result = shortest_word(string);

    println!("{result}");

    
}
