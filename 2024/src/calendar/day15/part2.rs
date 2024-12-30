use crate::common::{types::{Direction, Position, SimpleGrid, DIRECTIONS}, utils::{get_char_at, set}};

pub fn result(warehouse_map: &mut SimpleGrid, robot_instructions: String) -> isize {
    widen_map(warehouse_map);
    let mut robot_position: Position = get_updated_robot_position(warehouse_map);

    // println!("Widened Map: \n {:#?}",warehouse_map);

    let instructions: Vec<char> = robot_instructions.chars().collect();

    for instruction in instructions {
        let direction = get_direction_from_instruction(instruction);

        /* Check if robot can move in direction.
        if next tile is a [ or ] -> crate, could be moved, track it and check next position, 
        if next tile is a . -> empty space, robot plus tracked elements could move there
        if next tile is a # -> wall, cannot move, go to next instruction 
        note that in part 2, as the boxes are wider, a situation could arise where a tree branches off to an obstacle:
        #
        []
         []
          [][]
           []
           @
           
           instruction ^ could not move this situation 
           must always check when [ or ] the pair's obstacles ahead */

        let mut act = false;
        let mut obstacle_ahead = false;
        let mut next_start = ((robot_position.0 as isize + direction.0) as usize, (robot_position.1 as isize + direction.1) as usize);



        while !act {
            let (next_x, next_y) = next_start;
            match get_char_at((next_x, next_y), warehouse_map).as_str() {
                "[" | "]" => {
                    obstacle_ahead = true;
                },
                "." => {
                    if obstacle_ahead == true {
                        todo!("correct box moving");
                        // set(warehouse_map, (next_x, next_y), 'O');
                    }
                    // move robot
                    set(warehouse_map, ((robot_position.0 as isize + direction.0) as usize, (robot_position.1 as isize + direction.1) as usize), '@');
                    set(warehouse_map, robot_position, '.');
                    robot_position = ((robot_position.0 as isize + direction.0) as usize, (robot_position.1 as isize + direction.1) as usize);
                    act = true;
                },
                "#" => {
                    // skip turn
                    act = true;
                },
                _ => {}
            }
            next_start = ((next_start.0 as isize + direction.0) as usize, (next_start.1 as isize + direction.1) as usize);
        }        
        
    }

    println!("Part 2: \n {:#?}",warehouse_map);
    calculate_score(warehouse_map)
}

fn get_updated_robot_position(warehouse_map: &SimpleGrid) -> Position {
    let mut robot_position: Position = (0,0);

    for row in 0..warehouse_map.len() {
        for col in 0..warehouse_map[0].len() {
            if warehouse_map[row].chars().nth(col).unwrap() == '@' {
                robot_position = (col, row)
            }
        }
    }
    robot_position
}

fn widen_map(warehouse_map: &mut SimpleGrid) {
    for row in warehouse_map.iter_mut() {
        *row = row.chars()
        .flat_map(|c| match c {
            '#' => "##".chars(),  // Replace '#' with "##"
            'O' => "[]".chars(), // Replace 'O' with "[]"
            '.' => "..".chars(), // Replace '.' with ".."
            '@' => "@.".chars(), // Replace '@' with "@."
            _ => "..".chars(), // Default: duplicate the character
        })
        .collect();
    }
}

fn get_direction_from_instruction(instruction: char) -> Direction{
    let mut direction = (0,0);
    match instruction {
        '^' => {
            direction = DIRECTIONS[0];
        },
        '>' => {
            direction = DIRECTIONS[1];
        },
        'v' => {
            direction = DIRECTIONS[2];
        },
        '<' => {
            direction = DIRECTIONS[3];
        },
        _ => {
            eprintln!("Unknown instruction: {}", instruction);
        }            
    }
    direction
}

fn calculate_score(warehouse_map: &SimpleGrid) -> isize {
    let mut score = 0;
    for row in 0..warehouse_map.len() {
        for col in 0..warehouse_map[0].len() {
            if get_char_at((col,row), warehouse_map) == "[" {
                score += (100 * row) + col;
            }
        }
    }
    score as isize
}