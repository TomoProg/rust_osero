use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, Hash)]
enum FieldValue {
    Black,
    White,
    Wall,
    None,
}

struct Position {
    row: usize,
    col: usize,
}
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

struct Field {
    field: [[FieldValue; 10]; 10],
}

impl Field {
    fn new() -> Field {
        Field {
            field: [
                [FieldValue::Wall, FieldValue::Wall, FieldValue::Wall, FieldValue::Wall, FieldValue::Wall,  FieldValue::Wall,  FieldValue::Wall, FieldValue::Wall, FieldValue::Wall, FieldValue::Wall],
                [FieldValue::Wall, FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::None,  FieldValue::None,  FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::Wall],
                [FieldValue::Wall, FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::None,  FieldValue::None,  FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::Wall],
                [FieldValue::Wall, FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::None,  FieldValue::None,  FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::Wall],
                [FieldValue::Wall, FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::Black, FieldValue::White, FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::Wall],
                [FieldValue::Wall, FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::White, FieldValue::Black, FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::Wall],
                [FieldValue::Wall, FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::None,  FieldValue::None,  FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::Wall],
                [FieldValue::Wall, FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::None,  FieldValue::None,  FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::Wall],
                [FieldValue::Wall, FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::None,  FieldValue::None,  FieldValue::None, FieldValue::None, FieldValue::None, FieldValue::Wall],
                [FieldValue::Wall, FieldValue::Wall, FieldValue::Wall, FieldValue::Wall, FieldValue::Wall,  FieldValue::Wall,  FieldValue::Wall, FieldValue::Wall, FieldValue::Wall, FieldValue::Wall],
            ]
        }
    }

    fn put(&mut self, pos: Position, v: FieldValue) -> Option<String> {
        let available_positions = self.search_available_positions(&v);
        match available_positions.iter().find(|&p| p.row == pos.row && p.col == pos.col) {
            Some(p) => {
                for rp in self.search_reversed_positions(&p, &v).iter() {
                    self.field[rp.row][rp.col] = v;
                }
                self.field[p.row][p.col] = v;
                None
            }
            None => {
                Some(format!("({},{})には置けません", pos.row, pos.col))
            }
        }
    }

    fn can_put(&self, v: &FieldValue) -> bool {
        self.search_available_positions(v).len() > 0
    }

    fn search_available_positions(&self, value: &FieldValue) -> Vec<Position> {
        let mut result = Vec::new();
        for (row_i, row) in self.field.iter().enumerate() {
            for (col_i, v) in row.iter().enumerate() {
                if v == &FieldValue::None {
                    let pos = Position { row: row_i, col: col_i };
                    if self.search_reversed_positions(&pos, value).len() == 0 {
                        continue;
                    }
                    result.push(pos);
                }
            }
        }
        result
    }

    fn search_reversed_positions(&self, pos: &Position, value: &FieldValue) -> Vec<Position> {
        let mut result: Vec<Vec<Position>> = vec![];
        let enemy = match value {
            FieldValue::White => FieldValue::Black,
            FieldValue::Black => FieldValue::White,
            _ => return result.into_iter().flatten().collect(),
        };

        let directions: [[i32; 2]; 8] = [[-1, 0], [-1, 1], [0, 1], [1, 1], [1, 0], [1, -1], [0, -1], [-1, -1]];
        for d in directions.iter() {
            let mut row_i = ((pos.row as i32) + d[0]) as usize;
            let mut col_i = ((pos.col as i32) + d[1]) as usize;
            let mut positions = vec![];
            if self.field[row_i][col_i] == enemy {
                positions.push(Position { row: row_i, col: col_i });
                loop {
                    row_i = (row_i as i32 + d[0]) as usize;
                    col_i = (col_i as i32 + d[1]) as usize;
                    let v = self.field[row_i][col_i];
                    if v == enemy {
                        positions.push(Position { row: row_i, col: col_i });
                    } else if v == *value {
                        break;
                    } else {
                        positions = Vec::new();
                        break;
                    }
                }
            }
            result.push(positions);
        }

        result.into_iter().flatten().collect()
    }

