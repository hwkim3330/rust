/// 힙 정렬 (Heap Sort) 구현 (정수형 i32 전용, 오름차순 정렬)
///
/// 각 단계별로 배열 상태를 출력하여 내부 동작을 시각화합니다.

/// 범위 `[0..n)`에서 `i` 인덱스를 루트로 하는 서브트리를 최대 힙으로 조정
fn heapify(data: &mut [i32], n: usize, i: usize) {
    let mut largest = i;
    let left  = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && data[left] > data[largest] {
        largest = left;
    }
    if right < n && data[right] > data[largest] {
        largest = right;
    }
    if largest != i {
        data.swap(i, largest);
        heapify(data, n, largest);
    }
}

/// 힙 정렬 함수 (단계별 출력 포함)
fn heap_sort(data: &mut [i32]) {
    let n = data.len();

    println!("▶ 배열로 최대 힙 구성 시작");
    // 1) 최대 힙 빌드
    for i in (0..n / 2).rev() {
        heapify(data, n, i);
        println!("  heapify at i={}: {:?}", i, data);
    }
    println!("★ 최대 힙 완성:         {:?}\n", data);

    // 2) 힙 정렬: 루트(최댓값)와 끝값을 교환 후 heapify
    println!("▶ 힙 정렬 단계별 진행");
    for end in (1..n).rev() {
        println!("  swap data[0]={} <-> data[{}]={}", data[0], end, data[end]);
        data.swap(0, end);
        println!("   ↳ swap 후: {:?}", data);

        heapify(data, end, 0);
        println!("   ↳ heapify up to {}: {:?}\n", end, data);
    }
}

fn main() {
    let mut vec = vec![5, 2, 9, 1, 5, 6];

    println!("▶ 정렬 시작 전: {:?}", vec);
    heap_sort(&mut vec);
    println!("✅ 최종 정렬 완료: {:?}", vec);
}
