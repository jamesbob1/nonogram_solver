mod solver;

fn main() {
    // let column_values: Vec<Vec<i32>> = vec![vec![5], vec![4, 3], vec![5, 2], vec![7, 2], vec![7, 2, 1], vec![1, 10, 2], vec![1, 2, 7, 1], vec![1, 1, 1, 7, 1], vec![1, 8], vec![1, 7], vec![2, 4], vec![1, 6], vec![1, 7], vec![9], vec![5], ];
    // let rows_values: Vec<Vec<i32>> = vec![vec![5], vec![2, 2], vec![4, 1, 1, 1], vec![6, 1], vec![7, 1], vec![6, 2], vec![8, 3], vec![1, 6, 3], vec![1, 5, 4], vec![2, 6, 4], vec![1, 10], vec![2, 9], vec![2, 7], vec![3, 4], vec![5], ];
    // let column_values: Vec<Vec<i32>> = vec![vec![2, 3, 4], vec![2, 4, 6], vec![2, 4, 8], vec![3, 4, 1, 6], vec![2, 4, 5, 2], vec![5, 7, 1], vec![2, 3, 2, 2], vec![1, 3, 2, 2], vec![4, 4], vec![2], vec![4, 4], vec![1, 2, 6], vec![3, 3, 8], vec![4, 4, 1, 6], vec![5, 8, 1], vec![2, 2, 4, 1], vec![2, 3, 2, 2], vec![2, 2, 2, 2], vec![3, 2, 4], vec![5], ];
    // let rows_values: Vec<Vec<i32>> = vec![vec![4], vec![3, 11], vec![9, 3, 3], vec![3, 2, 1, 1, 3, 1, 2], vec![2, 2, 2, 2, 4, 1], vec![1, 4, 1, 2, 5], vec![4, 1, 1, 2], vec![5, 1, 2], vec![4, 1, 1], vec![2, 2, 1], vec![1, 2], vec![1, 1], vec![5, 5], vec![2, 4, 2, 4], vec![6, 2, 6, 2], vec![6, 1, 6, 1], vec![5, 1, 5, 1], vec![4, 2, 5, 2], vec![4, 2, 3, 2], vec![5, 5], ];
    // let column_values: Vec<Vec<i32>> = vec![vec![1],vec![3],vec![5],vec![4],vec![2,2,3],vec![3,1,6],vec![7,1,3,2],vec![6,4,2,2],vec![7,12],vec![9,13],vec![8,14],vec![7,15],vec![23],vec![7,3,1,8],vec![5,4,2,7],vec![4,3,3,1,3],vec![6,2,2,7],vec![5,2,2,10],vec![5,2,11],vec![7,12],vec![11,1,7],vec![8,1,4],vec![4,2,1],vec![1,1],vec![3]];
    // let rows_values: Vec<Vec<i32>> = vec![vec![1,1],vec![1,2,2],vec![1,10,1],vec![15],vec![16],vec![19],vec![10,5],vec![8,4,4],vec![17],vec![3,4,4],vec![4,2,2],vec![3,2,3],vec![6,2,2],vec![5,2,2,1],vec![6,1,8],vec![7,1,3,1],vec![1,6,5],vec![1,8,5],vec![2,1,7,6],vec![1,2,14],vec![2,1,8,6],vec![2,10,6],vec![2,3,13],vec![2,2,9,3],vec![3,2,9]];
    // let column_values: Vec<Vec<i32>> = vec![vec![1,1],vec![1,2],vec![1,1,3,1],vec![4,6],vec![3,3,3,2],vec![3,6,2],vec![2,2,1,3,1,2],vec![2,2,2,5,2],vec![2,2,2,3,2],vec![2,2,2,5],vec![2,1,2,4],vec![2,4,4],vec![3,4,5],vec![2,1,6,8],vec![2,1,12],vec![22],vec![23],vec![25],vec![21],vec![7,9],vec![3,9],vec![10],vec![9],vec![2,3],vec![4],vec![2],vec![1]];
    // let rows_values: Vec<Vec<i32>> = vec![vec![1],vec![1],vec![2],vec![1,3],vec![2,1,4],vec![3,6],vec![4,6],vec![3,1,5],vec![1,4,7],vec![3,3,6],vec![3,2,6],vec![3,1,5],vec![3,3,5],vec![12,3],vec![10,5],vec![11,2],vec![13],vec![12],vec![11],vec![11],vec![8,12],vec![20],vec![20],vec![1,1,13],vec![4,2,6],vec![4,2],vec![5,3],vec![3,3],vec![3],vec![3],vec![1]];
    let column_values: Vec<Vec<i32>> = vec![vec![2],vec![3],vec![3],vec![5],vec![2,2,1],vec![2,2,2],vec![2,2,3],vec![1,2,3],vec![2,2,3,1],vec![2,2,3,4],vec![2,2,4,3,5],vec![2,3,4,3,7],vec![2,3,6,7],vec![2,12,6],vec![2,13,5],vec![3,11,5],vec![4,13],vec![9,7],vec![8,7],vec![13],vec![10],vec![11],vec![9],vec![6],vec![3]];
    let rows_values: Vec<Vec<i32>> = vec![vec![6],vec![10],vec![2,3],vec![2,4,3],vec![2,8,3],vec![1,3,4,3],vec![3,3,2],vec![3,1,3,3],vec![2,2,7],vec![2,10],vec![1,11],vec![2,10],vec![1,5,3],vec![6,4],vec![12],vec![3,10],vec![2,10],vec![2,2,11],vec![1,3,9],vec![2,15],vec![1,9,3],vec![2,7,3],vec![2,7,2],vec![3,8,1]];
    print_grid(solver::solve(rows_values, column_values));
}

