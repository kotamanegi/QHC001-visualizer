#![allow(non_snake_case, unused_macros)]
use std::cmp::min;

use proconio::input;
use rand::prelude::*;
use svg::node::element::{
    path::Data, Circle, Definitions, Group, Image, Line, Path, Rectangle, Style, Use,
};

#[derive(Clone, Debug)]
pub struct Input {
    pub d: usize,
    pub c: usize,
    pub e_d: f64,
    pub e_m: f64,
    pub t: usize,
    pub testcase: Vec<Instance>,
}

#[derive(Clone, Debug)]
pub struct Instance {
    pub x_depolarizing_error: Vec<Vec<(usize, usize)>>,
    pub z_depolarizing_error: Vec<Vec<(usize, usize)>>,
    pub measure_error: Vec<Vec<(usize, usize)>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {} {} {} {}",
            self.d, self.c, self.e_d, self.e_m, self.t
        )?;

        let mut x_counter = 0;
        for case in 0..self.t {
            for turn in 0..self.c {
                x_counter += self.testcase[case].x_depolarizing_error[turn].len();
            }
        }
        writeln!(f, "{}", x_counter)?;
        for case in 0..self.t {
            for turn in 0..self.c {
                for i in 0..self.testcase[case].x_depolarizing_error[turn].len() {
                    writeln!(
                        f,
                        "{} {} {} {}",
                        case,
                        turn,
                        self.testcase[case].x_depolarizing_error[turn][i].0,
                        self.testcase[case].x_depolarizing_error[turn][i].1
                    )?;
                }
            }
        }

        let mut z_counter = 0;
        for case in 0..self.t {
            for turn in 0..self.c {
                z_counter += self.testcase[case].z_depolarizing_error[turn].len();
            }
        }
        writeln!(f, "{}", z_counter)?;
        for case in 0..self.t {
            for turn in 0..self.c {
                for i in 0..self.testcase[case].z_depolarizing_error[turn].len() {
                    writeln!(
                        f,
                        "{} {} {} {}",
                        case,
                        turn,
                        self.testcase[case].z_depolarizing_error[turn][i].0,
                        self.testcase[case].z_depolarizing_error[turn][i].1
                    )?;
                }
            }
        }

        let mut m_counter = 0;
        for case in 0..self.t {
            for turn in 0..self.c {
                m_counter += self.testcase[case].measure_error[turn].len();
            }
        }
        writeln!(f, "{}", m_counter)?;
        for case in 0..self.t {
            for turn in 0..self.c {
                for i in 0..self.testcase[case].measure_error[turn].len() {
                    writeln!(
                        f,
                        "{} {} {} {}",
                        case,
                        turn,
                        self.testcase[case].measure_error[turn][i].0,
                        self.testcase[case].measure_error[turn][i].1
                    )?;
                }
            }
        }

        Ok(())
    }
}

