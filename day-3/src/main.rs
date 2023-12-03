use std::fs;

fn check_boundaries(row: usize, ds: usize, de: usize, characters: &Vec<Vec<char>>) -> bool {
    // Digit starts at j and ends at j+digit_index non-inclusive
    // We need to check all the digit boundaries to see if we find a
    // symbol, except '.'

    // I have no idea how to deal with that in Rust
    // May God forgive my actions...
    let row: i32 = row.try_into().unwrap();
    let si: i32 = row - 1;
    let ei: i32 = row + 1;
    let ds: i32 = ds.try_into().unwrap();
    let de: i32 = de.try_into().unwrap();
    let rows: i32 = characters.len().try_into().unwrap();
    let cols: i32 = characters[0].len().try_into().unwrap();

    let mut valid = false;

    'row: for i in si..=ei {
        'col: for j in ds - 1..de + 1 {
            if i < 0 || i >= rows {
                continue 'row;
            }

            if j < 0 || j >= cols {
                continue 'col;
            }

            let row: usize = i.try_into().unwrap();
            let col: usize = j.try_into().unwrap();

            let c = characters[row][col];

            if c.is_ascii_punctuation() && c != '.' {
                valid = true;
            }
        }
    }

    return valid;
}

fn main() {
    let contents = fs::read_to_string("./schematic.txt").unwrap();
    let contents = contents.trim();

    let mut sum = 0;

    let mut characters: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        characters.push(line.chars().collect());
    }

    let mut i = 0;
    let mut j = 0;

    let col_boundary = characters[0].len();

    while i < characters.len() {
        j = 0;
        while j < col_boundary {
            let mut digit_index = 0;
            let mut digit_starting_index = 0;

            let current_char = characters[i][j];

            if current_char.is_numeric() {
                digit_index = j;
                digit_starting_index = j;

                while digit_index < col_boundary && characters[i][digit_index].is_numeric() {
                    digit_index += 1;
                }

                let is_valid = check_boundaries(i, digit_starting_index, digit_index, &characters);

                if is_valid {
                    let mut num = 0;
                    let mut k = digit_index - 1;
                    let mut multiplier = 1;

                    while k >= digit_starting_index {
                        num += multiplier * characters[i][k].to_digit(10).unwrap();
                        multiplier *= 10;

                        if k == 0 && digit_starting_index == 0 {
                            break;
                        }

                        k -= 1;
                    }

                    sum += num;
                }

                j = digit_index;
            } else {
                j += 1;
            }
        }
        i += 1;
    }

    println!("The final sum is equal to: {}", sum);
}
