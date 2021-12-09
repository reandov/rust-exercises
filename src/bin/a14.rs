// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    color: String,
    age: i32,
}

fn print(name: &str) {
    println!("{:?}", name);
}

fn main() {
    let people = vec![
        Person {
            name: String::from("Evandro"),
            color: String::from("black"),
            age: 23,
        },
        Person {
            name: String::from("Jordão"),
            color: String::from("light pink"),
            age: 21,
        },
        Person {
            name: String::from("Leonardo"),
            color: String::from("grey"),
            age: 24,
        }
    ];

    for person in people {
        if person.age > 20 {
            print(&person.name);
        }
    }
}
