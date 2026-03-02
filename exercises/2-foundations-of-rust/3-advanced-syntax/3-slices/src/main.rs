// This a unfinished implementation of the well-known merge sort algorithm
//
// 1. Fix the language problems in the function merge
//
// 2. Finish the implementation of the function merge_sort
//
// 3. EXTRA: try changing the type from i32 into String everywhere; does your program still compile? What changes are necessary?

/// Merge two array slices (that have to be sorted) into a vector
fn merge(a: &[String], b: &[String]) -> Vec<String> {
    let mut dest = Vec::new();

    let mut a_idx = 0;
    let mut b_idx = 0;

    while a_idx < a.len() && b_idx < b.len() {
        if a[a_idx] <= b[b_idx] {
            dest.push(a[a_idx].clone());
            a_idx += 1
        } else {
            dest.push(b[b_idx].clone());
            b_idx += 1
        }
    }

    for elem in &a[a_idx..] {
        dest.push(elem.clone())
    }
    for elem in &b[b_idx..] {
        dest.push(elem.clone())
    }

    dest
}

/// Take an array slice, and sort into a freshly constructed vector using the above function
fn merge_sort(data: &[String]) -> Vec<String> {
    if data.len() > 1 {
        let mid = data.len() / 2;
        let first_half = &data[..mid];
        let second_half = &data[mid..];
        merge(&merge_sort(first_half), &merge_sort(second_half))
    } else {
        data.to_vec()
    }
}

/// Read a bunch of numbers from standard input into a Vec<i32>.
fn read_numbers() -> Vec<String> {
    use std::io;
    let mut result = Vec::new();
    for line in io::stdin().lines().flatten() {
        for word in line.split_whitespace() {
            result.push(word.parse().unwrap())
        }
    }

    result
}

// This wil keep reading numbers from stdin until stdin is closed.
// You can pipe in a text file or close the std in using ctrl+D on linux or the equivalent on your platform.
// You can also just run `cargo test` to finish this exercise.
fn main() {
    let input = read_numbers();
    println!("Data to be sorted:");
    println!("{input:?}");

    let sorted_input = merge_sort(&input);
    println!("Sorted data:");
    println!("{sorted_input:?}");
}

// you can run these automatic tests by typing 'cargo test'
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_strings() {
        assert_eq!(
            merge_sort(&["cherry".to_string(), "apple".to_string(), "banana".to_string()]),
            vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()]
        );

        assert_eq!(
            merge_sort(&["5".to_string(), "47".to_string(), "100".to_string()]),
            vec!["100".to_string(), "47".to_string(), "5".to_string()]
        );
    }
}
