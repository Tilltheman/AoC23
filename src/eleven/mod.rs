use std::collections::HashSet;
fn expand(mut universe: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rows_to_insert_to: Vec<usize> = vec![];
    let mut cols_to_insert_not_to: Vec<usize> = vec![];
    let mut rows = universe.len();
    println!("rows: {rows}");
    let mut cols = universe[0].len();
    let mut elems_inserted = 0;
    for (r, row) in universe.iter().enumerate() {
        let mut found_hashes = false;
        for elem in row.iter() {
            if elem == &'#' {
                found_hashes = true;
                println!("Found hashes on r: {r}");
            }
        }
        if !found_hashes {
            rows_to_insert_to.push(r);
        }
    }
    for row in &universe[..] {
        for (i, (j, col)) in row.iter().zip(row.iter().rev()).enumerate() {
            if j == &'#' {
                cols_to_insert_not_to.push(i);
            }
        }
    }
    println!("Cols to insert not to: {:?}", cols_to_insert_not_to);
    let mut cols_to_insert_to: Vec<usize> = vec![];
    for i in (0..cols).rev() {
        println!("{}", i);
        if ! cols_to_insert_not_to.contains(&i) {
            cols_to_insert_to.push(i);
        }
    }
    println!("Cols to insert to: {:?}", cols_to_insert_to);
    // insert cols
    let len_cols = cols_to_insert_to.len();
    for col in cols_to_insert_to {
        for i in 0..rows {
            universe[i].insert(col, '.');
        }
    }

    println!("Rows to insert to: {:?}", rows_to_insert_to);
    for row in rows_to_insert_to.iter().rev() {
        universe.insert(*row, vec!['.';cols+len_cols]);
    }
    for row in &universe {
        println!("{:?}", row);
    }
    universe
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Galaxy {
    x: i32,
    y: i32,
}


impl Galaxy {
    fn shortest_path_to(&self, other: &Galaxy) -> i32{
        let x1 = self.x;
        let y1 = self.y;
        let x2 = other.x;
        let y2 = other.y;
        println!("{}", x1 -x2 );
        let count = (x1 - x2).abs() + (y1 -y2).abs();
        count
    }
}

fn solve1(input: &str) -> i32 {
    let lines = input.lines();
    let mut sum = 0;
    let amount_rows = lines.clone().count();
    let amount_cols = lines.clone().nth(0).unwrap().chars().count();
    println!("{amount_rows} {amount_cols}");
    let mut universe: Vec<Vec<char>> = vec![vec!['0'; amount_cols]; amount_rows];
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            universe[i][j] = c;
        }
    }
    // expand universe
    let universe = expand(universe.clone());
    let mut galaxies: Vec<Galaxy> = vec![];
    for (i, line) in universe.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c == &'#' {
                galaxies.push(Galaxy { x: i as i32, y: j as i32 });
            }
        }
    }
    let mut hashset = HashSet::new();
    for (i, el1) in galaxies.iter().enumerate() {
        for (_j, el2) in galaxies[i+1..].iter().enumerate() {
            hashset.insert((el1, el2));
        }
    }
    for element in hashset.iter() {
        sum += element.0.shortest_path_to(element.1);
    }
    sum
}

fn solve2(input: &str) -> u32 {
    0
}

pub fn solve() {
    let input = include_str!("input");
    println!("Solution 1: {}", solve1(input));
    println!("Solution 2: {}", solve2(input));
}

#[cfg(test)]
mod tests {
    use crate::eleven::*;

    const SAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    const SAMPLE_EXPANDED: &str = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

    #[test]
    fn one_solved() {
        let res = solve1(SAMPLE);
        assert_eq!(res, 374);
    }

    #[test]
    fn two_solved() {
        let res = solve2(SAMPLE);
        assert_eq!(res, 71503);
    }

    #[test]
    fn test_expansion_works() {
        let lines = SAMPLE.lines();
        let amount_rows = lines.clone().count();
        let amount_cols = lines.clone().nth(0).unwrap().chars().count();
        let mut universe: Vec<Vec<char>> = vec![vec!['0'; amount_cols]; amount_rows];
        for (i, line) in lines.enumerate() {
            for (j, c) in line.chars().enumerate() {
                universe[i][j] = c;
            }
        }
        // expand universe
        let universe = expand(universe);
        let lines = SAMPLE_EXPANDED.lines();
        let amount_rows = lines.clone().count();
        let amount_cols = lines.clone().nth(0).unwrap().chars().count();
        let mut universe2: Vec<Vec<char>> = vec![vec!['0'; amount_cols]; amount_rows];
        for (i, line) in lines.enumerate() {
            for (j, c) in line.chars().enumerate() {
                universe2[i][j] = c;
            }
        }
        assert_eq!(universe2, universe);
    }
}
