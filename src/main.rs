use std::iter::repeat;

use itertools::Itertools;

const MAX_ITERATIONS: i32 = 15;

fn main() {
    // let column_values: Vec<Vec<i32>> = vec![
    //     vec![5], vec![4, 3], vec![5, 2], vec![7, 2], vec![7, 2, 1], vec![1, 10, 2], vec![1, 2, 7, 1], vec![1, 1, 1, 7, 1], vec![1, 8], vec![1, 7], vec![2, 4], vec![1, 6], vec![1, 7], vec![9], vec![5],
    // ];
    // let rows_values: Vec<Vec<i32>> = vec![
    //     vec![5], vec![2, 2], vec![4, 1, 1, 1], vec![6, 1], vec![7, 1], vec![6, 2], vec![8, 3], vec![1, 6, 3], vec![1, 5, 4], vec![2, 6, 4], vec![1, 10], vec![2, 9], vec![2, 7], vec![3, 4], vec![5],
    // ];
    // let column_values: Vec<Vec<i32>> = vec![
    //     vec![2, 3, 4], vec![2, 4, 6], vec![2, 4, 8], vec![3, 4, 1, 6], vec![2, 4, 5, 2], vec![5, 7, 1], vec![2, 3, 2, 2], vec![1, 3, 2, 2], vec![4, 4], vec![2], vec![4, 4], vec![1, 2, 6], vec![3, 3, 8], vec![4, 4, 1, 6], vec![5, 8, 1], vec![2, 2, 4, 1], vec![2, 3, 2, 2], vec![2, 2, 2, 2], vec![3, 2, 4], vec![5],
    // ];
    // let rows_values: Vec<Vec<i32>> = vec![
    //     vec![4], vec![3, 11], vec![9, 3, 3], vec![3, 2, 1, 1, 3, 1, 2], vec![2, 2, 2, 2, 4, 1], vec![1, 4, 1, 2, 5], vec![4, 1, 1, 2], vec![5, 1, 2], vec![4, 1, 1], vec![2, 2, 1], vec![1, 2], vec![1, 1], vec![5, 5], vec![2, 4, 2, 4], vec![6, 2, 6, 2], vec![6, 1, 6, 1], vec![5, 1, 5, 1], vec![4, 2, 5, 2], vec![4, 2, 3, 2], vec![5, 5],
    // ];

    let column_values: Vec<Vec<i32>> = vec![vec![1],vec![3],vec![5],vec![4],vec![2,2,3],vec![3,1,6],vec![7,1,3,2],vec![6,4,2,2],vec![7,12],vec![9,13],vec![8,14],vec![7,15],vec![23],vec![7,3,1,8],vec![5,4,2,7],vec![4,3,3,1,3],vec![6,2,2,7],vec![5,2,2,10],vec![5,2,11],vec![7,12],vec![11,1,7],vec![8,1,4],vec![4,2,1],vec![1,1],vec![3]];
    let rows_values: Vec<Vec<i32>> = vec![vec![1,1],vec![1,2,2],vec![1,10,1],vec![15],vec![16],vec![19],vec![10,5],vec![8,4,4],vec![17],vec![3,4,4],vec![4,2,2],vec![3,2,3],vec![6,2,2],vec![5,2,2,1],vec![6,1,8],vec![7,1,3,1],vec![1,6,5],vec![1,8,5],vec![2,1,7,6],vec![1,2,14],vec![2,1,8,6],vec![2,10,6],vec![2,3,13],vec![2,2,9,3],vec![3,2,9]];

    solve(rows_values, column_values)
}

fn print_grid(grid: &Vec<Vec<i32>>) {
    for x in grid {
        for &y in x.iter() {
            print!("{} ", if y == 1 { "1" } else { if y == -1 { "X" } else { "?" } })
        }
        println!()
    }
}


// -1 empty, 0 unknown, 1 full
fn generate_possible_rows(groups: &Vec<i32>, n: i32) -> Vec<Vec<i32>> {
    let num_groups: i32 = groups.len() as i32;
    let empty_spaces: i32 = n - groups.iter().sum::<i32>() - num_groups + 1;
    let num_options = empty_spaces + num_groups;
    (0..num_options)
        .combinations(num_groups as usize)
        .map(|x| convert_to_row(x, num_options, &groups))
        .collect()
}

fn convert_to_row(option: Vec<i32>, num_options: i32, groups: &Vec<i32>) -> Vec<i32> {
    let mut row = Vec::with_capacity(num_options as usize);
    let mut options_iter = option.iter().peekable();
    let mut groups_iter = groups.iter().clone();

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

fn find_common_values_with_constraints(possible_slots: &Vec<Vec<i32>>, slot_solved_values: &Vec<i32>) -> Vec<i32> {
    let z = possible_slots
        .iter()
        .filter(|x| (x.iter()).zip(slot_solved_values.iter()).all(|(&a, &b)| a == b || b == 0));

    let len = z.clone().count();

    z
        .map(|x| x.iter().map(|&y| (y + 1) / 2))
        .map(|x| x.collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>()
        .into_iter()
        .reduce(|x: Vec<i32>, y: Vec<i32>| x.iter().zip(y.iter()).map(|(a, b)| a + b).collect::<Vec<i32>>()).expect("invalid puzzle")
        .iter()
        .map(|&x| if x == len as i32 { 1 } else { if x == 0 as i32 { -1 } else { 0 } })
        .collect()
}


fn solve(rows_values: Vec<Vec<i32>>, column_values: Vec<Vec<i32>>) {
    assert_eq!(rows_values.len(), column_values.len());
    let n = rows_values.len() as i32;
    let possible_rows_vec: Vec<Vec<Vec<i32>>> = rows_values.iter().map(|x| generate_possible_rows(&x, n)).collect();
    let possible_column_vec: Vec<Vec<Vec<i32>>> = column_values.iter().map(|x| generate_possible_rows(&x, n)).collect();


    let mut side_possible_slots_a = &possible_rows_vec;
    let mut side_possible_slots_b = &possible_column_vec;

    let mut old_transposed_grid = vec![vec![0; n as usize]; n as usize];

    for k in 0..MAX_ITERATIONS {
        let mut new_transposed_grid = vec![Vec::<i32>::new(); n as usize];
        for (possible_slots, slot_solved_values) in (side_possible_slots_a).iter().zip(old_transposed_grid.iter()) {
            for (i, &v) in find_common_values_with_constraints(possible_slots, slot_solved_values).iter().enumerate() {
                new_transposed_grid[i].push(v);
            }
        }
        old_transposed_grid = new_transposed_grid;
        (side_possible_slots_a, side_possible_slots_b) = (side_possible_slots_b, side_possible_slots_a);
        println!("{}", k);
        print_grid(&old_transposed_grid);

        if old_transposed_grid.iter().all(|x| x.iter().all(|&y| y != 0)) {
            println!("done in {} iterations",k);
            break;
        }
    }

    print_grid(&old_transposed_grid);
}