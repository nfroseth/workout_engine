struct Exercise {
    movement: String,
    volume: i32,
    reps: i32,
}

struct WeightLifting {
    exercises: Vec<Exercise>
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
    duration: i32
}
enum Card {
    Workout(Workout),
    Rest(Rest),
}

struct Stack {
    cards: Vec<Card>,
}

struct User {
    stack: Stack
}

fn main() {
    // Sample data
    let pushups = Exercise {
        movement: String::from("Push-ups"),
        volume: 3, // Sets
        reps: 15,
    };

    let squats = Exercise {
        movement: String::from("Squats"),
        volume: 4, // Sets
        reps: 12,
    };

    let weight_lifting_workout = WeightLifting {
        exercises: vec![pushups, squats],
    };

    let running_workout = Running {
        distance: 5, // kilometers
        duration: 30, // minutes
    };

    let rest_duration = 10; // minutes

    // Create a workout stack
    let my_stack = Stack {
        cards: vec![
            Card::Workout(Workout::WeightLifting(weight_lifting_workout)),
            Card::Rest(Rest { duration: rest_duration }),
            Card::Workout(Workout::Running(running_workout)),
        ],
    };

    // Create a user
    let me = User { stack: my_stack };

    println!("Booting up the WOE (Workout Organizer Engine)...");

    println!("Example User Stack: {}", me);

    // Access and process user's workout stack here!
}