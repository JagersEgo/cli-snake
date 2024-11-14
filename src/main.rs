use rand::Rng;
use std::{io, process::exit, usize};

fn main() {
    println!("=============== RESTART ===============");
    let board_size_x: usize = 8;
    let board_size_y: usize = 8;
    
    let mut board: Vec<Vec<String>> = make_board(board_size_x, board_size_y);

    let mut snake = Snake::new(vec![0usize, 0usize]);

    let mut input = "s".to_string();
    while input != "" {
        board = add_food(board, board_size_x, board_size_y);
        board = snake.move_snake(board, input);
        print_board(&board, board_size_y);
        input = get_input();
    }
}

fn make_board(x: usize, y: usize) -> Vec<Vec<String>> {
    let mut board: Vec<Vec<String>> = Vec::new();
    for row in 0..y {
        board.push(Vec::new());

        for _col in 0..x {
            board[row].push("□".to_string()); //▢
        }

    }
    return board
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.truncate(input.len() - 1);
    return input
}

fn add_food(mut board: Vec<Vec<String>>, board_size_x: usize, board_size_y: usize) -> Vec<Vec<String>> {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    loop {
        let x = rng.gen_range(0..board_size_x);
        let y = rng.gen_range(0..board_size_y);

        if board[y][x] == "□" {
            board[y][x] = "◆".to_string();
        }
        return board
    }
}

fn print_board(board: &Vec<Vec<String>>, board_size_y: usize) {
    for line in 0..board_size_y {
        println!("{}", board[line].join(" "));
    }
    println!("-");
}

struct Snake {
    snake_head_pos: Vec<usize>,
    max_len: usize,
    len: usize,
    tail_positions: Vec<Vec<usize>>
}

impl Snake {
    fn new(snake_head_pos: Vec<usize>) -> Self {
        Snake {
            snake_head_pos: snake_head_pos,
            max_len: 1,
            len: 1,
            tail_positions: Vec::new(),
        }
    }

    fn move_snake(&mut self, mut board: Vec<Vec<String>>, input: String) -> Vec<Vec<String>> {
        self.tail_positions.push(self.snake_head_pos.clone());
        
        let head: &str;
        if input == "w" {
            self.snake_head_pos[1] -= 1;
            head = "🠵";
        } else if input == "a" {
            self.snake_head_pos[0] -= 1;
            head = "🠴";
        } else if input == "s" {
            self.snake_head_pos[1] += 1;
            head = "🠷";
        } else if input == "d" {
            self.snake_head_pos[0] += 1;
            head = "🠶";
        } else {
            head = "E";
        }
        // println!("{:?}", self.snake_head_pos);

        // Eat logic
        if board[self.snake_head_pos[1]][self.snake_head_pos[0]] == "◆" {
            println!("{:?}", self.max_len);
            self.max_len += 1;
        } else if board[self.snake_head_pos[1]][self.snake_head_pos[0]] == "■" {
            println!("{}", "LOSER");
            exit(123)
        } else {
            // Decay tail, stack
            board[self.tail_positions[0][1]][self.tail_positions[0][0]] = "□".to_string();
            self.tail_positions.remove(0);
        }
        

        // Draw
        board = self.draw(board, head);

        return board
    }

    fn draw(&self, mut board: Vec<Vec<String>>, head_sign: &str) -> Vec<Vec<String>> {
        board[self.snake_head_pos[1]][self.snake_head_pos[0]] = head_sign.to_string();
        
        for pos in &self.tail_positions {
            board[pos[1]][pos[0]] = "■".to_string()
        }
        return board
    }
}