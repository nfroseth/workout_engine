fn main() {
    
    println!("Hello, world!");
    let z = 5;

    // let x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    let x = vec![1, 2, 3]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y + z);
    
    println!("The value of x: {}", x);

    greet();

    println!("The value of: {}", fair_dice_roll());

    let s1 = std::str::from_utf8(
        &[240, 159, 141, 137]
    );

    println!("{:?}", s1); 
}

fn greet() {
    println!("Hi there!");
}

fn fair_dice_roll() -> i32 {
    4
}