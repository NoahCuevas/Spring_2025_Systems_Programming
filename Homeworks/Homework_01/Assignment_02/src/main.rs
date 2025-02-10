fn is_even(n:i32) ->  bool {
    n % 2 == 0
}

fn main() {
    let nums = [1,8,12,3,17,0,21,11,29,5];

    for &num in &nums {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{} -> FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{} -> Fizz", num);
        } else if num % 5 == 0 {
            println!("{} -> Buzz", num);
        } else {
            println!("{} -> {}", num, if is_even(num) { "Even" } else { "Odd" });
        }
    }

    let mut sum = 0;
    let mut i = 0;

    while i < nums.len() {
        sum += nums[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);

    let mut biggest = nums[0];
    for &num in &nums {
        if num > biggest {
            biggest = num;
        }
    }
    println!("Biggest number: {}", biggest);
}
