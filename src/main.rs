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

// Custom trait for displaying data
trait Displayable {
    fn display(&self) -> String;
}

// Implement Displayable for Exercise
impl Displayable for Exercise {
    fn display(&self) -> String {
        format!("{} ({} sets x {} reps)", self.movement, self.volume, self.reps)
    }
}

// Implement Displayable for WeightLifting
impl Displayable for WeightLifting {
    fn display(&self) -> String {
        let exercises_str: Vec<String> = self.exercises.iter().map(|e| e.display()).collect();
        format!("Weight Lifting: [{}]", exercises_str.join(", "))
    }
}

// Implement Displayable for Running
impl Displayable for Running {
    fn display(&self) -> String {
        format!("Running: {} km in {} minutes", self.distance, self.duration)
    }
}

// Implement Displayable for Workout
impl Displayable for Workout {
    fn display(&self) -> String {
        match self {
            Workout::WeightLifting(weightlifting) => weightlifting.display(),
            Workout::Running(running) => running.display(),
        }
    }
}

// Implement Displayable for Card
impl Displayable for Card {
    fn display(&self) -> String {
        match self {
            Card::Workout(workout) => workout.display(),
            Card::Rest(rest) => format!("Rest: {} minutes", rest.duration),
        }
    }
}

// Implement Displayable for Stack
impl Displayable for Stack {
    fn display(&self) -> String {
        let cards_str: Vec<String> = self.cards.iter().map(|c| c.display()).collect();
        format!("Workout Stack: [{}]", cards_str.join(", "))
    }
}

// Implement Display for User
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.stack.display())
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
