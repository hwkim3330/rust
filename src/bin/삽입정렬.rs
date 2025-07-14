/// 삽입정렬 함수 (정수형 i32 전용, 오름차순 정렬)
fn insertion_sort(data: &mut [i32]) {
    for i in 1..data.len() {
        let key = data[i];     // 현재 삽입할 값
        let mut j = i;

        println!("[i = {}] 현재 값: {}", i, key);
        println!("  정렬 전 상태 : {:?}", data);

        // 앞쪽 요소가 현재 값보다 크면 뒤로 이동
        while j > 0 && data[j - 1] > key {
            println!(
                "  ↳ data[{}] = {} > {} → data[{}] = {}로 이동",
                j - 1, data[j - 1], key, j, data[j - 1]
            );
            data[j] = data[j - 1]; // 한 칸 뒤로 밀기
            j -= 1;
        }

        data[j] = key; // 알맞은 위치에 삽입
        println!("  → {} 삽입 완료 (index {})", key, j);
        println!("  정렬 후 상태 : {:?}\n", data);
    }
}

fn main() {
    let mut vec = vec![5, 2, 9, 1, 5, 6];

    println!("▶ 정렬 시작 전 : {:?}\n", vec);

    insertion_sort(&mut vec);

    println!("✅ 최종 정렬 완료 : {:?}", vec);
}
