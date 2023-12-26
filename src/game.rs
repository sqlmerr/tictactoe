// use std::io;

pub mod game {
    pub struct Game {
        pub name: String
    }
    impl Game {
        pub fn run(self) {
            const WIDTH: usize = 3;
            const HEIGHT: usize = 3;

            let map = [["[]"; WIDTH]; HEIGHT];
            println!("{}", self.name);

            // loop {
            //     for row in 0..WIDTH {
            //         let mut line = String::new();
            //         for col in 0..HEIGHT {
            //             line.push_str("[ ]")
            //         }
            //     }
            // }
            println!("{:?}", map);
        }
    }
}