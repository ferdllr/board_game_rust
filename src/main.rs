use std::io;

#[derive(PartialEq, Eq, Copy, Clone)]
struct Point {
    y: isize,
    x: isize,
}

fn main() {
    let mut board: [[usize; 5]; 5] = [[0; 5]; 5];
    let mut player = Point { y: 2, x: 2 };
    let mut enemy = Point { y: 0, x: 0 };
    let mut lose = false;

    display_table(&mut board, player, enemy);

    while !lose {
        println!("insira uma direção (W, A, S, D): ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a entrada");

        let direction = input.trim().to_uppercase();

        player = move_player(player, &direction, enemy);
        enemy = move_enemy(player, enemy);
        lose = check_win(player, enemy);

        display_table(&mut board, player, enemy);
    }
    println!("Você perdeu!");
}

fn check_win(p: Point, enemy: Point) -> bool {
    p == enemy
}

fn display_table(board: &mut [[usize; 5]; 5], pl: Point, enemy: Point) {
    for row in board.iter_mut() {
        for square in row.iter_mut() {
            *square = 0;
        }
    }

    board[pl.y as usize][pl.x as usize] = 1;
    board[enemy.y as usize][enemy.x as usize] = 2;

    for row in board {
        for square in row {
            match *square {
                0 => print!(" [ ] "),
                1 => print!(" [P] "),
                2 => print!(" [E] "),
                _ => print!(" ERR "),
            }
        }
        println!();
    }
    println!("=====================");
}

fn move_player(pl: Point, direction: &str, enemy: Point) -> Point {
    let new_coord = move_entity(pl, direction);
    if new_coord == enemy {
        println!("Essa direção ja está ocupada");
        pl
    } else {
        new_coord
    }
}

fn move_entity(e: Point, direction: &str) -> Point {
    let mut new_coord = e;
    match direction {
        "A" => {
            if new_coord.x > 0 {
                new_coord.x -= 1;
            }
        }
        "D" => {
            if new_coord.x < 4 {
                new_coord.x += 1;
            }
        }
        "W" => {
            if new_coord.y > 0 {
                new_coord.y -= 1;
            }
        }
        "S" => {
            if new_coord.y < 4 {
                new_coord.y += 1;
            }
        }
        _ => {
            println!("Direção inválida. Use W, A, S ou D.");
        }
    }
    new_coord
}

fn move_enemy(pl: Point, enemy: Point) -> Point {
    let mut new_coord = enemy;
    let dist_x = pl.x - enemy.x;
    let dist_y = pl.y - enemy.y;

    if dist_x.abs() > dist_y.abs() {
        new_coord.x += dist_x.signum();
    } else {
        new_coord.y += dist_y.signum();
    }
    new_coord
}
