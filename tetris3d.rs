// tetris3d.rs — трёхмерный тетрис на Rust

use std::io::{self, Write, stdin, stdout};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use rand::Rng;
use termion::{clear, cursor, color, style};
use termion::input::TermRead;

const W: usize = 4;
const H: usize = 4;
const D: usize = 4;

type Point = (i32, i32, i32);
type Piece = Vec<Point>;

struct Tetris3D {
    field: [[[u8; D]; H]; W],
    score: i32,
    level: i32,
    fall_interval: f64,
    current_piece: Piece,
    next_piece: Piece,
    current_pos: Point,
    game_over: bool,
}

impl Tetris3D {
    fn new() -> Self {
        let shapes: Vec<Piece> = vec![
            vec![(0,0,0),(1,0,0),(0,1,0),(1,1,0)],
            vec![(0,0,0),(1,0,0),(2,0,0),(3,0,0)],
            vec![(0,0,0),(1,0,0),(2,0,0),(1,1,0)],
            vec![(0,0,0),(1,0,0),(2,0,0),(0,1,0)],
            vec![(0,0,0),(1,0,0),(2,0,0),(2,1,0)],
            vec![(1,0,0),(2,0,0),(0,1,0),(1,1,0)],
            vec![(0,0,0),(1,0,0),(1,1,0),(2,1,0)],
        ];
        let mut rng = rand::thread_rng();
        let next = shapes[rng.gen_range(0..shapes.len())].clone();
        let mut game = Tetris3D {
            field: [[[0; D]; H]; W],
            score: 0,
            level: 1,
            fall_interval: 1.0,
            current_piece: Vec::new(),
            next_piece: next,
            current_pos: (0,0,0),
            game_over: false,
        };
        game.spawn_piece();
        game
    }

    fn is_valid(&self, piece: &Piece, pos: Point) -> bool {
        let (px, py, pz) = pos;
        for &(dx, dy, dz) in piece {
            let x = px + dx;
            let y = py + dy;
            let z = pz + dz;
            if x < 0 || x >= W as i32 || y < 0 || y >= H as i32 || z < 0 || z >= D as i32 {
                return false;
            }
            if self.field[x as usize][y as usize][z as usize] != 0 {
                return false;
            }
        }
        true
    }

    fn place_piece(&mut self) {
        let (px, py, pz) = self.current_pos;
        for &(dx, dy, dz) in &self.current_piece {
            self.field[(px+dx) as usize][(py+dy) as usize][(pz+dz) as usize] = 1;
        }
        self.clear_layers();
        self.spawn_piece();
    }

    fn clear_layers(&mut self) {
        let mut cleared = 0;
        let mut z = 0;
        while z < D {
            let mut full = true;
            for x in 0..W {
                for y in 0..H {
                    if self.field[x][y][z] == 0 {
                        full = false;
                        break;
                    }
                }
                if !full { break; }
            }
            if full {
                for zz in z..D-1 {
                    for x in 0..W {
                        for y in 0..H {
                            self.field[x][y][zz] = self.field[x][y][zz+1];
                        }
                    }
                }
                for x in 0..W {
                    for y in 0..H {
                        self.field[x][y][D-1] = 0;
                    }
                }
                cleared += 1;
                // после сдвига, проверяем этот же уровень снова
            } else {
                z += 1;
            }
        }
        if cleared > 0 {
            self.score += cleared * 100;
            self.level = 1 + self.score / 500;
            self.fall_interval = (1.0 / (1.0 + (self.level-1) as f64 * 0.2)).max(0.2);
        }
    }

    fn spawn_piece(&mut self) {
        let shapes: Vec<Piece> = vec![
            vec![(0,0,0),(1,0,0),(0,1,0),(1,1,0)],
            vec![(0,0,0),(1,0,0),(2,0,0),(3,0,0)],
            vec![(0,0,0),(1,0,0),(2,0,0),(1,1,0)],
            vec![(0,0,0),(1,0,0),(2,0,0),(0,1,0)],
            vec![(0,0,0),(1,0,0),(2,0,0),(2,1,0)],
            vec![(1,0,0),(2,0,0),(0,1,0),(1,1,0)],
            vec![(0,0,0),(1,0,0),(1,1,0),(2,1,0)],
        ];
        let mut rng = rand::thread_rng();
        self.current_piece = self.next_piece.clone();
        self.next_piece = shapes[rng.gen_range(0..shapes.len())].clone();
        self.current_pos = (1,1,3);
        if !self.is_valid(&self.current_piece, self.current_pos) {
            self.game_over = true;
        }
    }

