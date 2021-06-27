use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

fn basic_stats(list: &Vec<i32>) -> (f64, f64, i32){
    let mut mean = 0.0;
    let mut median = 0.0;
    let mut mode = 0;
    let mut max = 0;
    let mut map = HashMap::new();
    let list_size = list.len();

    for &num in list {
        mean += num as f64;
        let count = map.entry(num).or_insert(0);
        *count += 1;
        if *count > max {
            mode = num;
            max = *count;
        }
    }

    mean = mean / (list_size as f64);

    if is_sorted(list) {
        let mid_index = list_size / 2;
        if list_size % 2 == 0 {
            median = (list[mid_index] as f64 + list[mid_index - 1] as f64 )/2.0 ;
        } else {
            median = list[mid_index] as f64;
        }
    } else {
        println!("The list is not sorted! (Default median = 0.0)");
    }

    (mean, median, mode)
}

enum SortOrder {
    Default,
    Reverse
}

use crate::SortOrder::{Default, Reverse};
fn is_sorted(list: &Vec<i32>) -> bool{
  is_sorted_helper(list, Default) || is_sorted_helper(list, Reverse)
}

fn is_sorted_helper(list: &Vec<i32>, order: SortOrder) -> bool {
    let mut prev_index = 0;

    for &num in &list[1..] {
        match order {
            SortOrder::Default => {
                if list[prev_index] > num {
                    return false 
                }  
            }
            SortOrder::Reverse => {
                if list[prev_index] < num {
                    return false 
                }  
            }
        }
        
        prev_index += 1
    }

    true 
}


fn to_pig_latin(word: &str, vowels: &HashSet<char>) -> String{
    // assume only english alphabets are in word and the word is at least 2 letters
    let first_char = word.chars().nth(0).unwrap();
    if vowels.contains(&first_char) {
        return vowel_format(word)
    } else {
        return consonant_format(word, &first_char)
    }
}

fn vowel_format(word: &str) -> String{
    format!("{}-hay", decapitalise(word))
}

fn consonant_format(word: &str, first_char: &char) -> String{
    format!("{}-{}ay", &word[1..], first_char.to_lowercase())
}

fn decapitalise(word: &str) -> String{
    let first_ch = word.chars().nth(0).unwrap();
    
    format!("{}{}", first_ch.to_lowercase(), &word[1..])
}

enum DisplayType {
    Company,
    Department
}

fn display_company(company: &HashMap<String, Vec<String>>) {
    for (department, _) in company {
        display_department_sorted(department, company, DisplayType::Company);
    }
}

fn display_department_sorted(department: &str, company: &HashMap<String, Vec<String>>, display_type: DisplayType) {
    match display_type {
        DisplayType::Company => {
            println!();
            println!(" {}", capitalise(department));
        }
        _ => ()
    }
       
    let employees = company.get(department);
    
    match employees {
        None => (),
        Some(value) => {
            print_employees(value);
        }
    }
}

fn print_employees(employees: &Vec<String>) {
    for person in employees {
        println!(" {}", capitalise(person)); 
    }
}

fn capitalise(word: &str) -> String{
    let split_word: Vec<&str> = word.split(' ').collect();
    let mut res: Vec<String> = vec![];

    for w in &split_word {
        let first_ch = w.chars().nth(0).unwrap();
        res.push(format!("{}{}", first_ch.to_uppercase(), &w[1..]));
    }

    res.join(" ")
}

fn main() {
    println!("\nMean Median Mode\n");

    let a = vec![1, 2, 3, 4, 5];
    let b = vec![10, 7, 2, -1, -1];
    let c = vec![1, 1, 3, 5, 4, 2, 4, 4, 3];
    let d = vec![-5, 1, 1, 1, 1, 2, 4];
    let e = vec![-27];

    for list in &[&a, &b, &c, &d, &e] {
        println!("{:?}", list);
        let (mean, median, mode) = basic_stats(list);
        println!("Mean = {} Median = {} Mode = {}\n", mean, median, mode);
    }

    println!("\nPig Latin\n");
    
    let a = String::from("Apple");
    let b = String::from("ace");
    let c = String::from("first");
    let d = String::from("legendary");
    let e = String::from("Face");
    
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let vowels: HashSet<char> = vowels.into_iter().collect();

    for word in &[&a, &b, &c, &d, &e] {
        println!("{} -> {}", word, to_pig_latin(word, &vowels));
    }

    println!("\nEmployee & Department\n");

    println!("Add employees to a department by specifying {{Add <first_name> to <department>}} \nFor example, \"Add Maya to Quantitative Trading\"\n");

    let mut quit = false;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    while !quit {
        let mut query = String::new();

        print!("Add an employee to a department or enter \"get list\", \"get <department>\" or \"quit\"\n> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut query)
            .expect("Failed to read line");

        query = query.trim().to_string();
        let query_list: Vec<&str> = query.split(' ').collect();

        if query.eq_ignore_ascii_case("quit") {
            println!("See you soon!");
            quit = true;

        } else if query.eq_ignore_ascii_case("get list") {
            display_company(&map) // dispays all departments (with employees sorted alphabetically)

        } else if query_list[0].eq_ignore_ascii_case("get") {
            let department = format!("{}", &query_list[1..].join(" "));
            if map.contains_key(&department.to_lowercase()) {
                display_department_sorted(&department.to_lowercase(), &map, DisplayType::Department);
            } else {
                println!("There are no employees in this department");
            }
            
        } else if query_list[0].eq_ignore_ascii_case("add") && query_list[2].eq_ignore_ascii_case("to") {
            let employee = query_list[1];
            let department = query_list[3..].join(" ");

            let employees_in_department = map.entry(department.to_lowercase()).or_insert(Vec::new());
            employees_in_department.push(employee.to_lowercase());
            employees_in_department.sort();                          // a min heap or max heap should be better but getting practice with Vecs
            println!("{:?}", map)
        } else  {
            invalid_query_message();
        }
    }

    fn invalid_query_message() {
        println!("Invalid query or command! Refer to instruction");
    }
}






