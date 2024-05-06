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
    caesar(abc, letters, key);
}

fn caesar(abc: [char; 26], plaintext: Vec<char>, key: usize){
    let mut cipher_text_vec: Vec<char>=vec![];
    let mut n: usize;

    for i in 0..plaintext.len() {
        for j in 0..abc.len(){
            if abc[j]==plaintext[i] {
                n=j+key;
                if n>25{
                    n-=26;
                }
                cipher_text_vec.push(abc[n]);
                break;
            }
            if plaintext[i]==' '{
                cipher_text_vec.push(' ');
                break;
            }
        }
    }
    println!();

    //Convert vector to string
    let cipher_text: String = cipher_text_vec.iter().collect();
    
    println!("The Ciphertext is: {}", cipher_text);
}
