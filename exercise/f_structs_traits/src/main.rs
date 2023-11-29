// 1. Define a trait named `Bite`
// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.
trait Bite {
    fn bite(self: &mut Self);
}


// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.
#[derive(Debug)] // include this line right before your struct definition
struct Grapes {
    amount_left: i32,
}

fn main() {
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    // Generic `bunny_nibbles` function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    fn bunny_nibbles<T: Bite>(food: &mut T) {
        println!("Bunny is nibbling...");
        food.bite();
        println!("Bunny is nibbling some more...");
        food.bite();
    }
    
    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

// 3. Implement Bite for Grapes. When you bite a Grapes, subtract 1 from how many grapes are left.
impl Bite for Grapes {
    fn bite(&mut self) {
        // Eat 1 grape of the remaining grapes. It may take awhile to eat it all...
        self.amount_left -= 1;
    }
}