pub fn gen(seed: u64) -> Input {
    let c = 100;
    let d = 21;
    let mut t = 0;
    let mut e_d = 0.;
    let mut e_m = 0.;

    if seed > 100000 {
        t = 100;
    } else {
        t = 1;
    }

    if ((seed / 200) as usize) % 5 == 0 {
        e_d = 0.001;
        e_m = 0.01;
    } else if ((seed / 200) as usize) % 5 == 1 {
        e_d = 0.01;
        e_m = 0.03;
    } else if ((seed / 200) as usize) % 5 == 2 {
        e_d = 0.03;
        e_m = 0.00;
    } else if ((seed / 200) as usize) % 5 == 3 {
        e_d = 0.03;
        e_m = 0.05;
    } else if ((seed / 200) as usize) % 5 == 4 {
        e_d = 0.05;
        e_m = 0.1;
    }

    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let mut testcase: Vec<Instance> = vec![];
    for case in 0..t {
        let mut x_depolarizing_total = vec![];
        let mut z_depolarizing_total = vec![];
        let mut measure_total = vec![];
        for turn in 0..c {
            let mut x_depolarizing_error = vec![];
            let mut z_depolarizing_error = vec![];
            if (turn < c - 5) {
                for i in 0..d {
                    for j in 0..d {
                        if ((i + j) % 2 == 0) {
                            // with probability E_d / 3 X error occurs, with probability E_d / 3 Z error occurs, and with probability E_d / 3 Y error occurs.
                            if rng.gen_bool(e_d) {
                                let choice = rng.gen_range(0, 3);
                                if choice == 0 {
                                    x_depolarizing_error.push((i, j));
                                } else if choice == 1 {
                                    z_depolarizing_error.push((i, j));
                                } else {
                                    x_depolarizing_error.push((i, j));
                                    z_depolarizing_error.push((i, j));
                                }
                            }
                        }
                    }
                }
            }
            let mut measure_error = vec![];
            for i in 0..d {
                for j in 0..d {
                    if ((i + j) % 2 == 1) {
                        if rng.gen_bool(e_m) {
                            measure_error.push((i, j));
                        }
                    }
                }
            }
            x_depolarizing_total.push(x_depolarizing_error);
            z_depolarizing_total.push(z_depolarizing_error);
            measure_total.push(measure_error);
        }
        testcase.push(Instance {
            x_depolarizing_error: x_depolarizing_total,
            z_depolarizing_error: z_depolarizing_total,
            measure_error: measure_total,
        });
    }
    return Input {
        d: d,
        c: c,
        e_d: e_d,
        e_m: e_m,
        t: t,
        testcase: testcase,
    };
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        d: usize,
        c: usize,
        e_d: f64,
        e_m: f64,
        t: usize,
        x_size: usize,
        x_depolarizing_turn: [(usize, usize, usize, usize); x_size],
        z_size: usize,
        z_depolarizing_turn: [(usize, usize, usize, usize); z_size],
        m_size: usize,
        measure_turn: [(usize, usize, usize, usize); m_size],
    }
    let mut instances = vec![];
    for _case in 0..t {
        instances.push(Instance {
            x_depolarizing_error: vec![vec![]; c],
            z_depolarizing_error: vec![vec![]; c],
            measure_error: vec![vec![]; c],
        });
    }

    for T in 0..x_size {
        instances[x_depolarizing_turn[T].0].x_depolarizing_error[x_depolarizing_turn[T].1]
            .push((x_depolarizing_turn[T].2, x_depolarizing_turn[T].3))
    }
    for T in 0..z_size {
        instances[z_depolarizing_turn[T].0].z_depolarizing_error[z_depolarizing_turn[T].1]
            .push((z_depolarizing_turn[T].2, z_depolarizing_turn[T].3))
    }
    for T in 0..m_size {
        instances[measure_turn[T].0].measure_error[measure_turn[T].1]
            .push((measure_turn[T].2, measure_turn[T].3))
    }

    Input {
        d,
        c,
        e_d,
        e_m,
        t,
        testcase: instances,
    }
}
pub struct Output {
    pub x_correction: Vec<Vec<Vec<(usize, usize)>>>,
    pub z_correction: Vec<Vec<Vec<(usize, usize)>>>,
}

pub fn parse_output(f: &str, c: usize, t: usize) -> Output {
    if f == "" {
        return Output {
            x_correction: vec![vec![vec![]; c]; t],
            z_correction: vec![vec![vec![]; c]; t],
        };
    }

    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        x_size: usize,
        x_correction_turn: [(usize, usize, usize, usize); x_size],
        z_size: usize,
        z_correction_turn: [(usize, usize, usize, usize); z_size],
    }
    let mut x_correction: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![vec![]; c]; t];
    let mut z_correction: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![vec![]; c]; t];
    for T in 0..x_size {
        x_correction[x_correction_turn[T].0][x_correction_turn[T].1]
            .push((x_correction_turn[T].2, x_correction_turn[T].3))
    }
    for T in 0..z_size {
        z_correction[z_correction_turn[T].0][z_correction_turn[T].1]
            .push((z_correction_turn[T].2, z_correction_turn[T].3))
    }
    Output {
        x_correction: x_correction,
        z_correction: z_correction,
    }
}

