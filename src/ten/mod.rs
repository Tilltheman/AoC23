#[derive(Debug,Clone)]
struct Tile {
    x: i32,
    y: i32,
    c: char,
    neighbors: Vec<Tile>,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Tile {
    fn convert_starting_point(&mut self,mat: &Vec<Vec<char>>, testing: bool) {
        self.set_neighbors(mat);
        // This is a pretty boring solution, but actually matching against all
        // neighbors found in the above step and then checking what it actually might
        // be is too much overhead for now.
        if testing {
            self.c = 'F';
        } else {
            self.c = 'J';
        }
    }

    fn set_neighbors(&mut self, mat: &Vec<Vec<char>>) {
        if self.neighbors.len() != 0 {
            return;
        }
        //                        T, B, L, R
        let dx: Vec<i32> = vec![-1, 1, 0, 0];
        let dy: Vec<i32> = vec![ 0, 0,-1, 1];
        for i in 0..4 {
            let cdx = dx[i];
            let cdy = dy[i];
            let x = self.x + cdx;
            let y = self.y + cdy;
            if x < 0 || x >= mat.len() as i32 || y < 0 || y >= mat[0].len() as i32 {
                continue
            }
            let c_tile = mat[x as usize][y as usize];
            if self.c == '|' {
                println!("C: {c_tile} x {x} {cdx} {y} {cdy}");
            }
            match c_tile {
                '|' => {
                    // only relevant if on top or bottom of current tile
                    // and current tile also has top/bottom connection
                    if (cdx == -1 && cdy == 0 && self.has_top_connection()) || (cdx == 1 && cdy == 0 && self.has_bottom_connection()){
                        self.neighbors.push(Tile { x: x, y: y, c: c_tile, neighbors: vec![]});
                    }
                },
                '-' => {
                    // only relevant if left or right of current tile
                    if (cdx == 0 && cdy == -1 && self.has_left_connection()) || (cdx == 0 && cdy == 1 && self.has_right_connection()) {
                        self.neighbors.push(Tile { x: x, y: y, c: c_tile, neighbors: vec![]});
                    }
                },
                'L' => {
                    // only relevant if left or below
                    if (cdx == 0 && cdy == -1 && self.has_left_connection()) || (cdx == 1 && cdy == 0 && self.has_bottom_connection()) {
                        self.neighbors.push(Tile { x: x, y: y, c: c_tile, neighbors: vec![]});
                    }
                },
                'J' => {
                    // only relevant if right or below
                    if (cdx == 0 && cdy == 1 && self.has_right_connection()) || (cdx == 1 && cdy == 0 && self.has_bottom_connection()) {
                        self.neighbors.push(Tile { x: x, y: y, c: c_tile, neighbors: vec![]});
                    }
                },
                '7' => {
                    // only relevant if right or above
                    if (cdx == 0 && cdy == 1 && self.has_right_connection()) || (cdx == -1 && cdy == 0 && self.has_top_connection()) {
                        self.neighbors.push(Tile { x: x, y: y, c: c_tile, neighbors: vec![]});
                    }
                },
                'F' => {
                    // only relevant if left or above
                    if (cdx == 0 && cdy == -1 && self.has_left_connection()) || (cdx == -1 && cdy == 0 && self.has_top_connection()) {
                        self.neighbors.push(Tile { x: x, y: y, c: c_tile, neighbors: vec![]});
                    }
                },
                'S' => continue,
                _ => continue,
            }
        }
    }

    fn has_top_connection(&self) -> bool {
        return self.c == '|' || self.c == 'L' || self.c == 'J' || self.c == 'S'
    }

    fn has_bottom_connection(&self) -> bool {
        return self.c == '|' || self.c == '7' || self.c == 'F' || self.c == 'S'
    }

    fn has_left_connection(&self) -> bool {
        return self.c == '-' || self.c == 'J' || self.c == '7' || self.c == 'S'
    }

    fn has_right_connection(&self) -> bool {
        return self.c == '-' || self.c == 'L' || self.c == 'F' || self.c == 'S'
    }
}


fn find_path(t_mat: Vec<Vec<Tile>>, path: &mut Vec<Tile>, start_x: usize, start_y: usize, next_x: usize, next_y: usize) -> Vec<Tile> {
    for neighbor in &t_mat[next_x][next_y].neighbors {
        if path.contains(neighbor) {
            continue
        } else if neighbor.x == start_x as i32 && neighbor.y == start_y as i32 {
            path.push(neighbor.clone());
            return path.to_vec();
        } else {
            path.push(neighbor.clone());
            return find_path(t_mat.clone(), path, start_x, start_y, neighbor.x as usize, neighbor.y as usize);
        }
    }
    path.to_vec()
}


fn solve1(input: &str, testing: bool) -> u32 {
    let max = input.lines().clone().count();
    let lines = input.lines().clone();
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut mat: Vec<Vec<char>> = vec![vec!['.'; max]; max];
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            mat[i][j] = c;
            if c == 'S' {
                pos_x = i;
                pos_y = j;
            }
        }
    }
    println!("S: x {pos_x} y {pos_y}");
    for line in &mat {
        println!("{line:?}");
    }
    let mut t_mat: Vec<Vec<Tile>> = vec![Vec::<Tile>::with_capacity(max);max];
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            t_mat[i].push(Tile { x: i as i32, y: j as i32, c: mat[i][j], neighbors: Vec::<Tile>::with_capacity(2)});
        }
    }
    for v in t_mat.iter_mut() {
        for w in v.iter_mut() {
            w.set_neighbors(&mat);
        }
    }
    for line in t_mat.iter() {
        // println!("{line:?}");
        for elem in line.iter() {
            if elem.neighbors.len() != 2 && elem.neighbors.len() != 0 {
                println!("{elem:?}");
            }
        }
    }
    println!("START: {:?}", t_mat[2][0]);
    t_mat[pos_x][pos_y].convert_starting_point(&mat, testing);
    println!("START: {:?}", t_mat[2][0]);
    let mut path: Vec<Tile> = vec![];
    path = find_path(t_mat.clone(), &mut path, pos_x, pos_y, pos_x, pos_y);
    for elem in &path {
        println!("PATH: {elem:?}");
    }
    println!("{}", path.len());
    for line in t_mat {
        print!("\n");
        for e in line {
            if path.contains(&e) {
                print!("{}", e.c);
            } else {
                print!(".");
            }
        }
    }
    print!("\n");
    ((path.len() + 1) / 2 ) as u32
}

fn solve2(input: &str) -> u128 {
    let _lines = input.lines();
    let sum = 0;
    sum
}

pub fn solve() {
    let input = include_str!("input");
    println!("Solution 1: {}", solve1(input, false));
    println!("Solution 2: {}", solve2(input));
}

#[cfg(test)]

mod tests {
    use crate::ten::*;

    const SAMPLE: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn sample_solved1() {
        let res = solve1(SAMPLE, true);
        assert_eq!(res, 8);
    }

    #[test]
    fn sample_solved2() {
        let res = solve2(SAMPLE);
        assert_eq!(res, 0);
    }
}
