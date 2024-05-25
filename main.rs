use serde::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

#[derive(Deserialize)]
struct Application {
    name: String,
    path: String,
}

fn main() {
    let file_content = 
    // change json path here
    fs::read_to_string(r"C:\Users\Twenty-Sixth May\.cargo\mystuff\apps.json") 
    .expect("Unable to read file");

    let apps: Vec<Application> = serde_json::from_str(&file_content).expect("Unable to parse JSON");

    println!("Available applications:");
    for (index, app) in apps.iter().enumerate() {
        println!("{}. {} ({})", index + 1, app.name, app.path);
    }

    print!("Please select an application by number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let selected_index: usize = input.trim().parse().expect("Please enter a valid number");

    if selected_index > 0 && selected_index <= apps.len() {
        let selected_app = &apps[selected_index - 1];
        println!("You selected: {} ({})", selected_app.name, selected_app.path);

        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &selected_app.path])
                .spawn()
                .expect("Failed to launch application");
        } else {
            Command::new(&selected_app.path)
                .spawn()
                .expect("Failed to launch application");
        }
    } else {
        println!("Invalid selection");
    }
}
