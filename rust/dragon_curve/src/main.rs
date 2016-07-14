// Y is up
use Turn::*;

#[derive(Copy, Clone)]
enum Turn {
    Left,
    Right,
}

impl Turn {
    fn flip(&self) -> Self {
        match *self {
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Copy, Clone)]
struct Dir(i32, i32);

impl Dir {
    fn turn(&self, turn: &Turn) -> Self {
        let Dir(x, y) = *self;
        match *turn {
            Left => Dir(-y, x),
            Right => Dir(y, -x),
        }
    }
}

#[derive(Copy, Clone)]
struct Pt(i32, i32);

impl std::ops::Add<Dir> for Pt {
    type Output = Pt;
    fn add(self, other: Dir) -> Pt {
        let (Pt(x1, y1), Dir(x2, y2)) = (self, other);
        Pt(x1 + x2, y1 + y2)
    }
}

fn dragon_curve(iterations: u32) -> Vec<Turn> {
    let mut curve = vec![Right];

    for _ in 0..iterations {
        let mut new_curve = vec![];
        for (&turn, i) in curve.iter().zip(0..) {
            let new_turn = match i % 2 {
                0 => Right,
                _ => Left,
            };
            new_curve.push(turn);
            new_curve.push(new_turn);
        }

        curve = new_curve;
    }

    curve
}

fn get_points(curve: &Vec<Turn>) -> Vec<Pt> {
    let mut pt = Pt(0, 0);
    let mut dir = Dir(-1, 0);
    let mut pts = Vec::with_capacity(curve.len() + 1);

    pts.push(pt);

    for turn in curve {
        dir = dir.turn(turn);
        pt = pt + dir;
        pts.push(pt);
    }
    pts
}

fn main() {
    let curve = dragon_curve(12);

    let string = curve.iter()
                      .map(|t| match *t {
                          Left => "L",
                          Right => "R",
                      })
                      .collect::<Vec<_>>()
                      .join("");
    println!("{}", string);

    let pts = get_points(&curve);

    // print out x, y pairs so that they can be copied into Excel.
    for pt in pts {
        let Pt(x, y) = pt;
        println!("{}\t{}", x, y);
    }
}
