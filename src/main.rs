use std::io;
fn main() {
    let mut board = [[0; 5]; 5];
    let mut player = [4, 4];
    let mut enemy = [0, 0];
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
fn check_win(p: [usize; 2], enemy: [usize; 2]) -> bool {
    let mut result = false;
    if p == enemy {
        result = true;
    }
    result
}
fn display_table(board: &mut [[i32; 5]; 5], pl: [usize; 2], enemy: [usize; 2]) {
    for row in board.iter_mut() {
        for square in row.iter_mut() {
            *square = 0;
        }
    }
    board[pl[0]][pl[1]] = 1;
    board[enemy[0]][enemy[1]] = 2;
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

fn move_player(pl: [usize; 2], direction: &str, enemy: [usize; 2]) -> [usize; 2] {
    let new_coord = move_entity(pl, direction);
    if new_coord == enemy {
        println!("Essa direção ja está ocupada");
        pl
    } else {
        new_coord
    }
}

fn move_entity(e: [usize; 2], direction: &str) -> [usize; 2] {
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

fn move_enemy(pl: [usize; 2], enemy: [usize; 2]) -> [usize; 2] {
    let mut new_coord = enemy;
    if pl[0] > enemy[0] {
        new_coord = move_entity(enemy, "D");
    } else if pl[0] < enemy[0] {
        new_coord = move_entity(enemy, "U");
    } else if pl[1] > enemy[1] {
        new_coord = move_entity(enemy, "R");
    } else if pl[1] < enemy[1] {
        new_coord = move_entity(enemy, "L");
    }
    new_coord
}
