use std::io;

fn main() {
    //Values and things to use
    let abc=
    ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
     'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
     's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut plaintext = String::new();
    let mut key= String::new();

    //Gather data from user
    println!("Enter the message to cipher: ");
    io::stdin().read_line(&mut plaintext)
        .expect("Input failed :(");
    println!("Enter the desired key: ");
    io::stdin().read_line(&mut key)
        .expect("Input failed :(");
    let key = key.trim().parse::<usize>()
        .expect("Invalid entry");

    //Make vector list for input characters
    let letters: Vec<char> = plaintext.chars().collect();

    //Create and display ciphertext
    caesar_it(abc, letters, key);
}

fn caesar_it(abc: [char; 26], plaintext: Vec<char>, key: usize){

    for i in 0..plaintext.len() {
        for j in 0..abc.len(){
            if abc[j]==plaintext[i] {
                print!("{}", abc[j+key]);
                
                break;
            }
        }
    }
    println!();
    
    //println!("The Ciphertext is: {}", cipherText); //Prepared for success
}
