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
        let mut valid = true;
        
        let games = line.split(";")
            .map(| games | games.trim());

        'outer_1: for game in games {
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
                            if amount > RED_CUBES {
                                valid = false;
                                break 'outer_1;
                            }
                        },
                        "green" => {
                            if amount > GREEN_CUBES {
                                valid = false;
                                break 'outer_1;
                            }
                        },
                        "blue" => {
                            if amount > BLUE_CUBES {
                                valid = false;
                                break 'outer_1;
                            }
                        },
                        _ => panic!("Something went royally wrong!")
                    }
                }
            }
        }
    
        if valid {
            sum += game_id;
        }
    }

    println!("The sum of all possible game ids is: {}", sum);
}
