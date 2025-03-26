
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

/*
// Week 6 Day 2

use std::fs::File;
use::std::io::Write;

fn main(){
    let file = File::open("example.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
*/

/*
// Week 7 Day 1
// used 06rust-execute... and 07rust-python...
use std::process::Command;

fn main(){
    let output = Command::new("ls")
        .arg("-1")
        .output() //exec happens here
        .expect("Failed to execute command");
    
    let new_file_name = "hello_linux.txt";
    let result = Command::new("touch").arg(new_file_name).output();
    let msg = "UTRGV";

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}
*/


/*
// Week 7 Day 2

#[derive(Debug)]
enum Insurance{
    House,
    Car,
}


#[derive(Debug)]
struct Car{
    model:String,
}


#[derive(Debug)]
struct Person{
    name:String,
    insurances:Vec<Insurance>,
}


impl Person {
    fn new(n:String) -> Person {
        Person{
            name:n,
            insurances:vec![],
        }
    }
    fn add_insurance(&mut self,i:Insurance){
        self.insurances.push(i);
    }
    fn show_insurances(&self){
        println!("Hey I am {:?}. I have these insurances:", self);
        for i in self.insurances.iter(){
            match i {
                Insurance::Car => println!("I have insured my {:?}", self.car),
                Insurance::House => println!("I have insured my house"),
                _ => println!("Something else"),
            };
        }
    }
}


fn main(){
    let c = Insurance::Car;
    let h = Insurance::House;
    let l = Insurance::Life;
    let _car = Car {
        model:"BMW".to_string(),
    };
    let mut person = Person::new("John".to_string());

    person.add_insurance(c);
    person.add_insurance(h);
    person.add_insurance(l);
    person.show_insurances();
}
*/

/*
// Week 8 Day 1
// Did LeetCode #20 Valid Parentheses, #13 Roman to Integer, and #680 Valid Palindrome II
// Valid Parentheses NEED TO FIX
impl Solution {
    pub fn is_valid(s: String) -> bool {
        fn get_match(closingBrack){
            
        }

        let mut list = vec![];
        for c in s.chars() { 
            if (c == '(' | c == '{' | c == '['){
                list.push(c);
            } else{
                if(list.isEmpty()){
                    return false;
                } else{
                    match parentheses{
                    ')' => list.pop('('),
                    '}' => list.pop('{'),
                    ']' => list.pop('['),
                    } 
                }
            }
        }
        return true
    }
}

//Roman to Integer NEED TO FIX
use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut roman_nums: HashMap<char, i32> = HashMap::new();
        roman_nums.insert('I', 1);
        roman_nums.insert('V', 5);
        roman_nums.insert('X', 10);
        roman_nums.insert('L', 50);
        roman_nums.insert('C', 100);
        roman_nums.insert('C', 100);
        roman_nums.insert('D', 500);
        roman_nums.insert('M', 1000);

        let mut total = 0;
        let mut prev = 0;
        for num in s.chars() {
            if let Some(&value) = roman_nums.get(&char) {
                if value < prev {
                    total -= value;
                } else {
                    total += value;
                }
                prev = value;
            }
        }
        return total;
    }
}
*/


// Week 8 Day 2
fn main() {
    println!("Everything is good");
    // panic!("Crash the program, stop running, clean the memory");
    // println!("This won't be printed.");
    let v = vec![1, 2, 3];
    println!("{:?}", v[99]); // This will cause a panic because the index is out of bounds
}
