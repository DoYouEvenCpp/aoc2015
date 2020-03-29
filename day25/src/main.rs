
fn get_cell_number(x: usize, y: usize) -> usize {
    let mut pos = (1,1);
    let mut counter = 1;

    loop {
        if pos == (x, y) {
            break;
        }
        counter += 1;
        if pos.0 == 1 {
            pos.0 = pos.1 + 1;
            pos.1 = 1;
        } else {
            pos.0 -= 1;
            pos.1 += 1;
        }
    }
    counter
}

fn main() {
    let cell_number = get_cell_number(2981, 3075);
    let mut val: usize = 20151125;
    for _ in 1..cell_number {
        val *= 252533;
        val %= 33554393;
    }
    println!("{}", val);
}

#[cfg(test)]
mod day25 {
    use super::*;

    #[test]
    fn test_get_cell_number() {
        assert_eq!(1, get_cell_number(1,1));
        assert_eq!(2, get_cell_number(2,1));
        assert_eq!(3, get_cell_number(1,2));
        assert_eq!(4, get_cell_number(3,1));
        assert_eq!(5, get_cell_number(2,2));
        assert_eq!(6, get_cell_number(1,3));
        assert_eq!(21, get_cell_number(1,6));
    }
}