    fn move_piece(&mut self, dx: i32, dy: i32, dz: i32) {
        let new_pos = (self.current_pos.0 + dx, self.current_pos.1 + dy, self.current_pos.2 + dz);
        if self.is_valid(&self.current_piece, new_pos) {
            self.current_pos = new_pos;
        }
    }

    fn rotate(&mut self, axis: char) {
        let new_piece: Piece = self.current_piece.iter().map(|&(x,y,z)| {
            match axis {
                'z' => (-y, x, z),
                'x' => (x, -z, y),
                'y' => (z, y, -x),
                _ => (x,y,z),
            }
        }).collect();
        if self.is_valid(&new_piece, self.current_pos) {
            self.current_piece = new_piece;
        }
    }

    fn hard_drop(&mut self) {
        while self.is_valid(&self.current_piece, (self.current_pos.0, self.current_pos.1, self.current_pos.2 - 1)) {
            self.current_pos.2 -= 1;
        }
        self.place_piece();
    }

    fn update(&mut self) {
        if self.is_valid(&self.current_piece, (self.current_pos.0, self.current_pos.1, self.current_pos.2 - 1)) {
            self.current_pos.2 -= 1;
        } else {
            self.place_piece();
        }
    }

    fn draw(&self) {
        print!("{}{}", clear::All, cursor::Goto(1,1));
        println!("ТЕТРИС 3D");
        println!("Очки: {} | Уровень: {}", self.score, self.level);
        for z in (0..D).rev() {
            println!("\nСЛОЙ {} (Y={})", z+1, z);
            println!("  0 1 2 3");
            for y in (0..H).rev() {
                print!("{} ", y);
                for x in 0..W {
                    if self.field[x][y][z] != 0 {
                        print!("X ");
                    } else {
                        let mut in_piece = false;
                        for &(dx,dy,dz) in &self.current_piece {
                            if x as i32 == self.current_pos.0+dx && y as i32 == self.current_pos.1+dy && z as i32 == self.current_pos.2+dz {
                                in_piece = true;
                                break;
                            }
                        }
                        print!("{} ", if in_piece { "█" } else { "." });
                    }
                }
                println!();
            }
        }
        println!("\nСледующая фигура:");
        for dy in 0..2 {
            for dx in 0..2 {
                let mut found = false;
                for &(ex,ey,ez) in &self.next_piece {
                    if ex == dx && ey == dy && ez == 0 {
                        found = true;
                        break;
                    }
                }
                print!("{} ", if found { "█" } else { "." });
            }
            println!();
        }
        println!("\nWASD - движение, Q/E - поворот Z, R - поворот X, Space - падение, Esc - выход");
        stdout().flush().unwrap();
    }

    fn run(&mut self) {
        let mut last_fall = Instant::now();
        let stdin = stdin();
        let mut keys = stdin.keys();

        while !self.game_over {
            self.draw();
            // Проверка ввода
            if let Some(Ok(key)) = keys.next() {
                match key {
                    termion::event::Key::Char('a') => self.move_piece(-1,0,0),
                    termion::event::Key::Char('d') => self.move_piece(1,0,0),
                    termion::event::Key::Char('w') => self.move_piece(0,1,0),
                    termion::event::Key::Char('s') => self.move_piece(0,-1,0),
                    termion::event::Key::Char('q') => self.rotate('z'),
                    termion::event::Key::Char('e') => { self.rotate('z'); self.rotate('z'); self.rotate('z'); },
                    termion::event::Key::Char('r') => self.rotate('x'),
                    termion::event::Key::Char(' ') => self.hard_drop(),
                    termion::event::Key::Esc => break,
                    _ => {}
                }
            }
            if last_fall.elapsed().as_secs_f64() > self.fall_interval {
                self.update();
                last_fall = Instant::now();
            }
            thread::sleep(Duration::from_millis(50));
        }
        println!("ИГРА ОКОНЧЕНА! Счёт: {}", self.score);
    }
}

fn main() {
    let mut game = Tetris3D::new();
    game.run();
}
