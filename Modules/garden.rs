// modified from rustlings example
mod delicious_snacks {

    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const BANANA: &'static str = "Banana";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CELERY: &'static str = "Celery";
    }
}

fn main() {
    println!(
        "My garden has {} and {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
        delicious_snacks::veggies::CELERY
    );
}
