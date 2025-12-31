// macros1.rs

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
        println!("Look, I can print multiple lines!");
    };
}

fn main() {
    my_macro();
}