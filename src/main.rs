use std::ops::RangeInclusive;

const WINDOW_SIZE: (usize, usize) = (200, 32);

fn main() {
    let top_down = &mut [["."; WINDOW_SIZE.0]; WINDOW_SIZE.1];
    insert_to_array(top_down, 1, 20, "0");

    let line1 = Line {
        point1: (100, 30),
        point2: (50, 20),
    };
    let line2 = Line {
        point2: (30, 20),
        ..line1
    };
    let circle = Circle {
        center: (80, 15),
        radius: 15,
    };

    line1.overlay(top_down);
    line2.overlay(top_down);
    circle.overlay(top_down);

    render(&top_down);
}

struct Circle {
    center: (i32, i32),
    radius: i32,
}

impl Circle {
    fn overlay(self, array: &mut [[&str; WINDOW_SIZE.0]; WINDOW_SIZE.1]) {
        let range = -1 * self.radius..=self.radius;
        for x in range {
            //print!("x: {x}");
            let y = ((self.radius * self.radius - x * x) as f64).sqrt();
            //println!("  y: {}", y.floor());
            insert_to_array(
                array,
                x as i32 + self.center.0,
                y.floor() as i32 + self.center.1,
                "*",
            );
            insert_to_array(
                array,
                x as i32 + self.center.0,
                -y.floor() as i32 + self.center.1,
                "*",
            );
        }
    }
}

struct Line {
    point1: (usize, usize),
    point2: (usize, usize),
}

impl Line {
    fn overlay(self, array: &mut [[&str; WINDOW_SIZE.0]; WINDOW_SIZE.1]) {
        fn absolute_range(start: usize, end: usize) -> RangeInclusive<i32> {
            if start < end {
                return start as i32..=end as i32;
            } else {
                return end as i32..=start as i32;
            }
        }

        if self.point1.0 == self.point2.0 {
            for y in 1..=WINDOW_SIZE.1 as i32 {
                insert_to_array(array, self.point1.0 as i32, y, "*")
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
                let y = (tan * x as f64 + offset) as i32;
                //println!("y: {y}");
                insert_to_array(array, x, y, "*");
            }
        }
    }
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
