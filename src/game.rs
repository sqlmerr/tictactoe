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
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clears console


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
                let current_player = match player.as_str() {
                    "нолик" => "[o]",
                    "крестик" => "[x]",
                    _ => {
                        println!("неизвестный игрок");
                        return
                    }
                };

                match self.calc_winner(&map, current_player.to_string()) {
                    true => {
                        println!("В ЭТОЙ ИГРЕ ПОБЕДИЛИ {}И", player.to_uppercase());
                        return
                    }
                    false => {}
                };

                if map.iter().all(|&row| row.iter().all(|&cell| cell != "[ ]")) {
                    println!("НИЧЬЯ");
                    return
                }
            }
        }

        fn calc_winner(&self, map: &[[&str; 3]; 3], current_player: String) -> bool {
            // Проверка по горизонтали
            for row in map.iter() {
                if row.iter().all(|&cell| cell == current_player) {
                    return true;
                }
            }

            // Проверка по вертикали
            for col in 0..3 {
                if (0..3).all(|row| map[row][col] == current_player) {
                    return true;
                }
            }

            // Проверка по диагонали (левая верхняя - правая нижняя)
            if (0..3).all(|i| map[i][i] == current_player) {
                return true;
            }

            // Проверка по диагонали (левая нижняя - правая верхняя)
            if (0..3).all(|i| map[i][2 - i] == current_player) {
                return true;
            }

            false
        }

    }
}