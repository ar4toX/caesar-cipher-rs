use std::io;

fn main() {
    //Values and things to use
    let abc: &str="abcdefghijklmnopqrstuvwxyz";
    let mut plainText = String::new();
    let mut key= String::new();

    //Gather data from user
    println!("Enter the message to cipher: ");
    io::stdin().read_line(&mut plainText)
        .expect("Input failed :(");

    println!("Enter the desired key: ");
    io::stdin().read_line(&mut key)
        .expect("Input failed :(");

    let key = key.trim().parse::<i32>()
        .expect("Invalid entry");

    //Create encrypted message

    //Display message
    println!("{} and {} with {}", abc, plainText, key);
}
