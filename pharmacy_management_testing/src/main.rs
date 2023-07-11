use dotenv::dotenv;
use postgres::{Client, Error, NoTls};
use std::io;

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
// fn main() -> Result<(), Error> {
//     let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;

//     client.batch_execute(
//         "
//         CREATE TABLE IF NOT EXISTS author (
//             id              SERIAL PRIMARY KEY,
//             name            VARCHAR NOT NULL,
//             country         VARCHAR NOT NULL
//             )
//     ",
//     )?;

//     client.batch_execute(
//         "
//         CREATE TABLE IF NOT EXISTS book  (
//             id              SERIAL PRIMARY KEY,
//             title           VARCHAR NOT NULL,
//             author_id       INTEGER NOT NULL REFERENCES author
//             )
//     ",
//     )?;

//     Ok(())
// }
struct Drugs {
    name: String,
    amount: String,
    pckg_date: String,
    expiry_date: String,
    _id: String,
}
fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let _ = create_tables();
    login();
    main_menu();
}

fn create_tables() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgres://jnmuchkv:CSMms0y5t6sD9ESRcqFrFtfdzzOz4BMG@stampy.db.elephantsql.com/jnmuchkv",
        NoTls,
    )?;

    client.batch_execute("CREATE TABLE IF NOT EXISTS drugs (name TEXT, amount TEXT, package_date TEXT, expiry_date TEXT, id TEXT)")?;

    Ok(())
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
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
    dotenv().ok();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Main Menu");
    println!("What would you like to do?");
    println!("1. List All Drugs");
    println!("2. Add a new drug");
    println!("3. Add a existing drug");
    println!("4. Withdraw a Drug");
    println!("5. Exit");
    let db_url: String = std::env::var("DB_URL").expect("DB_URL must be set.");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice = choice.trim_end();
    if choice == "1" {
        let _ = list_drug(db_url);
    } else if choice == "2" {
        let _ = add_drug(db_url);
    } else if choice == "3" {
        let _ = add_exisiting_drug(db_url);
    } else if choice == "4" {
        withdraw_drug(db_url);
    } else if choice == "5" {
        println!("Bye Bye!");
        std::process::exit(0x0100);
    }
}

fn list_drug(db_url: String) -> Result<(), Error> {
    let mut client = Client::connect(db_url.as_str(), NoTls)?;

    for row in client.query(
        "SELECT name, amount, package_date, expiry_date, id FROM drugs",
        &[],
    )? {
        let data = Drugs {
            name: row.get(0),
            amount: row.get(1),
            pckg_date: row.get(2),
            expiry_date: row.get(3),
            _id: row.get(4),
        };
        println!(
            "Name: {} Amount: {} Packaged Date: {} Expiry Date: {} ID: {}",
            data.name, data.amount, data.pckg_date, data.expiry_date, data._id
        )
    }
    println!("Hit enter to return to main menu: ");
    io::stdin().read_line(&mut String::new()).unwrap();
    main_menu();
    Ok(())
}

fn add_drug(db_url: String) -> Result<(), Error> {
    let mut client = Client::connect(db_url.as_str(), NoTls)?;

    println!("Enter the name of your drug: ");
    let names = take_input();

    println!("Enter the amount of your drug: ");
    let amounts = take_input();

    println!("Enter the packaged date: ");
    let packaged = take_input();

    println!("Enter the expiry date: ");
    let expiry = take_input();

    println!("Enter the id: ");
    let _ids = take_input();
    let data = Drugs {
        name: names,
        amount: amounts,
        pckg_date: packaged,
        expiry_date: expiry,
        _id: _ids,
    };

    client.execute(
            "INSERT INTO drugs (name, amount, package_date, expiry_date, id) VALUES ($1, $2, $3, $4, $5)",
            &[&data.name, &data.amount, &data.pckg_date, &data.expiry_date, &data._id],
        )?;

    println!("Hit enter to return to main menu: ");
    io::stdin().read_line(&mut String::new()).unwrap();
    main_menu();

    Ok(())
}

fn add_exisiting_drug(db_url: String) -> Result<(), Error> {
    let mut client = Client::connect(db_url.as_str(), NoTls)?;
    println!("Enter the id of your drug: ");
    let id = take_input();

    println!("Enter the amount of your drug: ");
    let amount: i32 = take_input().parse().unwrap();

    let mut curr_amount: i32;
    for row in client.query("SELECT * FROM drugs WHERE id=$1", &[&id])? {
        let s: String = row.get(1);
        curr_amount = s.parse().unwrap();
        let i = curr_amount + amount;
        let i = i.to_string();
        client.execute("UPDATE drugs SET amount=$1 WHERE id=$2", &[&i, &id])?;
    }

    println!("Hit enter to return to main menu: ");
    io::stdin().read_line(&mut String::new()).unwrap();
    main_menu();
    Ok(())
}
fn withdraw_drug(db_url: String) -> Result<(), Error> {
    let mut client = Client::connect(db_url.as_str(), NoTls)?;
    println!("Enter the id of your drug: ");
    let id = take_input();

    println!("Enter the amount of your drug you would like to decrease: ");
    let amount: i32 = take_input().parse().unwrap();

    let mut curr_amount: i32;
    for row in client.query("SELECT * FROM drugs WHERE id=$1", &[&id])? {
        let s: String = row.get(1);
        curr_amount = s.parse().unwrap();
        if amount >= curr_amount {
            println!("The amount fo the drug has resulted to 0 or less, we are by default setting it to 0");
            client.execute("UPDATE drugs SET amount=0 WHERE id=$1", &[&id])?;
            println!("Would you like to delete the drug from the system? Y or N");
            let s = take_input();
            if s == "Y" || s == "y" {
                client.execute("DELETE FROM drugs WHERE id=$1", &[&id])?;
                println!("Deleted successfully");
            } else {
                println!("Deletion aborted");
            }
        } else {
            let i = curr_amount - amount;
            let i = i.to_string();
            client.execute("UPDATE drugs SET amount=$1 WHERE id=$2", &[&i, &id])?;
        }
    }

    println!("Hit enter to return to main menu: ");
    io::stdin().read_line(&mut String::new()).unwrap();
    main_menu();
    Ok(())
}
