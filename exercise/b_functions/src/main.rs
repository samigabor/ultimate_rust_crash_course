use b_functions::{area_of, volume};

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;
    
    {
        let area = area_of(width, height);
        println!("Area is {}", area);
    }
    
    println!("Volume is {}", volume(width, height, depth));
}
