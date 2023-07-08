use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, Write};
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

fn get_num_lines() -> usize {
    let file = BufReader::new(File::open("./src/drugs.txt").expect("Unable to open file"));
    let mut cnt: usize = 0;

    for _ in file.lines() {
        cnt = cnt + 1;
    }

    cnt
}

fn take_input() -> String {
    let mut u_input: String = String::new();
    io::stdin()
        .read_line(&mut u_input)
        .expect("Something went wrong");
    let len = u_input.trim_end().len();
    u_input.truncate(len);
    u_input
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
        let _ = list_drug();
    } else if choice == "2" {
        add_drug();
    } else if choice == "3" {
        let _ = add_exisiting_drug();
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

fn add_drug() {
    println!("Enter the name of your drug: ");
    let name = take_input();

    println!("Enter the amount of your drug: ");
    let amount = take_input();

    println!("Enter the packaged date: ");
    let packaged = take_input();

    println!("Enter the expiry date: ");
    let expiry = take_input();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./src/drugs.txt")
        .unwrap();
    let s: String = format!("{name}|{amount}|{packaged}|{expiry}\n");
    let _ = file.write_all(s.as_bytes());

    println!("Hit enter to return to main menu: ");
    io::stdin().read_line(&mut String::new()).unwrap();
    main_menu();
}

fn add_exisiting_drug() -> std::io::Result<()> {
    let file: File = File::open("./src/drugs.txt")?;
    let reader = BufReader::new(file);

    println!("Enter the name of your drug: ");
    let name = take_input();

    println!("Enter the amount of your drug: ");
    let amount: i32 = take_input().parse().unwrap();

    let mut found = false;
    let mut arr = vec![String::new(); get_num_lines()];
    let mut iter = 0;
    for line in reader.lines() {
        let s = line.unwrap();
        let split = s.split("|");
        let split = split.collect::<Vec<&str>>();
        let a = split[0];
        let b = split[1];
        let c = split[2];
        let d = split[3];

        if a == name {
            found = true;
            let total_amount: i32 = b.parse().unwrap();
            let total_amount: i32 = total_amount + amount;
            let names = a;
            let packaged = c;
            let expiry = d;
            arr[iter] = format!("{}|{}|{}|{}", names, total_amount, packaged, expiry);
        } else {
            arr[iter] = s;
        }
        iter += 1;
    }

    if found == false {
        println!("ERROR: Could not find specified drug");
    } else {
        fs::remove_file("./src/drugs.txt").expect("Error encountered while deleting file");
        File::create("./src/drugs.txt").expect("Error encountered while creating file!");
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("./src/drugs.txt")
            .unwrap();

        for i in arr.iter_mut() {
            let n = format!("{}\n", i);
            let _ = file.write_all(n.as_bytes());
        }
    }
    println!("{:?}", arr);
    println!("Hit enter to return to main menu: ");
    io::stdin().read_line(&mut String::new()).unwrap();
    main_menu();
    Ok(())
}
fn withdraw_drug() {}
