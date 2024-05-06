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

    //Create and display ciphertext
    caesar_it(abc, plainText, key);
}

fn caesar_it(abc: &str, plainText: String, key: i32){
    let mut cipherText= String::from("hey guys");

    println!("the ciphertext is: {}", cipherText);
}
