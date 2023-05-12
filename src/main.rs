use std::io;

const WINDOW_SIZE: (usize, usize) = (100, 32);

struct AsciiTABLE<'a> {
    bg: &'a str,
    graph: &'a str,
}

static TABLE: AsciiTABLE = AsciiTABLE {
    bg: ".",
    graph: "*",
};

fn main() {

    let top_down = &mut [[TABLE.bg; WINDOW_SIZE.0]; WINDOW_SIZE.1];
    let polyonym = (1 as f64, 1 as f64, 2 as f64, 4 as f64);
    
    for constant in 1..=4 { polyonym.constant = read_int() }

    for x in 1..=(WINDOW_SIZE.0)
    {
        let x2 = x as f64;
        let mut y: f64 = polyonym.0*x2*x2*x2 + polyonym.1*x2*x2 + polyonym.2 * x2 + polyonym.3;
        if y < 0.0 { y = 0.0; }
        let y: i32 = y.round() as i32;
        insert_to_array(top_down, x as i32, y as i32, TABLE.graph);
    }

    render(&top_down);
}

fn read_int() -> f64
{
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let output: f64 = input_line.trim().parse().expect("Input not an integer");
    output
}

fn insert_to_array<'a>(
    array: &mut [[&'a str; WINDOW_SIZE.0]; WINDOW_SIZE.1],
    x: i32,
    y: i32,
    value: &'a str,
) {
    fn i32_to_usize(number: i32) -> usize {
        match number {
            ..=0 => 0,
            _ => number as usize,
        }
    }
    let y = i32_to_usize(y);
    let x = i32_to_usize(x);

    if y <= WINDOW_SIZE.1 && y >= 1 && x >= 1 && x <= WINDOW_SIZE.0 {
        array[WINDOW_SIZE.1 - y as usize][x as usize - 1] = value;
    }
}

fn render(array: &[[&str; WINDOW_SIZE.0]; WINDOW_SIZE.1]) {
    for i in 0..=(WINDOW_SIZE.1 - 1) {
        for j in 0..=(WINDOW_SIZE.0 - 1) {
            print!("{}", array[i][j]);
        }
        println!(" | {}", WINDOW_SIZE.1 - i);
    }
    for _ in 1..=WINDOW_SIZE.0 {
        print!("_")
    }
    println!();

    for j in (0..=(WINDOW_SIZE.0 - 1)).filter(|e| e % 5 == 0) {
        match j + 1 {
            ..=9 => print!("{}----", j + 1),
            ..=99 => print!("{}---", j + 1),
            _ => print!("{}--", j + 1),
        }
    }
}
