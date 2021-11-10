// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Strawberry,
    Orange,
    Lemon,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Strawberry => println!("Strawberry"),
        Flavor::Orange => println!("Orange"),
        Flavor::Lemon => println!("Lemon"),
    }

    println!("Oz: {:?}", drink.fluid_oz);
}

fn main() {
    let drink_one = Drink {
        flavor: Flavor::Orange,
        fluid_oz: 5.25,
    };

    print_drink(drink_one);
}