fn hakidashi(
    x_error: Vec<Vec<usize>>,
    z_error: Vec<Vec<usize>>,
) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut new_x_error: Vec<Vec<usize>> = x_error.clone();
    let mut new_z_error: Vec<Vec<usize>> = z_error.clone();

    let d = x_error.len();
    // Xエラーを掃き出す
    for i in (0..d).rev() {
        if (i % 2 == 0) {
            continue;
        }
        for j in 0..d {
            if (j % 2 == 1) {
                continue;
            }
            if (new_x_error[i + 1][j] == 1) {
                new_x_error[i + 1][j] ^= 1;
                if (j != 0) {
                    new_x_error[i][j - 1] ^= 1;
                }
                if (j != d - 1) {
                    new_x_error[i][j + 1] ^= 1;
                }
                new_x_error[i - 1][j] ^= 1;
            }
        }
    }
    // Zエラーを掃き出す
    for j in (0..d).rev() {
        if (j % 2 == 0) {
            continue;
        }
        for i in 0..d {
            if (i % 2 == 1) {
                continue;
            }
            if (new_z_error[i][j + 1] == 1) {
                new_z_error[i][j + 1] ^= 1;
                if (i != 0) {
                    new_z_error[i - 1][j] ^= 1;
                }
                if (i != d - 1) {
                    new_z_error[i + 1][j] ^= 1;
                }
                new_z_error[i][j - 1] ^= 1;
            }
        }
    }

    return (new_x_error, new_z_error);
}

fn calculate_score(input: &Input, output: &Output) -> i64 {
    let mut total_score = 0;
    for case in 0..input.t {
        let mut x_error: Vec<Vec<usize>> = vec![];
        let mut z_error: Vec<Vec<usize>> = vec![];
        for i in 0..input.d {
            x_error.push(vec![0; input.d]);
            z_error.push(vec![0; input.d]);
        }

        for z in 0..input.c {
            for i in 0..input.testcase[case].z_depolarizing_error[z].len() {
                let (x, y) = input.testcase[case].z_depolarizing_error[z][i];
                z_error[x][y] ^= 1;
            }
            for i in 0..input.testcase[case].x_depolarizing_error[z].len() {
                let (x, y) = input.testcase[case].x_depolarizing_error[z][i];
                x_error[x][y] ^= 1;
            }
        }

        for z in 0..input.c {
            for i in 0..output.z_correction[case][z].len() {
                let (x, y) = output.z_correction[case][z][i];
                z_error[x][y] ^= 1;
            }
            for i in 0..output.x_correction[case][z].len() {
                let (x, y) = output.x_correction[case][z][i];
                x_error[x][y] ^= 1;
            }
        }

        let mut faulty_syndrome_count = 0;
        for i in 0..input.d {
            for j in 0..input.d {
                if (i + j) % 2 == 0 {
                    continue;
                }
                let mut correction = 0;
                let dx = [0, 0, 1, -1];
                for dir in 0..4 {
                    let nx = i as i32 + dx[dir];
                    let ny = j as i32 + dx[3 - dir];
                    if nx < 0 || ny < 0 || nx >= input.d as i32 || ny >= input.d as i32 {
                        continue;
                    }
                    if j % 2 == 1 {
                        if x_error[nx as usize][ny as usize] == 1 {
                            correction += 1;
                        }
                    } else {
                        if z_error[nx as usize][ny as usize] == 1 {
                            correction += 1;
                        }
                    }
                }
                faulty_syndrome_count += correction % 2;
            }
        }
        total_score += (5000.0 * (220.0 - (faulty_syndrome_count as f64)) / 220.0).round() as i64;

        if (faulty_syndrome_count == 0) {
            // bonus point!
            // 掃き出す
            let (new_x_error, new_z_error) = hakidashi(x_error, z_error);
            if (new_x_error[0][0] + new_z_error[0][0]) == 0 {
                // 両方正しい
                total_score += 5000;
            } else if (new_x_error[0][0] + new_z_error[0][0]) == 1 {
                // 片方は正しい
                total_score += 2000;
            }
        }
    }

    return total_score;
}

