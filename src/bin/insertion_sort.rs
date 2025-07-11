/// 삽입정렬 (오름차순, 제자리)
fn insertion_sort(data: &mut [i32]) {
    for i in 1..data.len() {
        let key = data[i];
        let mut j = i;
        while j > 0 && data[j - 1] > key {
            data[j] = data[j - 1];
            j -= 1;
        }
        data[j] = key;
    }
}

fn main() {
    let mut vec = vec![5, 2, 9, 1, 5, 6];
    println!("정렬 전  : {vec:?}");

    insertion_sort(&mut vec);

    println!("정렬 후  : {vec:?}");
}
