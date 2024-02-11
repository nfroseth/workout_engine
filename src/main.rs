use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::{env, fmt};

#[derive(Serialize, Deserialize)]
struct Exercise {
    movement: String,
    volume: i32,
    reps: i32,
}

#[derive(Serialize, Deserialize)]
struct WeightLifting {
    exercises: Vec<Exercise>,
}

#[derive(Serialize, Deserialize)]
struct Running {
    distance: i32,
    duration: i32,
}

#[derive(Serialize, Deserialize)]
enum Workout {
    WeightLifting(WeightLifting),
    Running(Running),
}

#[derive(Serialize, Deserialize)]
struct Rest {
    duration: i32,
}

#[derive(Serialize, Deserialize)]
enum Card {
    Workout(Workout),
    Rest(Rest),
}

#[derive(Serialize, Deserialize)]
struct Stack {
    cards: Vec<Card>,
}

#[derive(Serialize, Deserialize)]
struct User {
    stack: Stack,
}

// Implement Display for Exercise
impl fmt::Display for Exercise {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({} sets x {} reps)",
            self.movement, self.volume, self.reps
        )
    }
}

// Implement Display for WeightLifting
impl fmt::Display for WeightLifting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let exercises_str: Vec<String> = self.exercises.iter().map(|e| format!("{}", e)).collect();
        write!(f, "Weight Lifting: [{}]", exercises_str.join(", "))
    }
}

// Implement Display for Running
impl fmt::Display for Running {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Running: {} km in {} minutes",
            self.distance, self.duration
        )
    }
}

// Implement Display for Workout
impl fmt::Display for Workout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Workout::WeightLifting(weightlifting) => write!(f, "{}", weightlifting),
            Workout::Running(running) => write!(f, "{}", running),
        }
    }
}

// Implement Display for Card
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Card::Workout(workout) => write!(f, "{}", workout),
            Card::Rest(rest) => write!(f, "Rest: {} minutes", rest.duration),
        }
    }
}

// Implement Display for Stack
impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cards_str: Vec<String> = self.cards.iter().map(|c| format!("{}", c)).collect();
        write!(f, "Workout Stack: [{}]", cards_str.join(", "))
    }
}

// Implement Display for User
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User Stack: {}", self.stack)
    }
}

async fn import_data_from_google_sheets() -> Result<User, Box<dyn Error>> {
    // Create a client.
    let client = Client::new();

    // Define the ID of your Google Sheet and the range of cells you want to access.
    let sheet_id = env::var("WORKOUT_ENGINE_SHEET_ID").expect("SHEET_ID is not set in env var.`");
    let range = "Sheet1!A1:D5";

    // Construct the URL for the Google Sheets API.
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}",
        sheet_id, range
    );

    // Send a GET request to the Google Sheets API.
    let res = client.get(&url).send().await?;

    // Parse the response body as JSON.
    let json: Value = res.json().await?;

    // Extract the data from the JSON response.
    let values = json["values"].as_array().ok_or("missing values")?;

    // Convert the data into a User struct.
    // This is a placeholder - you'll need to replace this with actual code to convert the data.
    let user: User = todo!();

    Ok(user)
}

// Function to write data to disk
fn write_data_to_disk(user: &User) -> std::io::Result<()> {
    let serialized = serde_json::to_string(user).unwrap();

    let mut file = File::create("workout_data.json")?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Booting up the WOE (Workout Organizer Engine)...");

    // Create a workout stack
    let my_stack = Stack {
        cards: vec![
            Card::Workout(Workout::WeightLifting(WeightLifting {
                exercises: vec![Exercise {
                    movement: String::from("Push-ups"),
                    volume: 3,
                    reps: 15,
                }],
            })),
            Card::Rest(Rest { duration: 10 }),
            Card::Workout(Workout::Running(Running {
                distance: 5,
                duration: 30,
            })),
        ],
    };

    // Create a user
    let me = User { stack: my_stack };
    println!("{}", me);

    let user = import_data_from_google_sheets().await?;

    write_data_to_disk(&user)?;

    Ok(())
}
