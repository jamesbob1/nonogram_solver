use itertools::Itertools;
use std::iter::repeat;

const MAX_ITERATIONS: i32 = 20;

// -1 empty, 0 unknown, 1 full
fn generate_possible_rows(groups: &Vec<i32>, n: i32) -> Vec<Vec<i32>> {
    let num_groups: i32 = groups.len() as i32;
    let empty_spaces: i32 = n - groups.iter().sum::<i32>() - num_groups + 1;
    let num_options = empty_spaces + num_groups;
    (0..num_options)
        .combinations(num_groups as usize)
        .map(|x| convert_to_row(&x, num_options, &groups))
        .collect()
}

fn convert_to_row(option: &Vec<i32>, num_options: i32, groups: &Vec<i32>) -> Vec<i32> {
    let mut row = Vec::with_capacity(num_options as usize);
    let mut options_iter = option.iter().peekable();
    let mut groups_iter = groups.iter();

    for i in 0..num_options {
        if options_iter.peek() == Some(&&i) {
            row.extend(repeat(1).take((*groups_iter.next().unwrap()) as usize));
            options_iter.next();
        }
        if i != num_options - 1 {
            row.push(-1);
        }
    }
    row
}

fn find_common_values_with_constraints(
    possible_slots: &mut Vec<Vec<i32>>,
    slot_solved_values: &Vec<i32>,
) -> Vec<i32> {
    possible_slots.retain(|x| {
        x.iter()
            .zip(slot_solved_values.iter())
            .all(|(&a, &b)| a == b || b == 0)
    });

    let len = possible_slots.len();
    possible_slots
        .iter()
        .map(|x| x.iter().map(|&y| (y + 1) / 2).collect())
        .reduce(|x: Vec<i32>, y: Vec<i32>| {
            x.iter()
                .zip(y.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<i32>>()
        })
        .expect("invalid puzzle")
        .iter()
        .map(|&x| match x {
            x if x == len as i32 => 1,
            x if x == 0 => -1,
            _ => 0,
        })
        .collect()
}

pub fn solve(rows_values: Vec<Vec<i32>>, column_values: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // assert_eq!(rows_values.len(), column_values.len());
    let n = rows_values.len() as i32;
    let m = column_values.len() as i32;
    let mut possible_rows_vec: Vec<Vec<Vec<i32>>> = rows_values.iter().map(|x| generate_possible_rows(&x, m)).collect();
    let mut possible_column_vec: Vec<Vec<Vec<i32>>> = column_values.iter().map(|x| generate_possible_rows(&x, n)).collect();
    let mut grid = vec![vec![0; m as usize]; n as usize];

    for _ in 0..MAX_ITERATIONS {
        for (row, possible_rows) in grid.iter_mut().zip(possible_rows_vec.iter_mut()) {
            *row = find_common_values_with_constraints(possible_rows, &row)
        }

        if grid.iter().all(|x| x.iter().all(|&y| y != 0)) {
            break;
        }

        for (i, possible_columns) in (0..grid.len()).zip(possible_column_vec.iter_mut()) {
            let col = grid.iter().map(|z| z[i]).collect();
            let values = find_common_values_with_constraints(possible_columns, &col);
            for (x, v) in grid.iter_mut().map(|z| &mut z[i]).zip(values.iter()) {
                *x = *v;
            }
        }

        if grid.iter().all(|x| x.iter().all(|&y| y != 0)) {
            break;
        }
    }
    return grid;
}
