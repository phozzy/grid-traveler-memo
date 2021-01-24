use std::collections::HashMap;
#[derive(PartialEq, Eq, Hash)]
struct Position {
    small: usize,
    big: usize,
}
impl Position {
    fn new(row: usize, column: usize) -> Position {
        if row < column {
            Position {small: row, big: column}
        } else {
            Position {small: column, big: row}
        }
    }
}
fn grid_traveler(row: usize, column: usize) -> usize {
    let mut memo: HashMap<Position, usize> = HashMap::new();
    grid_traveler_memo(row, column, &mut memo)
}
fn grid_traveler_memo(
    row: usize,
    column: usize,
    memo_l: &mut HashMap<Position, usize>
) -> usize {
    if row == 0 || column == 0 {
        0
    } else if row == 1 || column == 1 {
        1
    } else {
        let key = Position::new(row, column);
        match memo_l.get(&key) {
            Some(number) => *number,
            _ => {
                let down: usize = grid_traveler_memo(row - 1, column, memo_l);
                let right: usize = grid_traveler_memo(row, column - 1, memo_l);
                memo_l.insert(key, down + right);
                grid_traveler_memo(row, column, memo_l)
            },
        }
    }
}
fn main() {
    println!("{}", grid_traveler(1,1));
    println!("{}", grid_traveler(2,3));
    println!("{}", grid_traveler(3,2));
    println!("{}", grid_traveler(3,3));
    println!("{}", grid_traveler(18,18));
}
