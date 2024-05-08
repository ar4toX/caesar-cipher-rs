use std::io;

fn main() {
    //Values and things to use
    let mut plaintext = String::new();
    let mut key= String::new();
    let mut option= String::new();

    //Thing
    println!("Hello what are you in here for? ");
    println!("1. Cypher");
    println!("2. Decypher (Bruteforce)");
    
    io::stdin().read_line(&mut option)
        .expect("Input failed :(");
    let option = option.trim().parse::<i32>()
        .expect("Invalid entry");

    //Gather data from user
    println!("Enter the message to cipher: ");
    io::stdin().read_line(&mut plaintext)
        .expect("Input failed :(");

    match option{
        1=>{
            println!("Enter the desired key: ");
            io::stdin().read_line(&mut key)
                .expect("Input failed :(");
            let key = key.trim().parse::<usize>()
                .expect("Invalid entry");
                caesar(&plaintext, key);}

        2=>{for i in 1..26 {
            println!("ROT-{}: {}", i, (caesar(&plaintext,i)));
        }}
        _=>println!("Invalid Option"),
    }
}

fn caesar(plaintext: &String, key: usize) -> String{
    let abc=
    ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
     'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
     's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

     //Make vector list for input characters
    let letters: Vec<char> = plaintext.chars().collect();
    
    let mut cipher_text_vec: Vec<char>=vec![];
    let mut n: usize;

    for i in 0..letters.len() {
        for j in 0..abc.len(){
            if abc[j]==letters[i] {
                n=j+key;
                if n>25{
                    n-=26;
                }
                cipher_text_vec.push(abc[n]);
                break;
            }
            if letters[i]==' '{
                cipher_text_vec.push(' ');
                break;
            }
        }
    }
    println!();

    //Convert vector to string
    let cipher_text: String = cipher_text_vec.iter().collect();

    return cipher_text;
}
