use std::io;


fn main() {
    let password: [&str; 6] = ["I", "Am", "The", "Hacker", "To", "You"];

    let mut username_input = String::new();
    let mut password_input = String::new();

    println!("Input the username: ");
    io::stdin()
        .read_line(&mut username_input)
        .expect("Input couldn't be read.");

    println!("Input the password: ");
    io::stdin()
        .read_line(&mut password_input)
        .expect("Input couldn't be read.");

    if username_input.trim() == "Mihir" {
        for number in 0..6 {
            if password[number] == password_input.split(" ").collect::<Vec<&str>>()[number].trim() {
                if number == 5 {
                    println!("Access Granted!");
                    break;
                }
                continue;
            }
            println!("Wrong password!");
            break;
        }
    }
    else {
        println!("Wrong username!");
    }
}
