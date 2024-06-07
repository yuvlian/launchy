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
    loop {
        clear_terminal();
        let file_content = fs::read_to_string("apps.json").unwrap();
        let apps: Vec<Application> = serde_json::from_str(&file_content).unwrap();
        println!("");
        println!("Input E to exit, input R to refresh app list after editing JSON, input number to launch app.");
        println!("Available applications:");
        for (index, app) in apps.iter().enumerate() {
            println!("{}. {} ({})", index + 1, app.name, app.path);
        }
        println!("");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("R") {
            continue;
        } else if input.eq_ignore_ascii_case("E") {
            println!("Exiting...");
            break;
        }

        let selected_index: usize = match input.parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        if selected_index > 0 && selected_index <= apps.len() {
            let selected_app = &apps[selected_index - 1];

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
            continue;
        }
    }
}

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}