    fn aggregate(&self) -> HashMap<FieldValue, u32> {
        let mut result: HashMap<FieldValue, u32> = HashMap::new();
        result.insert(FieldValue::Black, 0);
        result.insert(FieldValue::White, 0);
        result.insert(FieldValue::Wall, 0);
        result.insert(FieldValue::None, 0);

        for row in self.field.iter() {
            for v in row {
                if let Some(c) = result.get_mut(v) {
                    *c += 1;
                }
            }
        }
        result
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str("  1 2 3 4 5 6 7 8");
        for (i, row) in self.field.iter().enumerate() {
            if 0 < i && i < self.field.len() - 1 {
                result.push_str(&format!("{} ", i.to_string()));
            }
            for v in row {
                result.push_str(match v {
                    FieldValue::Black => "B ",
                    FieldValue::White => "W ",
                    FieldValue::Wall => "",
                    FieldValue::None => "+ ",
                });
            }
            result.push_str("\n");
        }
        result
    }
}

fn forcibly_print(msg: &str) {
    // Rustでは標準出力は行バッファリングされており、改行がないと出力されないため、
    // print!はすぐには表示されない(println!は改行付きのためすぐに表示される)
    // そのため、print!のあとにflushすることで即表示するようにしている
    print!("{}", msg);
    io::stdout().flush().unwrap();
}

fn read_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}

fn parse_position(s: &str) -> Result<Position, &str> {
    let pos: Vec<&str> = s.split(',').collect();
    if pos.len() < 2 {
        return Err("指定した座標は無効です。")
    }

    let row = pos[0].parse::<usize>();
    let col = pos[1].parse::<usize>();
    if row.is_err() || col.is_err() {
        return Err("指定した座標は無効です。")
    }

    Ok(Position { row: row.unwrap(), col: col.unwrap() })
}

fn main() {
    println!("----------------------");
    println!("-     Rust Osero     -");
    println!("----------------------");
    println!("プログラミング言語 Rustで作ったオセロゲームです。");
    forcibly_print("ゲームを開始しますか？(y/n):");
    if read_line() != "y" {
        println!("ゲームを終了します。");
        return;
    }

    let mut field = Field::new();
    let mut turn = 1;

    loop {
        let value = match turn % 2 {
            1 => FieldValue::Black,
            _ => FieldValue::White,
        };

        println!("----------------------");
        if value == FieldValue::Black {
            println!("黒のターン");
        } else {
            println!("白のターン");
        }
        let result = field.aggregate();
        let black_count = result.get(&FieldValue::Black).unwrap();
        let white_count = result.get(&FieldValue::White).unwrap();
        println!("黒:{:?} 白:{:?}", black_count, white_count);
        println!("");
        println!("{}", field.to_string());

        let available_positions = field.search_available_positions(&value);
        if available_positions.len() == 0 {
            println!("置ける場所がないためスキップします。");
            turn += 1;
            continue;
        }
        
        println!("置ける場所");
        for pos in available_positions {
            println!("{}", pos);
        }

        loop {
            forcibly_print("石を置く座標を指定してください。例：1,2 -> ");
            let buf = read_line();
            let pos = match parse_position(&buf) {
                Ok(p) => p,
                Err(msg) => {
                    println!("{}", msg);
                    continue;
                },
            };
            if let Some(msg) = field.put(pos, value) {
                println!("{}", msg);
                continue;
            }
            break;
        }

        if !field.can_put(&FieldValue::Black) && !field.can_put(&FieldValue::White) {
            // どちらも置ける場所が無くなったら終了
            break;
        }

        turn += 1;
    }

    println!("----------------------");
    println!("-        結果         -");
    println!("----------------------");
    println!("{}", field.to_string());
    let result = field.aggregate();
    let black_count = result.get(&FieldValue::Black).unwrap();
    let white_count = result.get(&FieldValue::White).unwrap();
    println!("黒:{:?} 白:{:?}", black_count, white_count);
    if black_count > white_count {
        println!("黒の勝ちです。");
    } else if black_count < white_count {
        println!("白の勝ちです。");
    } else {
        println!("引き分けです。");
    }
}
