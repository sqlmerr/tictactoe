pub mod game {
    pub struct Game {
        pub name: String
    }
    impl Game {
        pub fn run(&self) {
            let mut map = [["[ ]"; 3]; 3];
            println!("{}", self.name);

            let mut err = "";

            loop {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                for row in 0..3 {
                    let mut line = String::new();
                    for col in 0..3 {
                        line.push_str(map[row][col])
                    }
                    println!("{}", line);
                }
                println!("{}", err);
                println!("Выберите ячейку, куда поставить крестик: ");
                let mut guess = String::new();
                match std::io::stdin().read_line(&mut guess) {
                    Ok(_) => {
                        let num: usize = guess.trim().parse().unwrap();
                        if num > 9 {
                            err = "Слишком большое число!";
                            continue
                        }
                        let row = (num - 1) / 3;
                        let cell_index = (num - 1) % 3;

                        if map[row][cell_index] == "[ ]" {
                            map[row][cell_index] = "[x]"
                        } else {
                            err = "Эта ячейка уже занята";
                            continue
                        }
                    }
                    Err(e) => println!("Ошибка - {}", e)
                }
                err = "";


            }
        }

    }
}