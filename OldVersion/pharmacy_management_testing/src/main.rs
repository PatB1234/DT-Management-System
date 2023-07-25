use dotenv::dotenv;
use postgres::{Client, Error, NoTls};
use std::io;

struct Item {
    name: String,
    amount: String,
    id: String,
    location: String,
}

fn main() {
    dotenv().ok();
    let db_url: String = std::env::var("DB_URL").expect("DB_URL must be set.");
    let _ = create_tables(db_url);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let res = login();
    if res {
        main_menu();
    } else {
        println!("Could not login in, details incorrect");
        println!("Hit enter");
        io::stdin().read_line(&mut String::new()).unwrap();
        std::process::exit(0x0100);
    }
}

fn create_tables(db_url: String) -> Result<(), Error> {
    let mut client = Client::connect(db_url.as_str(), NoTls)?;

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS items (name TEXT, amount TEXT, id TEXT, location TEXT)",
    )?;

    Ok(())
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn take_input() -> String {
    let mut u_input: String = String::new();
    io::stdin()
        .read_line(&mut u_input)
        .expect("Something went wrong");
    let len = u_input.trim_end().len();
    u_input.truncate(len);
    u_input
}

fn login() -> bool {
    println!("Username: ");
    let username = take_input();

    println!("Password: ");
    let password = take_input();

    if username == "DT" && password == "Management" {
        true
    } else {
        false
    }
}

fn main_menu() {
    dotenv().ok();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Main Menu");
    println!("What would you like to do?");
    println!("1. List All Items");
    println!("2. Add a new item");
    println!("3. Add an existing item");
    println!("4. Withdraw an item");
    println!("5. Remove an item");
    println!("6. Change location of item");
    println!("7. Exit");
    let db_url: String = std::env::var("DB_URL").expect("DB_URL must be set.");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice = choice.trim_end();
    if choice == "1" {
        let _ = list_items(db_url);
        println!("Hit enter to return to main menu: ");
        io::stdin().read_line(&mut String::new()).unwrap();
        main_menu();
    } else if choice == "2" {
        let _ = add_item(db_url);
        println!("Hit enter to return to main menu: ");
        io::stdin().read_line(&mut String::new()).unwrap();
        main_menu();
    } else if choice == "3" {
        let _ = add_exisiting_item(db_url);
        println!("Hit enter to return to main menu: ");
        io::stdin().read_line(&mut String::new()).unwrap();
        main_menu();
    } else if choice == "4" {
        let _ = withdraw_item(db_url);
        println!("Hit enter to return to main menu: ");
        io::stdin().read_line(&mut String::new()).unwrap();
        main_menu();
    } else if choice == "5" {
        let _ = remove_item(db_url);
        println!("Hit enter to return to main menu: ");
        io::stdin().read_line(&mut String::new()).unwrap();
        main_menu();
    } else if choice == "6" {
        let _ = change_location_item(db_url);
        println!("Hit enter to return to main menu: ");
        io::stdin().read_line(&mut String::new()).unwrap();
        main_menu();
    } else if choice == "7" {
        println!("Bye Bye!");
        std::process::exit(0x0100);
    }
}

fn list_items(db_url: String) -> Result<(), Error> {
    let mut client = Client::connect(db_url.as_str(), NoTls)?;

    for row in client.query("SELECT name, amount, id, location FROM items", &[])? {
        let data = Item {
            name: row.get(0),
            amount: row.get(1),
            id: row.get(2),
            location: row.get(3),
        };
        println!(
            "Name: {} Amount: {} ID: {} Location: {}",
            data.name, data.amount, data.id, data.location
        )
    }
    Ok(())
}

fn add_item(db_url: String) -> Result<(), Error> {
    let mut client = Client::connect(db_url.as_str(), NoTls)?;

    println!("Enter the name of your item: ");
    let name = take_input();

    println!("Enter the amount of your item: ");
    let amount = take_input();

    println!("Enter the location: ");
    let location = take_input();

    println!("Enter the id: ");
    let id = take_input();

    let data = Item {
        name: name,
        amount: amount,
        id: id,
        location: location,
    };

    client.execute(
        "INSERT INTO items (name, amount, id, location) VALUES ($1, $2, $3, $4)",
        &[&data.name, &data.amount, &data.id, &data.location],
    )?;

    Ok(())
}

fn add_exisiting_item(db_url: String) -> Result<(), Error> {
    let mut client = Client::connect(db_url.as_str(), NoTls)?;
    println!("Enter the id of your item: ");
    let id = take_input();

    println!("Enter the amount of your item: ");
    let amount: i32 = take_input().parse().unwrap();

    let curr_amount: i32;
    let row = client.query("SELECT * FROM items WHERE id=$1", &[&id])?;
    if row.is_empty() {
        println!("Could not find item with the ID {}", id);
    } else {
        let s: String = row.iter().next().unwrap().get(1);
        curr_amount = s.parse().unwrap();
        let i = curr_amount + amount;
        let i = i.to_string();
        client.execute("UPDATE items SET amount=$1 WHERE id=$2", &[&i, &id])?;
    }
    Ok(())
}

fn withdraw_item(db_url: String) -> Result<(), Error> {
    let mut client = Client::connect(db_url.as_str(), NoTls)?;
    println!("Enter the id of your item: ");
    let id = take_input();

    println!("Enter the amount of your item you would like to decrease: ");
    let amount: i32 = take_input().parse().unwrap();

    let curr_amount: i32;
    let row = client.query("SELECT * FROM items WHERE id=$1", &[&id])?;
    if !row.is_empty() {
        let s: String = row.iter().next().unwrap().get(1);
        curr_amount = s.parse().unwrap();
        if amount > curr_amount {
            println!(
                "There is not enough of the item to remove, nothing has been removed as a result"
            );
        } else if amount == curr_amount {
            println!("The amount of the item has resulted to 0, we are by default setting it to 0");
            client.execute("UPDATE items SET amount=0 WHERE id=$1", &[&id])?;
            println!("Would you like to delete the item from the system? Y or N");
            let s = take_input();
            if s == "Y" || s == "y" {
                client.execute("DELETE FROM items WHERE id=$1", &[&id])?;
                println!("Deleted successfully");
            } else {
                println!("Deletion aborted");
            }
        } else {
            let i = curr_amount - amount;
            let i = i.to_string();
            client.execute("UPDATE items SET amount=$1 WHERE id=$2", &[&i, &id])?;
        }
    } else {
        println!("Could not find item with id: {}", id);
    }
    Ok(())
}

fn remove_item(db_url: String) -> Result<(), Error> {
    let mut client = Client::connect(db_url.as_str(), NoTls)?;
    println!("Enter the id of your item: ");
    let id = take_input();
    println!("Note, this will delete every single item with the specified ID: Say Y to continue");

    client.execute("DELETE FROM items WHERE id=$1", &[&id])?;
    println!("Removed!");

    Ok(())
}

fn change_location_item(db_url: String) -> Result<(), Error> {
    let mut client = Client::connect(db_url.as_str(), NoTls)?;
    println!("Enter the id of your item: ");
    let id = take_input();

    println!("Enter the new location of your item: ");
    let location = take_input();

    let row = client.query("SELECT * FROM items WHERE id=$1", &[&id])?;
    if row.is_empty() {
        println!("Could not find item with the ID {}", id);
    } else {
        client.execute(
            "UPDATE items SET location=$1 WHERE id=$2",
            &[&location, &id],
        )?;
    }
    Ok(())
}
