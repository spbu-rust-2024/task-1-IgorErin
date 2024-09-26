use itertools::Itertools;

fn insertion_sort(v: &mut Vec<i32>) {
    v.sort();
}

fn main() {
    let cin = std::io::stdin();
    let mut s = String::new();
    cin.read_line(&mut s).unwrap();
    let mut values = s
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();

    insertion_sort(&mut values);

    println!("{}", values.iter().format(" "));
}
