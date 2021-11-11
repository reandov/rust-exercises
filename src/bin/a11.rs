// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    id: i32,
    quantity: i32,
}

fn print_item_id(item: &GroceryItem) {
    println!("{:?}", item.id);
}

fn print_item_quantity(item: &GroceryItem) {
    println!("{:?}", item.quantity);
}

fn main() {
    let candy = GroceryItem {
        id: 32,
        quantity: 3,
    };

    print_item_id(&candy);
    print_item_quantity(&candy);
}
