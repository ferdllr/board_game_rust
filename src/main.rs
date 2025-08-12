use std::io;
fn main() {
    let mut board: [[usize; 5]; 5] = [[0; 5]; 5];
    let mut player: [isize; 2] = [2, 2];
    let mut enemy: [isize; 2] = [0, 0];
    let mut lose = false;
    display_table(&mut board, player, enemy);
    while !lose {
        println!("insira uma direção: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("insira apenas U, D, L e R");
        let direction = input.trim().to_uppercase();
        player = move_player(player, &direction, enemy);
        enemy = move_enemy(player, enemy);
        lose = check_win(player, enemy);
        display_table(&mut board, player, enemy);
    }
    println!("você perdeu");
}
fn check_win(p: [isize; 2], enemy: [isize; 2]) -> bool {
    let mut result = false;
    if p == enemy {
        result = true;
    }
    result
}
fn display_table(board: &mut [[usize; 5]; 5], pl: [isize; 2], enemy: [isize; 2]) {
    for row in board.iter_mut() {
        for square in row.iter_mut() {
            *square = 0;
        }
    }
    board[pl[0] as usize][pl[1] as usize] = 1;
    board[enemy[0] as usize][enemy[1] as usize] = 2;
    for row in board {
        for square in row {
            match *square {
                0 => print!(" [ ] "),
                1 => print!(" [P] "),
                2 => print!(" [E] "),
                _ => print!(" ERR "),
            }
        }
        println!(" ");
    }
    println!("=========");
}

fn move_player(pl: [isize; 2], direction: &str, enemy: [isize; 2]) -> [isize; 2] {
    let new_coord: [isize; 2] = move_entity(pl, direction);
    if new_coord == enemy {
        println!("Essa direção ja está ocupada");
        pl
    } else {
        new_coord
    }
}

fn move_entity(e: [isize; 2], direction: &str) -> [isize; 2] {
    let mut new_coord = e;
    match direction {
        "L" => {
            if e[1] > 0 {
                new_coord[1] -= 1;
            }
        }
        "R" => {
            if e[1] < 4 {
                new_coord[1] += 1;
            }
        }
        "U" => {
            if e[0] > 0 {
                new_coord[0] -= 1;
            }
        }
        "D" => {
            if e[0] < 4 {
                new_coord[0] += 1;
            }
        }
        _ => {
            println!("direção invalida")
        }
    }
    new_coord
}

fn move_enemy(pl: [isize; 2], enemy: [isize; 2]) -> [isize; 2] {
    let mut new_coord = enemy;
    let dist_x = pl[1] - enemy[1];
    let dist_y = pl[0] - enemy[0];
    if dist_x.abs() > dist_y.abs() {
        new_coord[1] += dist_x.signum();
    } else {
        new_coord[0] += dist_y.signum();
    }
    new_coord
}
