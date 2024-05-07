fn is_palindrome(string:String ) -> bool{
    let clean_string: Vec<char> = string.chars()
    .filter(|c| c.is_alphanumeric())
    .flat_map(|c| c.to_lowercase())
    .collect();

if clean_string.is_empty() {
    return true; 
}

let mut start = 0;
let mut end = clean_string.len() - 1;

    while start < end {
        if clean_string[start] != clean_string[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }

    true

}
fn main() {
   

    let string  = String::from("race:cars.");
    let result = is_palindrome(string);
    println!("{result}");
}
