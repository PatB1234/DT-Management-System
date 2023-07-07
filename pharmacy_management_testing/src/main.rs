use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader};
// fn read() -> std::io::Result<()> {
//     let file: File = File::open("./src/drugs.txt")?;

//     let reader = BufReader::new(file);

//     for line in reader.lines() {
//         println!("{}", line?);
//     }

//     Ok(())
// }

// fn write() {
//     let mut file = OpenOptions::new()
//         .write(true)
//         .open("./src/drugs.txt")
//         .unwrap();
//     file.write_all(b"PLACEHOLDER1\n");
//     file.write_all(b"PLACEHOLDER2\n");
// }
fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    login();
    main_menu();
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn login() {}
fn main_menu() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Main Menu");
    println!("What would you like to do?");
    println!("1. List All Drugs");
    println!("2. Add a new drug");
    println!("3. Add a existing drug");
    println!("4. Withdraw a Drug");
    println!("5. Exit");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice = choice.trim_end();
    if choice == "1" {
        list_drug();
    } else if choice == "2" {
        add_drug(false);
    } else if choice == "3" {
        add_drug(true);
    } else if choice == "4" {
        withdraw_drug();
    } else if choice == "5" {
        println!("Bye Bye!");
        std::process::exit(0x0100);
    }
}

fn list_drug() -> std::io::Result<()> {
    let file = File::open("./src/drugs.txt").unwrap();
    let reader = BufReader::new(file);
    println!("The current drugs in stock are:");
    for line in reader.lines() {
        let s = line.unwrap();
        let split = s.split("|");
        let split = split.collect::<Vec<&str>>();
        let a = split[0];
        let b = split[1];
        let c = split[2];
        let d = split[3];

        println!(
            "Name: {} Amount: {} Packaged Date: {} Expiry Date: {}",
            a, b, c, d
        )
    }
    println!("Hit enter to return to main menu: ");
    io::stdin().read_line(&mut String::new()).unwrap();
    main_menu();
    Ok(())
}

fn add_drug(existing: bool) {}

fn withdraw_drug() {}
