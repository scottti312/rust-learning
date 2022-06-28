fn main() {
    let mut a1 = [5, 6, 2, 2, -6, 23, 3, 1];
    println!("Start = {:?}", a1);
    a1 = bubble_sort(a1);
    println!("Result = {:?}", a1);
}
fn swap(mut first: i32, mut second: i32) -> (i32, i32) {
    let temp = first;
    first = second;
    second = temp;
    (first, second)
}

fn bubble_sort(mut a1: [i32; 8]) -> [i32; 8] {
    for _i in 0..a1.len() - 1 {
        for j in 0..a1.len() - 1 {
            if a1[j + 1] < a1[j] {
                (a1[j], a1[j + 1]) = swap(a1[j], a1[j + 1]);
            }
        }
    }
    a1
}

#[cfg(test)]
mod tests {
    use crate::bubble_sort;

    #[test]
    fn descending_test() {
        let inital_array = [8, 7, 6, 5, 4, 3, 2, 1];
        let actual = bubble_sort(inital_array);
        let expected = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(expected, actual);
    }

    #[test]
    fn random_test() {
        let initial_array = [5, 6, 2, 2, -6, 23, 3, 1];
        let actual = bubble_sort(initial_array);
        let expected = [-6, 1, 2, 2, 3, 5, 6, 23];
        assert_eq!(actual, expected);
    }
}
