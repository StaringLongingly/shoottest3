use std::ops::RangeInclusive;

const WINDOW_SIZE: (usize, usize) = (200, 32);

struct Line {
    point1: (usize, usize),
    point2: (usize, usize),
}

fn main() {
    let top_down = &mut [["."; WINDOW_SIZE.0]; WINDOW_SIZE.1];
    insert_to_array(top_down, 1, 20, "0");

    let line1 = Line {
        point1: (20, 30),
        point2: (10, 30),
    };
    let line2 = Line {
        point2: (10, 20),
        ..line1
    };

    line1.overlay_line(top_down);
    line2.overlay_line(top_down);

    render(&top_down);
}

impl Line {
    fn overlay_line(self, array: &mut [[&str; WINDOW_SIZE.0]; WINDOW_SIZE.1]) {
        fn absolute_range(start: usize, end: usize) -> RangeInclusive<usize> {
            if start < end {
                return start..=end;
            } else {
                return end..=start;
            }
        }

        if self.point1.0 == self.point2.0 {
            for y in 1..=WINDOW_SIZE.1 {
                insert_to_array(array, self.point1.0, y, "*")
            }
        } else {
            let tan: f64 = (self.point1.1 as f64 - self.point2.1 as f64)
                / (self.point1.0 as f64 - self.point2.0 as f64);
            //println!("tan: {tan}");
            let offset: f64 = self.point1.1 as f64 - tan * self.point1.0 as f64;
            //println!("offset: {offset}");

            let range = absolute_range(self.point1.0, self.point2.0);
            for x in range {
                //println!("x: {x}");
                let y = tan * x as f64 + offset;
                //println!("y: {y}");
                let y: usize = y as usize;

                if y <= WINDOW_SIZE.1 && y >= 1 {
                    insert_to_array(array, x, y, "*");
                }
            }
        }
    }
}

fn insert_to_array<'a>(
    array: &mut [[&'a str; WINDOW_SIZE.0]; WINDOW_SIZE.1],
    x: usize,
    y: usize,
    value: &'a str,
) {
    array[WINDOW_SIZE.1 - y][x - 1] = value;
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
