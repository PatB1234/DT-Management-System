use dotenv::dotenv;
use postgres::{Client, Error, NoTls};
use std::io;

struct Drugs {
    name: String,
    amount: String,
    pckg_date: String,
    expiry_date: String,
    _id: String,
}

struct Perscription {
    name: String,
    dr_name: String,
    refill_num: String,
    medicines: String,
    uid: String,
}
fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    //let _ = create_tables();
    login();
    main_menu();
}

fn create_tables() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgres://jnmuchkv:CSMms0y5t6sD9ESRcqFrFtfdzzOz4BMG@stampy.db.elephantsql.com/jnmuchkv",
        NoTls,
    )?;

    client.batch_execute("CREATE TABLE IF NOT EXISTS drugs (name TEXT, amount TEXT, package_date TEXT, expiry_date TEXT, id TEXT)")?;
    client.batch_execute("CREATE TABLE IF NOT EXISTS perscriptions (name TEXT, dr_name TEXT, refill_num TEXT, medicines TEXT, uid TEXT)")?;

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
    println!("5. Remove a Drug");
    println!("6. List all perscriptions (This may take a few minutes)");
    println!("7. Exit");
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
        let _ = withdraw_drug(db_url);
    } else if choice == "5" {
        let _ = delete_drug(db_url);
    } else if choice == "6" {
        let _ = list_perscriptions(db_url);
    } else if choice == "7" {
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
fn delete_drug(db_url: String) -> Result<(), Error> {
    let mut client = Client::connect(db_url.as_str(), NoTls)?;
    println!("Enter the id of your drug: ");
    let id = take_input();
    client.execute("DELETE FROM drugs WHERE id=$1", &[&id])?;
    println!("Removed!");
    println!("Hit enter to return to main menu: ");
    io::stdin().read_line(&mut String::new()).unwrap();
    main_menu();
    Ok(())
}
fn list_perscriptions(db_url: String) -> Result<(), Error> {
    let mut client = Client::connect(db_url.as_str(), NoTls)?;
    let db_url_clone = db_url.clone();
    for row in client.query(
        "SELECT name, dr_name, refill_num, medicines, uid FROM perscriptions",
        &[],
    )? {
        let data = Perscription {
            name: row.get(0),
            dr_name: row.get(1),
            refill_num: row.get(2),
            medicines: row.get(3),
            uid: row.get(4),
        };
        let text = data
            .medicines
            .split("|")
            .map(|medicine_id| get_medicines_by_id(db_url_clone.to_string(), medicine_id))
            .collect::<Result<Vec<_>, _>>()?
            .join(", ");
        println!(
            "Name: {}, Dr. Name: {}, Refill Amount: {}, Medicines: {} User ID: {}",
            data.name, data.dr_name, data.refill_num, text, data.uid
        );
    }
    println!("Hit enter to return to main menu: ");
    io::stdin().read_line(&mut String::new()).unwrap();
    main_menu();
    Ok(())
}

fn get_medicines_by_id(db_url: String, _id: &str) -> Result<String, Error> {
    let mut client = Client::connect(db_url.as_str(), NoTls)?;
    let name = client.query("SELECT name FROM drugs WHERE id=$1", &[&_id])?;
    let s: String = name.iter().next().unwrap().get(0);
    Ok(s)
}
