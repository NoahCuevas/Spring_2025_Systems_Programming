
/*
fn main() {

    let x = 3;

    let rem - x % 3;

    if rem == 0 {
        println!("Divisible");
    } else if rem == 1 {
        println!("Remainder = {}",rem);
    }
}
*/
    
/*
fn main() {
    let x = 5;
    let rem = x % 3;
    
    let phrase = match rem {
        0 => "Rem is Zero",
        1 => "Rem is One",
        2 => "Rem is Two",
        _ => "#",
    };

    println!("{}",phrase);
}
*/

/*
fn main() {
    let mut nums = vec![];
    //for each loop
    for idx in 0..10 {
        nums.push(idx);
    }
    println!("{:?}",nums);
    //println!("{}",nums);
}
*/


// Week 4 Day 1 (Module2/Preview 04rust-ownnership)    
    //drop([variable]); // same as free([variable]); from C++
    // can have multiple references to heap but only 2 MUTABLE references to heap


/*
//Week 5 Day 1
    Went over 2 leetcode problems: 121. Best Time to Buy and Sell Stock && 14. Longest Common Prefix
*/

/*
//Week 5 Day 2
//Also went over a leetcode problem: 1603. Design Parking System

#[derive(Debug)]
struct Car {
    body: String,
    year: u16,
    color: String,
}

// fn get_info(car:&Car) {
//     println!("{}", car.body);
//     println!("{}", car.year);
//     println!("{}", car.color);
// }

impl Car {
    fn new(b:String,y:u16,c:String) -> Self {
        Self {
            body: b,
            year: y,
            color: c,
        }
    }

    fn show_info(&self) {
        println!("{} {} {}", self.body, self.year, self.color);
    }

    fn change_color(&mut self, new_color:String) {
        self.color = new_color;
    }
}

fn main() {
    // let my_car = Car {
    //     body: "Sedan".to_string(),
    //     year: 2020,
    //     color: "Purple".to_string(),
    // };
    
    let mut my_car = Car::new("Sedan".to_string(),2020,"Purple".to_string());
    println!();
    
    println!("{:?}",my_car);
    println!();
    
    my_car.change_color("Black".to_string());
    my_car.show_info();
}
*/


/*
//Week 6 Day 1

use std::arch::asm;

fn main() {
    let message = b"Hello, direct syscall!\n";

    unsafe {
        // write syscall
        asm!(
            "mov rax, 1",  // syscall number for write
            "mov rdi, 1",  // file descriptor: 1 is stdout
            "syscall",
            in("rsi") message.as_ptr(),
            in("rdx") message.len(),
            out("rax") _,
            out("rcx") _,
            out("r11") _,
            clobber_abi("system")
        );

        // exit syscall
        asm!(
            "mov rax, 60", // syscall number for exit
            "xor rdi, rdi", // status code 0
            "syscall",
            options(noreturn)
        );
    }
}
*/


// Week 6 Day 2

use std::fs::File;
use::std::io::Write;

fn mcd ain(){
    let file = File::open("example.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
