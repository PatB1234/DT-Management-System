use dotenv::dotenv;
use postgres::{Client, NoTls};
use serde::Serialize;

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[derive(Serialize)]
struct Item {
    name: String,
    amount: String,
    id: String,
    location: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            login,
            list_items,
            add_item,
            add_existing_item,
            withdraw_item,
            remove_item,
            change_location_item,
            create_tables
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// fn main() {
//     dotenv().ok();
//     let db_url: String = std::env::var("DB_URL").expect("DB_URL must be set.");
//     let _ = create_tables(db_url);
//     print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
//     let res = login();
//     if res {
//         main_menu();
//     } else {
//         println!("Could not login in, details incorrect");
//         println!("Hit enter");
//         io::stdin().read_line(&mut String::new()).unwrap();
//         std::process::exit(0x0100);
//     }
// }
#[tauri::command]
fn create_tables(db_url: String) -> Result<(), String> {
    let mut client = Client::connect(db_url.as_str(), NoTls).map_err(|err| err.to_string())?;

    client
        .batch_execute(
            "CREATE TABLE IF NOT EXISTS items (name TEXT, amount TEXT, id TEXT, location TEXT)",
        )
        .map_err(|err| err.to_string())?;

    Ok(())
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

// fn take_input() -> String {
//     let mut u_input: String = String::new();
//     io::stdin()
//         .read_line(&mut u_input)
//         .expect("Something went wrong");
//     let len = u_input.trim_end().len();
//     u_input.truncate(len);
//     u_input
// }
#[tauri::command]

fn login(username: String, password: String) -> bool {
    if username == "DT" && password == "Management" {
        true
    } else {
        false
    }
}

// fn main_menu() {
//     dotenv().ok();
//     print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
//     println!("Main Menu");
//     println!("What would you like to do?");
//     println!("1. List All Items");
//     println!("2. Add a new item");
//     println!("3. Add an existing item");
//     println!("4. Withdraw an item");
//     println!("5. Remove an item");
//     println!("6. Change location of item");
//     println!("7. Exit");
//     let db_url: String = std::env::var("DB_URL").expect("DB_URL must be set.");

//     let mut choice = String::new();
//     io::stdin()
//         .read_line(&mut choice)
//         .expect("Failed to read line");
//     let choice = choice.trim_end();
//     if choice == "1" {
//         let _ = list_items(db_url);
//         println!("Hit enter to return to main menu: ");
//         io::stdin().read_line(&mut String::new()).unwrap();
//         main_menu();
//     } else if choice == "2" {
//         let _ = add_item(db_url);
//         println!("Hit enter to return to main menu: ");
//         io::stdin().read_line(&mut String::new()).unwrap();
//         main_menu();
//     } else if choice == "3" {
//         let _ = add_exisiting_item(db_url);
//         println!("Hit enter to return to main menu: ");
//         io::stdin().read_line(&mut String::new()).unwrap();
//         main_menu();
//     } else if choice == "4" {
//         let _ = withdraw_item(db_url);
//         println!("Hit enter to return to main menu: ");
//         io::stdin().read_line(&mut String::new()).unwrap();
//         main_menu();
//     } else if choice == "5" {
//         let _ = remove_item(db_url);
//         println!("Hit enter to return to main menu: ");
//         io::stdin().read_line(&mut String::new()).unwrap();
//         main_menu();
//     } else if choice == "6" {
//         let _ = change_location_item(db_url);
//         println!("Hit enter to return to main menu: ");
//         io::stdin().read_line(&mut String::new()).unwrap();
//         main_menu();
//     } else if choice == "7" {
//         println!("Bye Bye!");
//         std::process::exit(0x0100);
//     }
// }
#[tauri::command]
fn list_items() -> Result<Vec<Item>, String> {
    dotenv().ok();
    let db_url: String = std::env::var("DB_URL").expect("DB_URL must be set.");
    let mut client = Client::connect(db_url.as_str(), NoTls).map_err(|err| err.to_string())?;
    let mut items: Vec<Item> = Vec::new();

    for row in client
        .query("SELECT name, amount, id, location FROM items", &[])
        .map_err(|err| err.to_string())?
    {
        let item = Item {
            name: row.get(0),
            amount: row.get(1),
            id: row.get(2),
            location: row.get(3),
        };
        items.push(item);
    }

    Ok(items.into())
}
#[tauri::command]
fn add_item(name: String, amount: String, location: String, id: String) -> Result<(), String> {
    dotenv().ok();
    let db_url: String = std::env::var("DB_URL").expect("DB_URL must be set.");
    let mut client = Client::connect(db_url.as_str(), NoTls).map_err(|err| err.to_string())?;

    let data = Item {
        name: name,
        amount: amount,
        id: id,
        location: location,
    };

    client
        .execute(
            "INSERT INTO items (name, amount, id, location) VALUES ($1, $2, $3, $4)",
            &[&data.name, &data.amount, &data.id, &data.location],
        )
        .map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
fn add_existing_item(id: String, amount: String) -> Result<String, String> {
    dotenv().ok();
    let db_url: String = std::env::var("DB_URL").expect("DB_URL must be set.");
    let mut client = Client::connect(db_url.as_str(), NoTls).map_err(|err| err.to_string())?;

    let amount: i32 = amount.parse().unwrap();

    let curr_amount: i32;
    let row = client
        .query("SELECT * FROM items WHERE id=$1", &[&id])
        .map_err(|err| err.to_string())?;
    if row.is_empty() {
        let s = format!("Could not find item with the ID {}", id);
        return Ok(s.into());
    } else {
        let s: String = row.iter().next().unwrap().get(1);
        curr_amount = s.parse().unwrap();
        let i = curr_amount + amount;
        let i = i.to_string();
        client
            .execute("UPDATE items SET amount=$1 WHERE id=$2", &[&i, &id])
            .map_err(|err| err.to_string())?;
    }
    Ok(String::from("Successful").into())
}

#[tauri::command]
fn withdraw_item(id: String, amount: String, delete_y: String) -> Result<String, String> {
    dotenv().ok();
    let db_url: String = std::env::var("DB_URL").expect("DB_URL must be set.");
    let mut client = Client::connect(db_url.as_str(), NoTls).map_err(|err| err.to_string())?;
    let amount: i32 = amount.parse().unwrap();

    let curr_amount: i32;
    let row = client
        .query("SELECT * FROM items WHERE id=$1", &[&id])
        .map_err(|err| err.to_string())?;
    if !row.is_empty() {
        let s: String = row.iter().next().unwrap().get(1);
        curr_amount = s.parse().unwrap();
        if amount > curr_amount {
            return Ok(String::from(
                "There is not enough of the item to remove, nothing has been removed as a result",
            )
            .into());
        } else if amount == curr_amount {
            if delete_y == "Y" || delete_y == "y" {
                client
                    .execute("DELETE FROM items WHERE id=$1", &[&id])
                    .map_err(|err| err.to_string())?;
                return Ok(String::from("Deleted successfully").into());
            } else {
                return Ok(String::from("Deletion aborted").into());
            }
        } else {
            let i = curr_amount - amount;
            let i = i.to_string();
            client
                .execute("UPDATE items SET amount=$1 WHERE id=$2", &[&i, &id])
                .map_err(|err| err.to_string())?;
        }
    } else {
        let s = format!("Could not find item with id: {}", id);
        return Ok(s.into());
    }
    Ok(String::from("Error").into())
}

#[tauri::command]
fn remove_item(id: String) -> Result<String, String> {
    dotenv().ok();
    let db_url: String = std::env::var("DB_URL").expect("DB_URL must be set.");
    let mut client = Client::connect(db_url.as_str(), NoTls).map_err(|err| err.to_string())?;

    client
        .execute("DELETE FROM items WHERE id=$1", &[&id])
        .map_err(|err| err.to_string())?;

    Ok(String::from("Removed").into())
}

#[tauri::command]
fn change_location_item(id: String, location: String) -> Result<String, String> {
    dotenv().ok();
    let db_url: String = std::env::var("DB_URL").expect("DB_URL must be set.");
    let mut client = Client::connect(db_url.as_str(), NoTls).map_err(|err| err.to_string())?;

    let row = client
        .query("SELECT * FROM items WHERE id=$1", &[&id])
        .map_err(|err| err.to_string())?;
    if row.is_empty() {
        let s = format!("Could not find item with the ID {}", id);
        return Ok(s.into());
    } else {
        client
            .execute(
                "UPDATE items SET location=$1 WHERE id=$2",
                &[&location, &id],
            )
            .map_err(|err| err.to_string())?;
    }
    Ok("Successful".into())
}
