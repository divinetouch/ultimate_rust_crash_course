fn main() {
    struct RedFox {
        enemy: bool,
        life: u8,
    }

    impl RedFox {
        // associated functions
        // in other languages, can be considered as class functions/constructure
        fn new() -> Self {
            Self {
                enemy: true,
                life: 70,
            }
        }
    }

    let fox = RedFox::new();
    println!("{}, {}", fox.enemy, fox.life);

    // composition over inheritance
    trait Noisy {
        fn get_noise(&self) -> &str;
    }

    impl Noisy for RedFox {
        fn get_noise(&self) -> &str {
            "Meow?"
        }
    }

    fn print_noise<T: Noisy>(item: T) {
        println!("{}", item.get_noise());
    }

    print_noise(fox);

    // traits with default behavior
    trait Run {
        fn run(&self) {
            println!("I'm running!");
        }
    }

    struct Robot {}
    impl Run for Robot {}

    let robot = Robot {};
    robot.run();
}
