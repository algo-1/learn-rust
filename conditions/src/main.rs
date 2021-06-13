fn print_arr_i32(arr: &[i32]) {
    for elem in arr.iter() {
        print!("{} ", elem);
    }
    println!();
}

fn print_arr_str(arr: &[&str]) {          
    for elem in arr.iter() {
        print!("{} ", elem);
    }
    println!();
}

fn fib_recursive(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib_recursive(n - 1) + fib_recursive(n - 2)
    }    
}

fn fib_iter(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let tup = (b, a + b);
        a = tup.0;
        b = tup.1;
    }
    a
}

fn fib_memo(n: i32) -> i32 {
    let mut dp: Vec<i32> = vec![0; (n+1) as usize];
    
    if n > 0 {
        dp[1] = 1;
    }

    for i in 2..n+1 {
        dp[i as usize] = dp[(i-1) as usize] + dp[(i-2) as usize];
    }

    dp[n as usize]
}

fn main() {
    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {}", number);


    let mut counter = 0;

    let result = loop {
        counter += 1;
            
        println!("count = {}", counter);
            
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {}", result);

    // countdown (while loop) 
    let mut num = 3;
    while num > 0 {
        println!("{}!", num);

        num -= 1;
    }
    println!("LIFTOFF!!! 🚀🚀");

    println!();

    // countdown (for loop)
    for num in (1..4).rev() {
        println!("{}!", num);
    }
    println!("LIFTOFF!!! 🚀🚀");

    let array: [i32; 5] = [1, -1, 24, 3, 0]; 
    print_arr_i32(&array);

    let array = ["yeahh!"; 5];
    print_arr_str(&array);


    // test fibonacci
    for x in 0..21 {
        println!("fib({}) = {} fib({}) = {} fib({}) = {}",x, fib_recursive(x), x, fib_iter(x), x, fib_memo(x));
    }

    println!("fib(45) = {}", fib_memo(45)); 
}
