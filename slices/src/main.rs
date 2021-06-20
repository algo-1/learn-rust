use std::io::{self, Write};

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..].trim_end()
}



fn main() {
    let mut sentence = String::new();
       
    print!("Enter a sentence: ");
    io::stdout().flush().expect("unable to flush stdout");

    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");
    
    println!("{}", first_word("Hello!"));
    println!("The first word is '{}'", first_word(&sentence));

    let arr = [1, 4, 3, 5, 9, 0]; 
    assert_eq!(&arr[1..], [4, 3, 5, 9, 0]); println!("\nNO ERRORS!");
}
