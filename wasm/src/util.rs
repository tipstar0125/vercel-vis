#![allow(non_snake_case, unused_macros)]
use proconio::input;
use svg::node::element::{Rectangle, Line};
use itertools::Itertools;
use rand::prelude::*;

pub struct Input {
    pub N: usize,
    pub C: usize,
    pub grid: Vec<Vec<isize>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.N)?;
        writeln!(f, "{}", self.C)?;
        // atcoder format
        for i in 0..self.N {
            writeln!(f, "{}", self.grid[i].iter().join(" "))?;
        }
        // official format
        // for i in 0..self.N {
        //     for j in 0..self.N {
        //         writeln!(f, "{}", self.grid[i][j])?;
        //     }
        // }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        N: usize,
        C: usize,
        grid: [[isize; N]; N]
    }
    Input { N, C, grid }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    y: usize,
    x: usize,
}

#[derive(Clone, Debug)]
pub struct Output {
    pub L: usize,
    pub actions: Vec<(Coord, Coord)>,
}

pub fn parse_output(f: &str) -> Output {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        L: usize,
        _actions: [(usize, usize, usize, usize); L]
    }
    let mut actions = vec![];
    for a in _actions {
        let now_coord = Coord { y: a.0, x: a.1 };
        let next_coord = Coord { y: a.2, x: a.3 };
        actions.push((now_coord, next_coord));
    }
    Output { L, actions }
}

pub fn gen(seed: usize) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed as u64);
    let N = rng.gen_range(8, 31);
    let C = rng.gen_range(1, 7);
    let P = rng.gen::<f64>() / 4.0;

    'outer: loop {
        let mut grid = vec![vec![0; N]; N];
        for c in 1..=C {
            for _ in 0..N {
                let mut y = rng.gen_range(0, N);
                let mut x = rng.gen_range(0, N);
                while grid[y][x] != 0 {
                    y = rng.gen_range(0, N);
                    x = rng.gen_range(0, N);
                }
                grid[y][x] = c as isize;
            }
        }
        for y in 0..N {
            for x in 0..N {
                if grid[y][x] == 0 && rng.gen::<f64>() < P {
                    grid[y][x] = -1;
                }
            }
        }

        for y in 0..N {
            let mut cnt = 0;
            for x in 0..N {
                if grid[y][x] == -1 {
                    cnt += 1;
                }
            }
            if cnt > N - C {
                continue 'outer;
            }
        }
        for x in 0..N {
            let mut cnt = 0;
            for y in 0..N {
                if grid[y][x] == -1 {
                    cnt += 1;
                }
            }
            if cnt > N - C {
                continue 'outer;
            }
        }
        return Input { N, C, grid };
    }
}

pub fn rect(y: usize, x: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}


const COLOR: [&str; 7] = ["white", "blue", "red", "green", "yellow", "brown", "purple"];


pub fn vis(input: &Input, out: &Output, turn: usize)-> (i64, String, String) {
    let delta = 600 / input.N;
    let mut doc = svg::Document::new()
        .set(
            "viewBox",
            (-5, -5, delta * input.N + 10, delta * input.N + 10),
        )
        .set("width", delta * input.N + 10)
        .set("height", delta * input.N + 10)
        .set("stroke", "gray")
        .set("stroke-width", 1);

    let mut grid = input.grid.clone();

    for t in 0..turn {
        let c = grid[out.actions[t].0.y][out.actions[t].0.x];
        grid[out.actions[t].0.y][out.actions[t].0.x] = 0;
        grid[out.actions[t].1.y][out.actions[t].1.x] = c;
    }

    for y in 0..input.N {
        for x in 0..input.N {
            if input.grid[y][x] == -1 {
                doc = doc.add(rect(y * delta, x * delta, delta, delta, "gray"));
            } else {
                doc = doc.add(rect(
                    y * delta,
                    x * delta,
                    delta,
                    delta,
                    COLOR[grid[y][x] as usize],
                ));
            }
        }
    }
    if turn > 0 {
        for t in (0..turn).rev() {
            let (start, stop) = out.actions[t].clone();
            doc = doc.add(
                Line::new()
                    .set("x1", start.x * delta + delta / 2)
                    .set("y1", start.y * delta + delta / 2)
                    .set("x2", stop.x * delta + delta / 2)
                    .set("y2", stop.y * delta + delta / 2)
                    .set("stroke", "lightgray")
                    .set("stroke-width", 10)
                    .set("stroke-linecap", "round"),
            );
            if t > 0 {
                let (_, before_stop) = out.actions[t - 1].clone();
                if before_stop != start {
                    break;
                }
            }
        }
    }
    (0, "".to_string(), doc.to_string())
}