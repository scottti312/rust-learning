fn main() {
    let mut a1 = ['a', 'g', 'e', 'k', 'z', 'x'];
    println!("Start = {:?}", a1);
    bubble_sort(&mut a1);
    println!("Result = {:?}", a1);
}

fn bubble_sort<T: Ord>(a1: &mut [T]) -> &mut [T] {
    for _i in 0..a1.len() - 1 {
        for j in 0..a1.len() - 1 {
            if a1[j + 1] < a1[j] {
                a1.swap(j + 1, j);
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
        let mut inital_array = [8, 7, 6, 5, 4, 3, 2, 1];
        let actual = bubble_sort(&mut inital_array);
        let expected = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(expected, actual);
    }

    #[test]
    fn random_test() {
        let mut initial_array = [5, 6, 2, 2, -6, 23, 3, 1];
        let actual = bubble_sort(&mut initial_array);
        let expected = [-6, 1, 2, 2, 3, 5, 6, 23];
        assert_eq!(actual, expected);
    }
}
