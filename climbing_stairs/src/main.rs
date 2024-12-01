use std::io;

fn main() {
    println!("Enter the number of stairs: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n = input.trim().parse().expect("Please type a number!");
    
    let ans = climbing_stairs(n);
    println!("Ways to climbing the stair: {ans}");
}

fn climbing_stairs(n : i32) -> i32 {
    if n == 1 {
        return 1;
    } else if n == 2 {
        return 2;
    }

    let mut memo: Vec<i32> = vec![0; n as usize];
    memo[0] = 1;
    memo[1] = 2;

    for i in 2..n as usize{
        memo[i] = memo[i - 1] + memo[i - 2];
    }

    return memo[(n-1) as usize];
}
