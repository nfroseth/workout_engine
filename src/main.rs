use std::fmt;

struct Exercise {
    movement: String,
    volume: i32,
    reps: i32,
}

struct WeightLifting {
    exercises: Vec<Exercise>,
}

struct Running {
    distance: i32,
    duration: i32,
}

enum Workout {
    WeightLifting(WeightLifting),
    Running(Running),
}

struct Rest {
    duration: i32,
}

enum Card {
    Workout(Workout),
    Rest(Rest),
}

struct Stack {
    cards: Vec<Card>,
}

struct User {
    stack: Stack,
}

// Implement Display for Exercise
impl fmt::Display for Exercise {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({} sets x {} reps)", self.movement, self.volume, self.reps)
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
        write!(f, "Running: {} km in {} minutes", self.distance, self.duration)
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

fn main() {
    // Sample data (same as before)

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

    println!("Booting up the WOE (Workout Organizer Engine)...");
    println!("{}", me);
}
