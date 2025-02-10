fn main() {
    /*
    let x = 3;

    let rem - x % 3;

    if rem == 0 {
        println!("Divisible");
    } else if rem == 1 {
        println!("Remainder = {}",rem);
    }
    */
    
    /*
    let x = 5;
    let rem = x % 3;
    
    let phrase = match rem {
        0 => "Rem is Zero",
        1 => "Rem is One",
        2 => "Rem is Two",
        _ => "#",
    };

    println!("{}",phrase);
    */


    let mut nums = vec![];
    //for each loop
    for idx in 0..10 {
        nums.push(idx);
    }
    println!("{:?}",nums);
    //println!("{}",nums);
}
