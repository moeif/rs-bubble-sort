fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let mut n = arr.len();
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }
        n -= 1;
    }
}

fn main() {
    let mut numbers = vec![10, 8, 2, 11, 9, 1];
    println!("Raw i32: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("Sorted i32: {:?}", numbers);

    let mut float_numbers = vec![1.5, 3.5, 87.1, 2.1, 6.9];
    println!("Raw float: {:?}", float_numbers);
    bubble_sort(&mut float_numbers);
    println!("Sorted float: {:?}", float_numbers);

    let mut words = vec!["fred", "tina", "tom", "jack"];
    println!("Raw words: {:?}", words);
    bubble_sort(&mut words);
    println!("Sorted string: {:?}", words);
}
