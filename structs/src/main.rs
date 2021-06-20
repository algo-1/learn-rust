#[derive(Debug)]
struct Student {
    name: String,
    email: String,
    student_no: u64,
    year: u8,
    in_final_year: bool 
}

struct Coordinate(i64, i64, i64);
struct Color(u16, u16, u16);

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64
}
impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height 
    } 

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }

    fn square(length: f64) -> Rectangle {
        Rectangle {
            width: length,
            height: length 
        }
    }
}

fn build_new_student(name: String, email: String, student_no: u64) -> Student {
    Student {
        name, 
        email, 
        student_no, 
        year: 1,
        in_final_year: false
    }
}


fn main() {
    let student1 = Student {
        email: String::from("john221@test.edu"),
        name: String::from("John Stone"),
        in_final_year: false,
        year: 2,
        student_no: 1618719
    };

    let mut student2 = Student {
        name: String::from("Sarah Rice"),
        year: 3,
        in_final_year: student1.in_final_year,
        student_no: 1587293,
        email: String::from("sarah23@test.edu")
    };

    student2.in_final_year = true;

    let student3 = build_new_student(String::from("Arya Pitt"), String::from("arya11@test.edu"), 1682829);

    let student4 = Student {
        name: String::from("Jeff Hardy"),
        student_no: 1685279,
        email: String::from("jeff77@test.edu"),
        ..student3
    };

    for student in [&student1, &student2, &student3, &student4].iter() {
        println!("{}", student.name);
        println!("email: {}\nstudent no: {}\nyear: {}\n{}", 
        student.email, student.student_no, student.year, 
        if student.in_final_year {format!("{} is in final year", student.name)} else {format!("{} is not in final year", student.name)});
        println!();
    }

    let red = Color(256, 0, 0);

    let origin = Coordinate(0, 0, 0);

    println!("R value for red = {}\ny coordinate of origin = {}", red.0, origin.1);

    // using derive(Debug) to enable print for our Student struct 
    println!("\nstudent1 is {:?}", student1);   // normal print 
    println!("\nstudent3 is {:#?}", student3);  // pretty print 

    println!();

    for student in [&student1, &student2, &student3, &student4].iter() {
        println!("{:#?}\n", student);
    }

    
    let rect = Rectangle {
        width: 20.5,
        height: 15.0
    };

    println!("The area of the rectangle is {}", rect.area());

    let rect1 = Rectangle::square(22.5); 
    let rect2 = Rectangle {
        width: 20.0,
        height: 12.9
    };

    println!("\n{}", if rect1.can_hold(&rect2) {"rect1 can hold rect2"} else {"rect1 cannot hold rect2"});
}
