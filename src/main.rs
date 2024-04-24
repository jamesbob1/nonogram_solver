use itertools::Itertools;


fn main() {
    let column_values: Vec<Vec<i32>> = vec![
        vec![5], vec![4, 3], vec![5, 2], vec![7, 2], vec![7, 2, 1], vec![1, 10, 2], vec![1, 2, 7, 1], vec![1, 1, 1, 7, 1], vec![1, 8], vec![1, 7], vec![2, 4], vec![1, 6], vec![1, 7], vec![9], vec![5],
    ];
    let rows_values: Vec<Vec<i32>> = vec![
        vec![5], vec![2, 2], vec![4, 1, 1, 1], vec![6, 1], vec![7, 1], vec![6, 2], vec![8, 3], vec![1, 6, 3], vec![1, 5, 4], vec![2, 6, 4], vec![1, 10], vec![2, 9], vec![2, 7], vec![3, 4], vec![5],
    ];
    solve(rows_values, column_values)
    // let possible_slots = vec![vec![1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1], vec![1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1], vec![1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1], vec![1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1], vec![1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1], vec![1, -1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1], vec![1, -1, -1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1], vec![-1, 1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1], vec![-1, 1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1], vec![-1, 1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1], vec![-1, 1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1], vec![-1, 1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1], vec![-1, 1, -1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1], vec![-1, -1, 1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1], vec![-1, -1, 1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1], vec![-1, -1, 1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1], vec![-1, -1, 1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1], vec![-1, -1, 1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1], vec![-1, -1, -1, 1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1], vec![-1, -1, -1, 1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1], vec![-1, -1, -1, 1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1], vec![-1, -1, -1, 1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1], vec![-1, -1, -1, -1, 1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1], vec![-1, -1, -1, -1, 1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1], vec![-1, -1, -1, -1, 1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1], vec![-1, -1, -1, -1, -1, 1, -1, 1, 1, 1, 1, 1, 1, 1, -1], vec![-1, -1, -1, -1, -1, 1, -1, -1, 1, 1, 1, 1, 1, 1, 1], vec![-1, -1, -1, -1, -1, -1, 1, -1, 1, 1, 1, 1, 1, 1, 1]];
    // let slot_solved_values = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 1, 1, 1, 0, 0];
    // for a in possible_slots{
    //     println!("{:?} {:?}", a, slot_solved_values);
    // }
    // let z:Vec<Vec<i32>> = possible_slots.into_iter().filter(|x|x.into_iter().zip(slot_solved_values.clone().into_iter()).all(|(&a,b)|cond(a, b))).collect();
    // let optional_rows = generate_possible_rows(&vec![2,6,4],15);
    // println!("{:?}", optional_rows);
    // let len = optional_rows.clone().len();
    // let zs:Vec<Vec<i32>> = optional_rows.clone().into_iter().map(|x|x.into_iter().map(|y|(y+1)/2).collect()).collect();
    //
    // let za:Vec<i32> = zs.into_iter()
    //     .reduce(|x, y| x.iter().zip(y.iter()).map(|(&a, &b)| a + b).collect()).unwrap();
    // println!("{:?}", za);
    //
    // let zb:Vec<i32> = za.into_iter().map(|x| if x == len as i32 { 1 } else { if x == 0 as i32 { -1 } else { 0 } }).collect();
    // println!("{:?}", zb)
}

fn print_grid(grid: Vec<i32>, n: usize){
    for i in 0..grid.len() {
        if i%n==0 {println!() }
        print!("{} ", grid[i])
    }

}
fn print_grid2(grid: Vec<Vec<i32>>){
    for x in grid {
        for y in x.iter(){
            print!("{} ", if *y == 1 {"1"} else {if *y==-1 {"X"} else {"?"}})
        }
        println!()
    }
}
fn find_common_values(optional_rows: Vec<Vec<i32>>) ->Vec<i32> {
    let len = optional_rows.clone().len();
    optional_rows.clone()
        .into_iter().map(|x|x.into_iter().map(|y|(y+1)/2).collect()).collect::<Vec<Vec<i32>>>().into_iter()
        .reduce(|x, y| x.iter().zip(y.iter()).map(|(&a, &b)| a + b).collect()).unwrap()
        .into_iter().map(|x| if x == len as i32 { 1 } else { if x == 0 as i32 { -1 } else { 0 } }).collect()
}

// -1 empty, 0 unknown, 1 full
fn generate_possible_rows(groups: &Vec<i32>, n:i32) -> Vec<Vec<i32>>{
    let num_groups: i32  = groups.len() as i32;
    let empty_spaces: i32 = n - groups.iter().sum::<i32>() - num_groups + 1;
    let num_options = empty_spaces + num_groups;
    let it = (0..num_options).combinations(num_groups as usize);
    it.map(|x|convert_to_row(x,num_options, &groups)).collect()
}

fn convert_to_row(option:Vec<i32>, num_options:i32, groups: &Vec<i32>) -> Vec<i32>{
    let mut row = Vec::new();
    let mut it = option.into_iter().peekable();
    let mut groups_iter = groups.clone().into_iter();

    for i in 0..num_options {
        let v = it.peek();
        if v.is_some() && *v.unwrap() == i {
            row.extend(vec![1; groups_iter.next().unwrap() as usize] );
            it.next();
        }
        if i != num_options - 1 {
            row.push(-1);
        }
    }
    row
}

fn cond(a:i32, b:i32)->bool{
    a == b || b == 0
}

fn find_common_values_with_constraints(possible_slots: Vec<Vec<i32>>, slot_solved_values: Vec<i32>) -> Vec<i32> {
    let z:Vec<Vec<i32>> = possible_slots.into_iter()
                           .filter(|x|x.into_iter().zip(slot_solved_values.clone().into_iter()).all(|(&a,b)|cond(a, b)))
        .collect();
    find_common_values(z)
}

fn solve(
    rows_values: Vec<Vec<i32>>,
    column_values: Vec<Vec<i32>>,
) {
    assert_eq!(rows_values.len(), column_values.len());
    let n = rows_values.len() as i32;

    let possible_rows_vec:Vec<Vec<Vec<i32>>> = rows_values.iter().map(|x|generate_possible_rows(&x, n)).collect();
    let possible_column_vec:Vec<Vec<Vec<i32>>> = column_values.iter().map(|x|generate_possible_rows(&x, n)).collect();

    let mut side_possible_slots_a = &possible_rows_vec;
    let mut side_possible_slots_b = &possible_column_vec;

    let mut old_transposed_grid = vec![vec![0;n as usize]; n as usize];

    for k in 0..10 {
        let mut new_transposed_grid = vec![Vec::<i32>::new(); n as usize];
        for (possible_slots, slot_solved_values) in (side_possible_slots_a).into_iter().zip(old_transposed_grid.into_iter()) {
            for (i, v) in find_common_values_with_constraints(possible_slots.clone(), slot_solved_values).into_iter().enumerate() {
                new_transposed_grid[i].push(v);
            }
        }
        old_transposed_grid = new_transposed_grid;
        (side_possible_slots_a, side_possible_slots_b) = (side_possible_slots_b, side_possible_slots_a);
        println!("{}",k);
    }

    print_grid2(old_transposed_grid);


    // println!("{:?}", transposed_grid);
    // let mut grid: Vec<i32> = Vec::new();
    // for x in possible_rows_vec {
    //     grid.extend(find_common_values(x));
    // }
    //
    // print_grid(grid,n as usize)
}
