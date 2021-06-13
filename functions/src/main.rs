fn another_function(x: i32, y: i32) {
    println!("The value of x is {} \nThe value of y is {}", x, y);
}

fn block_expression() {
    let _x = 5;

    let y = {
        let x:u8 = 3;
        x + 1           // no semicolon here as that will turn it to a statement (it's an expression)
    };

    println!("The value of y is: {}", y);
}

fn seven() -> i32 {
    7                   // no semicolon here as that will turn it to a statement (it's an expression)
}

fn main() {
    another_function(5, 9);
    println!();
    
    block_expression();
    println!();
    
    let x = seven();
    println!("The value of x is {}", x);

}