pub fn vis(input: &Input, output: &Output, turn: usize) -> (i64, String, String) {
    // validation
    for turn in 0..input.t {
        for i in 0..output.x_correction[turn].len() {
            for j in 0..output.x_correction[turn][i].len() {
                let (x, y) = output.x_correction[turn][i][j];
                if (x >= input.d || y >= input.d || x < 0 || y < 0) {
                    return (
                        0,
                        format!("テストケース {} において、サイクル {} に範囲外の座標 ({} {}) のデータ量子ビットに X エラーがあると出力しています。", turn, i, x, y),
                        "".to_string(),
                    );
                }
                if ((x + y) % 2 != 0) {
                    return (
                        0,
                        format!("テストケース {} において、サイクル {} にデータ量子ビットではない座標 ({} {}) に X エラーがあると出力しています。", turn, i, x, y),
                        "".to_string(),
                    );
                }
            }
        }
        for i in 0..output.z_correction[turn].len() {
            for j in 0..output.z_correction[turn][i].len() {
                let (x, y) = output.z_correction[turn][i][j];
                if (x >= input.d || y >= input.d || x < 0 || y < 0) {
                    return (
                        0,
                        format!("テストケース {} において、サイクル {} に範囲外の座標 ({} {}) のデータ量子ビットに Z エラーがあると出力しています。", turn, i, x, y),
                        "".to_string(),
                    );
                }
                if ((x + y) % 2 != 0) {
                    return (
                        0,
                        format!("テストケース {} において、サイクル {} にデータ量子ビットではない座標 ({} {}) に Z エラーがあると出力しています。", turn, i, x, y),
                        "".to_string(),
                    );
                }
            }
        }
    }

    let score = calculate_score(input, output);
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, 1000, 1000))
        .set("width", 1010)
        .set("height", 1010)
        .set("style", "background-color:white");
    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central; font-size: {}}}",
        6
    )));

    let box_size = 1000 / input.d;
    let black = "#000000";
    let white = "#ffffff";
    let red = "#e60033";
    let blue = "#0095d9";
    let yellow = "#F4E511";
    let green = "#00ff00";

    for i in 0..input.d {
        for j in 0..input.d {
            doc = doc.add(
                Rectangle::new()
                    .set("x", i * box_size)
                    .set("y", j * box_size)
                    .set("width", box_size)
                    .set("height", box_size)
                    .set("fill", white)
                    .set("stroke", "black")
                    .set("stroke-width", 1)
                    .set("class", "box"),
            )
        }
    }

    // prepare d*d matrix
    let mut x_error: Vec<Vec<usize>> = vec![];
    let mut z_error: Vec<Vec<usize>> = vec![];
    let mut measure_error: Vec<Vec<usize>> = vec![];
    for i in 0..input.d {
        x_error.push(vec![0; input.d]);
        z_error.push(vec![0; input.d]);
        measure_error.push(vec![0; input.d]);
    }

    for z in 0..(min((turn / 2) + 1, input.c)) {
        for i in 0..input.testcase[0].z_depolarizing_error[z].len() {
            let (x, y) = input.testcase[0].z_depolarizing_error[z][i];
            z_error[x][y] ^= 1;
        }
        for i in 0..input.testcase[0].x_depolarizing_error[z].len() {
            let (x, y) = input.testcase[0].x_depolarizing_error[z][i];
            x_error[x][y] ^= 1;
        }
    }

    for z in 0..(min((turn + 1) / 2, input.c)) {
        for i in 0..output.z_correction[0][z].len() {
            let (x, y) = output.z_correction[0][z][i];
            z_error[x][y] ^= 1;
        }
        for i in 0..output.x_correction[0][z].len() {
            let (x, y) = output.x_correction[0][z][i];
            x_error[x][y] ^= 1;
        }
    }

    if turn < input.c * 2 {
        for i in 0..input.testcase[0].measure_error[turn / 2].len() {
            let (x, y) = input.testcase[0].measure_error[turn / 2][i];
            measure_error[x][y] ^= 1;
        }
    }

    if (score >= 5000 && turn == input.c * 2 + 1) {
        (x_error, z_error) = hakidashi(x_error, z_error);
    }

    // errors
    for i in 0..input.d {
        for j in 0..input.d {
            if (x_error[i][j] == 1) {
                doc = doc.add(
                    Rectangle::new()
                        .set("x", j * box_size)
                        .set("y", i * box_size)
                        .set("width", box_size / 2)
                        .set("height", box_size)
                        .set("fill", red)
                        .set("fill-opacity", 0.5)
                        .set("stroke", "black")
                        .set("stroke-width", 1)
                        .set("class", "box"),
                )
            }
            if (z_error[i][j] == 1) {
                doc = doc.add(
                    Rectangle::new()
                        .set("x", j * box_size + box_size / 2)
                        .set("y", i * box_size)
                        .set("width", box_size / 2)
                        .set("height", box_size)
                        .set("fill", blue)
                        .set("fill-opacity", 0.5)
                        .set("stroke", "black")
                        .set("stroke-width", 1)
                        .set("class", "box"),
                )
            }
        }
    }
    // syndromes
    for i in 0..input.d {
        for j in 0..input.d {
            if (i + j) % 2 == 0 {
                continue;
            }
            let mut correction = measure_error[i][j];
            let dx = [0, 0, 1, -1];
            for dir in 0..4 {
                let nx = i as i32 + dx[dir];
                let ny = j as i32 + dx[3 - dir];
                if nx < 0 || ny < 0 || nx >= input.d as i32 || ny >= input.d as i32 {
                    continue;
                }
                if j % 2 == 1 {
                    if x_error[nx as usize][ny as usize] == 1 {
                        correction += 1;
                    }
                } else {
                    if z_error[nx as usize][ny as usize] == 1 {
                        correction += 1;
                    }
                }
            }
            if (correction % 2 == 1) {
                doc = doc.add(
                    Rectangle::new()
                        .set("x", j * box_size + box_size / 6)
                        .set("y", i * box_size + box_size / 6)
                        .set("width", 4 * box_size / 6)
                        .set("height", 4 * box_size / 6)
                        .set("fill", if (j % 2 == 1) { red } else { blue })
                        .set("fill-opacity", 1.0)
                        .set("stroke", "black")
                        .set("stroke-width", 1)
                        .set("class", "box"),
                )
            }
        }
    }
    // qubits
    for i in 0..input.d {
        for j in 0..input.d {
            if (i + j) % 2 == 0 {
                doc = doc.add(
                    Circle::new()
                        .set("cx", i * box_size + box_size / 2)
                        .set("cy", j * box_size + box_size / 2)
                        .set("r", 8)
                        .set("fill", black)
                        .set("stroke", "black")
                        .set("stroke-width", 1)
                        .set("class", "box"),
                )
            }
        }
    }

    // syndrome graphic
    for i in 0..input.d {
        for j in 0..input.d {
            if (i + j) % 2 == 1 {
                if j % 2 == 1 {
                    // X syndrome
                    // add + mark
                    doc = doc.add(
                        Line::new()
                            .set("x1", j * box_size + box_size / 2 - 10)
                            .set("y1", i * box_size + box_size / 2)
                            .set("x2", j * box_size + box_size / 2 + 10)
                            .set("y2", i * box_size + box_size / 2)
                            .set("stroke", black)
                            .set("stroke-width", 4),
                    );
                    doc = doc.add(
                        Line::new()
                            .set("x1", j * box_size + box_size / 2)
                            .set("y1", i * box_size + box_size / 2 - 10)
                            .set("x2", j * box_size + box_size / 2)
                            .set("y2", i * box_size + box_size / 2 + 10)
                            .set("stroke", black)
                            .set("stroke-width", 4),
                    );
                } else {
                    // Z syndrome
                    // add ◇ mark
                    let data = Data::new()
                        .move_to((
                            j * box_size + box_size / 2 - 10,
                            i * box_size + box_size / 2,
                        ))
                        .line_by((10, -10))
                        .line_by((10, 10))
                        .line_by((-10, 10))
                        .close();
                    doc = doc.add(
                        Path::new()
                            .set("fill", "none")
                            .set("stroke", black)
                            .set("stroke-width", 4)
                            .set("d", data),
                    );
                }
            }
        }
    }

    if turn < input.c * 2 {
        // x error graphic
        for i in 0..input.testcase[0].x_depolarizing_error[turn / 2].len() {
            let (x, y) = input.testcase[0].x_depolarizing_error[turn / 2][i];
            doc = doc.add(
                Circle::new()
                    .set("cx", y * box_size + 10)
                    .set("cy", x * box_size + 10)
                    .set("r", 8)
                    .set("fill", yellow)
                    .set("stroke", "black")
                    .set("stroke-width", 3)
                    .set("class", "box"),
            )
        }

        // z error graphic
        for i in 0..input.testcase[0].z_depolarizing_error[turn / 2].len() {
            let (x, y) = input.testcase[0].z_depolarizing_error[turn / 2][i];
            doc = doc.add(
                Circle::new()
                    .set("cx", y * box_size + box_size - 10)
                    .set("cy", x * box_size + 10)
                    .set("r", 8)
                    .set("fill", yellow)
                    .set("stroke", "black")
                    .set("stroke-width", 3)
                    .set("class", "box"),
            )
        }
    }

    if turn % 2 == 1 && turn < input.c * 2 {
        // x correction graphic

        for i in 0..output.x_correction[0][turn / 2].len() {
            let (x, y) = output.x_correction[0][turn / 2][i];
            doc = doc.add(
                Rectangle::new()
                    .set("x", y * box_size + 3)
                    .set("y", x * box_size + box_size - 18)
                    .set("width", 15)
                    .set("height", 15)
                    .set("fill", green)
                    .set("stroke", "black")
                    .set("stroke-width", 1)
                    .set("class", "box"),
            )
        }

        // z correction graphic
        for i in 0..output.z_correction[0][turn / 2].len() {
            let (x, y) = output.z_correction[0][turn / 2][i];
            doc = doc.add(
                Rectangle::new()
                    .set("x", y * box_size + box_size - 18)
                    .set("y", x * box_size + box_size - 18)
                    .set("width", 15)
                    .set("height", 15)
                    .set("fill", green)
                    .set("stroke", "black")
                    .set("stroke-width", 1)
                    .set("class", "box"),
            )
        }
    }

    if turn < input.c * 2 {
        // measurement error graphic
        for i in 0..input.testcase[0].measure_error[turn / 2].len() {
            let (x, y) = input.testcase[0].measure_error[turn / 2][i];
            doc = doc.add(
                Circle::new()
                    .set("cx", y * box_size + box_size - 10)
                    .set("cy", x * box_size + 10)
                    .set("r", 8)
                    .set("fill", yellow)
                    .set("stroke", "black")
                    .set("stroke-width", 3)
                    .set("class", "box"),
            )
        }
    }
    (score as i64, "".to_string(), doc.to_string())
}
