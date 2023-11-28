const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (missles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missles...", ready, missles);
    println!("{} missles left", missles - ready);
    
    let _unused_var = 3; // "_" prefix & "snake case" to avoid unused variable warning
    // READY_AMOUNT = 1; // error: cannot assign twice to const variable
}
