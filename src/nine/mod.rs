fn solve1(input: &str) -> i32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let dataset: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        let max_length = dataset.len();
        let mut mat = vec![vec![1337_i32;max_length];max_length];
        for (i, date) in dataset.iter().enumerate() {
            mat[0][i] = *date;
        }
        for i in 1..max_length {
            for j in 0..max_length-i {
                mat[i][j] = mat[i-1][j+1] - mat[i-1][j];
            }
            let mut all_0 = true;
            for j in 0..max_length-i {
                if mat[i][j] != 0 {
                    all_0 = false;
                    break
                }
            }
            if all_0 {
                mat[i][max_length-i] = 0;
                for k in (0..i).rev() {
                    if max_length-k == max_length {
                        sum += mat[k+1][max_length-k-1] + mat[k][max_length-k-1];
                    } else {
                        mat[k][max_length-k] = mat[k+1][max_length-k-1] + mat[k][max_length-k-1]
                    }
                }
                break;
            }
        }
    }
    sum
}

fn solve2(input: &str) -> i32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let dataset: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        let max_length = dataset.len();
        let mut mat = vec![vec![1337_i32;max_length];max_length];
        for (i, date) in dataset.iter().enumerate() {
            mat[0][i] = *date;
        }
        for i in 1..max_length {
            for j in 0..max_length-i {
                mat[i][j] = mat[i-1][j+1] - mat[i-1][j];
            }
            let mut all_0 = true;
            for j in 0..max_length-i {
                if mat[i][j] != 0 {
                    all_0 = false;
                    break
                }
            }
            if all_0 {
                mat[i].insert(0,0);
                for k in (0..i).rev() {
                    let below = mat[k+1][0];
                    let right = mat[k][0];
                    mat[k].insert(0,right-below);
                }
                sum += mat[0][0];
                break;
            }
        }
    }
    sum
}

pub fn solve() {
    let input = include_str!("input");
    println!("Solution 1: {}", solve1(input));
    println!("Solution 2: {}", solve2(input));
}

#[cfg(test)]

mod tests {
    use crate::nine::*;

    const SAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn sample_solved1() {
        let res = solve1(SAMPLE);
        assert_eq!(res, 114);
    }

    #[test]
    fn sample_solved2() {
        let res = solve2(SAMPLE);
        assert_eq!(res, 2);
    }
}
