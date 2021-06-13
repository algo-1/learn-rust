#![allow(unused)]
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let x = 57u8;
    println!("{}", x);
    let x = 5/2;
    println!("{}", x);
    let x = 5.015678/2.0;
    println!("{}", x);

    // tuple type 
    let tup: (i32, f64, u8, i8) = (500, 3.4, 255, -128);
    let (x, y, z, w) = tup;
    println!("The value of y is {}", y);
    println!("The value of y is {}", tup.1);

    // array type 
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [-7; 5];
    println!("The second elements in the arrays a and b are {} and {} respectively", a[1], b[1]);

}