fn print_grid(grid: Vec<Vec<i32>>) {
    for x in grid {
        for &y in x.iter() {
            print!("{} ", if y == 1 { "1" } else { if y == -1 { "X" } else { "?" } })
        }
        println!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let column_values: Vec<Vec<i32>> = vec![vec![1],vec![3],vec![5],vec![4],vec![2,2,3],vec![3,1,6],vec![7,1,3,2],vec![6,4,2,2],vec![7,12],vec![9,13],vec![8,14],vec![7,15],vec![23],vec![7,3,1,8],vec![5,4,2,7],vec![4,3,3,1,3],vec![6,2,2,7],vec![5,2,2,10],vec![5,2,11],vec![7,12],vec![11,1,7],vec![8,1,4],vec![4,2,1],vec![1,1],vec![3]];
        let rows_values: Vec<Vec<i32>> = vec![vec![1,1],vec![1,2,2],vec![1,10,1],vec![15],vec![16],vec![19],vec![10,5],vec![8,4,4],vec![17],vec![3,4,4],vec![4,2,2],vec![3,2,3],vec![6,2,2],vec![5,2,2,1],vec![6,1,8],vec![7,1,3,1],vec![1,6,5],vec![1,8,5],vec![2,1,7,6],vec![1,2,14],vec![2,1,8,6],vec![2,10,6],vec![2,3,13],vec![2,2,9,3],vec![3,2,9]];
        let expected_result = vec![vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 1, -1, -1, 1, -1, -1, -1, -1, -1, -1, -1, -1], vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, 1, -1, -1, -1, 1, 1, -1, 1, 1, -1, -1, -1, -1, -1, -1, -1], vec![-1, -1, -1, -1, -1, -1, 1, -1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, 1, -1, -1, -1, -1], vec![-1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1], vec![-1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1], vec![-1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1], vec![-1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, 1, 1, 1, 1, 1, -1, -1], vec![-1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, 1, -1, 1, 1, 1, 1, -1, 1, 1, 1, 1, -1, -1], vec![-1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1], vec![-1, -1, -1, -1, -1, -1, -1, -1, 1, 1, 1, -1, 1, 1, 1, 1, -1, -1, 1, 1, 1, 1, -1, -1, -1], vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 1, 1, 1, 1, -1, 1, 1, -1, -1, 1, 1, -1, -1, -1], vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 1, 1, 1, -1, -1, -1, 1, 1, -1, -1, 1, 1, 1, -1, -1], vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, 1, 1, -1, 1, 1, -1], vec![-1, -1, -1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, -1, 1, 1, -1, -1, 1, 1, -1, -1, -1, -1, 1], vec![-1, -1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, -1, -1, 1, -1, 1, 1, 1, 1, 1, 1, 1, 1], vec![-1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1, 1, -1, 1, 1, 1, -1, -1, -1, -1, 1], vec![-1, -1, -1, -1, -1, 1, -1, 1, 1, 1, 1, 1, 1, -1, -1, -1, 1, 1, 1, 1, 1, -1, -1, -1, -1], vec![-1, -1, -1, -1, 1, -1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, 1, 1, 1, 1, 1, -1, -1, -1, -1], vec![-1, -1, -1, 1, 1, -1, 1, -1, 1, 1, 1, 1, 1, 1, 1, -1, 1, 1, 1, 1, 1, 1, -1, -1, -1], vec![-1, -1, -1, 1, -1, 1, 1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1], vec![-1, -1, 1, 1, -1, 1, -1, 1, 1, 1, 1, 1, 1, 1, 1, -1, 1, 1, 1, 1, 1, 1, -1, -1, -1], vec![-1, -1, 1, 1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, 1, 1, 1, 1, 1, 1, -1, -1, -1], vec![-1, 1, 1, -1, 1, 1, 1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1], vec![-1, 1, 1, -1, 1, 1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, 1, 1, 1, -1, -1, -1, -1, -1], vec![1, 1, 1, -1, 1, 1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1, -1, -1]];
        assert_eq!(solver::solve(rows_values, column_values), expected_result);
    }

    #[test]
    fn example_b() {
        let column_values: Vec<Vec<i32>> = vec![vec![2, 3, 4], vec![2, 4, 6], vec![2, 4, 8], vec![3, 4, 1, 6], vec![2, 4, 5, 2], vec![5, 7, 1], vec![2, 3, 2, 2], vec![1, 3, 2, 2], vec![4, 4], vec![2], vec![4, 4], vec![1, 2, 6], vec![3, 3, 8], vec![4, 4, 1, 6], vec![5, 8, 1], vec![2, 2, 4, 1], vec![2, 3, 2, 2], vec![2, 2, 2, 2], vec![3, 2, 4], vec![5], ];
        let rows_values: Vec<Vec<i32>> = vec![vec![4], vec![3, 11], vec![9, 3, 3], vec![3, 2, 1, 1, 3, 1, 2], vec![2, 2, 2, 2, 4, 1], vec![1, 4, 1, 2, 5], vec![4, 1, 1, 2], vec![5, 1, 2], vec![4, 1, 1], vec![2, 2, 1], vec![1, 2], vec![1, 1], vec![5, 5], vec![2, 4, 2, 4], vec![6, 2, 6, 2], vec![6, 1, 6, 1], vec![5, 1, 5, 1], vec![4, 2, 5, 2], vec![4, 2, 3, 2], vec![5, 5], ];
        let expected_result = vec![vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 1, 1, 1, 1, -1, -1, -1], vec![-1, -1, -1, 1, 1, 1, -1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1], vec![-1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, 1, 1, 1, -1, -1, 1, 1, 1], vec![-1, 1, 1, 1, -1, 1, 1, -1, 1, -1, 1, -1, 1, 1, 1, -1, 1, -1, 1, 1], vec![1, 1, -1, -1, 1, 1, -1, 1, 1, -1, 1, 1, -1, -1, 1, 1, 1, 1, -1, 1], vec![1, -1, 1, 1, 1, 1, -1, 1, -1, -1, -1, 1, 1, -1, -1, 1, 1, 1, 1, 1], vec![-1, 1, 1, 1, 1, -1, -1, 1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, 1, 1], vec![1, 1, 1, 1, 1, -1, 1, -1, -1, -1, -1, -1, 1, 1, -1, -1, -1, -1, -1, -1], vec![1, 1, 1, 1, -1, -1, 1, -1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, -1], vec![1, 1, -1, -1, -1, 1, 1, -1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, -1], vec![-1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, -1, -1, 1, 1, -1, -1, -1, -1, -1], vec![-1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1], vec![-1, -1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, -1, -1, -1], vec![-1, 1, 1, -1, 1, 1, 1, 1, -1, -1, -1, 1, 1, -1, 1, 1, 1, 1, -1, -1], vec![1, 1, 1, 1, 1, 1, -1, 1, 1, -1, 1, 1, 1, 1, 1, 1, -1, 1, 1, -1], vec![1, 1, 1, 1, 1, 1, -1, -1, 1, -1, 1, 1, 1, 1, 1, 1, -1, -1, 1, -1], vec![1, 1, 1, 1, 1, -1, -1, -1, 1, -1, 1, 1, 1, 1, 1, -1, -1, -1, 1, -1], vec![1, 1, 1, 1, -1, -1, -1, 1, 1, -1, 1, 1, 1, 1, 1, -1, -1, 1, 1, -1], vec![-1, 1, 1, 1, 1, -1, 1, 1, -1, -1, -1, 1, 1, 1, -1, -1, 1, 1, -1, -1], vec![-1, -1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, -1, -1, -1]];
        assert_eq!(solver::solve(rows_values, column_values), expected_result);
    }
}
