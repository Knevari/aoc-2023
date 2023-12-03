use std::fs;

// 12 red cubes
// 13 green cubes
// 14 blue cubes

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

fn main() {
    let contents = fs::read_to_string("./games.txt").unwrap();
    let contents = contents.trim();

    let mut sum = 0;

    // Separate the file by lines
    for (line_index, line) in contents.lines().enumerate() {
        let game_id = line_index + 1;
        // let mut valid = true;
        
        let games = line.split(";")
            .map(| games | games.trim());

        // I need to figure out the biggest values for all colors
        // for this game
        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;

        for game in games {
            // At this point I've got
            // a comma separated list of `n str`
            let hints = game.split(", ");


            for hint in hints {
                let values: Vec<&str> = hint.split_whitespace().collect();
                
                if values.len() == 2 {
                    let amount: u32 = values[0].parse().unwrap();
                    let color: &str = values[1];

                    match color {
                        "red" => {
                            if amount > max_red {
                                max_red = amount;
                            }
                            // if amount > RED_CUBES {
                            //     valid = false;
                            // }
                        },
                        "green" => {
                            if amount > max_green {
                                max_green = amount;
                            }
                            // if amount > GREEN_CUBES {
                            //     valid = false;
                            // }
                        },
                        "blue" => {
                            if amount > max_blue {
                                max_blue = amount;
                            }
                            // if amount > BLUE_CUBES {
                            //     valid = false;
                            // }
                        },
                        _ => panic!("Something went royally wrong!")
                    }
                }
            }
        
            // After checking every single hint we know for sure
            // the biggest values shown in each round
            
        }
        
        println!("Max Colors for Game {}", game_id);
        println!("Max red: {}", max_red);
        println!("Max green: {}", max_green);
        println!("Max blue: {}", max_blue);

        let power = max_red * max_green * max_blue;
        sum += power;
    }

    println!("The summation of all game powers is: {}", sum);
}
