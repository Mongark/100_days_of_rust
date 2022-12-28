fn bubble_sort(list: &Vec<i32>) -> Vec<i32> {
    let mut sorted: Vec<i32> = list.clone();

    let mut count: usize = 0;

    while count < list.len()-1 {
        if sorted[count] > sorted[count + 1] {
            sorted.swap(count, count + 1);
            count = 0;
        } else {
            count = count + 1;
        }
    }

    sorted
}

fn main() {
    let sample_values: Vec<i32> = vec![
        2,
        6,
        1,
        7,
        2,
        5,
    ];

    let sorted = bubble_sort(&sample_values);

    dbg!(sorted);
}
