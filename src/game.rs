pub mod game {
    use rand::Rng;

    pub struct Game {
        pub name: String
    }
    impl Game {
        pub fn run(&mut self) {
            let mut rng = rand::thread_rng();
            let mut map = [["[ ]"; 3]; 3];
            println!("{}", self.name);

            let mut err = "";
            let mut player = String::new();

            let random_num = rng.gen_range(1..=2);
            println!("{}", random_num);
            if random_num == 1 {
                player = "крестик".to_string();
            } else {
                player = "нолик".to_string();
            }


            loop {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

                if player == "крестик" {
                    player = "нолик".to_string();
                } else {
                    player = "крестик".to_string();
                }


                println!("Сейчас ходит {}", player);
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
                            if player == "крестик" {
                                map[row][cell_index] = "[x]"
                            } else {
                                map[row][cell_index] = "[o]"
                            }
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