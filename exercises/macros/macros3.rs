// macros3.rs

mod macros {
    #[macro_export]  // 关键在这里！
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}