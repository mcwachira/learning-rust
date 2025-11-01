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


struct Groceries {
    id:i32,
    quantity:i32
}

fn display_quantity(grocery:&Groceries){
    println!("quantity ={:?}" , grocery.quantity);

}
fn display_id(grocery:&Groceries){
    println!("quantity ={:?}" , grocery.id);

}
fn main() {

    let grocery = Groceries {
        id:2,
        quantity:5,
    };

    display_quantity(&grocery);
    display_id(&grocery);
}


