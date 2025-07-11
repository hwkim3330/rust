```markdown
# Rust Playground 🦀

실전 학습용 **미니 Rust 예제 모음**입니다.  
각 예제는 **완전히 독립된 실행 파일**(바이너리)로, `src/bin/` 아래에 `main()`을 하나씩 갖습니다.  
복잡한 모듈·라이브러리 참조 없이 **파일만 복사해도** 바로 돌려볼 수 있도록 설계했습니다.

---

## 🔖 포함된 예제

| 바이너리 이름 | 설명 | 실행 예시 |
|---------------|------|-----------|
| `calculator`      | 사용자 입력으로 사칙연산을 수행하는 CLI 계산기 | `cargo run --bin calculator` |
| `insertion_sort`  | 삽입정렬(Insertion Sort) 알고리즘 데모         | `cargo run --bin insertion_sort` |

새 예제를 추가하려면 `src/bin/`에 `foo.rs`를 만들고 `fn main()`을 작성하면 끝!

---

## 🗂️ 디렉터리 구조

```

rust/
├── src/
│   └── bin/
│       ├── calculator.rs      # CLI 계산기
│       └── insertion\_sort.rs  # 삽입정렬 데모
├── Cargo.toml
└── README.md                  # ← 바로 이 파일

````

> `src/main.rs` 는 **필요하지 않습니다**.  
> (만약 공통 안내용으로 쓰고 싶다면 자유롭게 추가하세요.)

---

## 🚀 빠른 시작

### 1. 설치 요건
- **Rust** stable (1.79 이상 권장)  
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````

### 2. 클론 & 실행

```bash
git clone https://github.com/hwkim3330/rust.git
cd rust

# 계산기 실행
cargo run --bin calculator

# 삽입정렬 실행
cargo run --bin insertion_sort
```

### 3. 최적화 빌드

```bash
cargo build --release --bin calculator
```

생성된 실행 파일은 `target/release/`에 위치합니다.

---

## ✏️ 학습 포인트

* **입력 처리 & 에러 핸들링** (계산기)
* **소유권 & 문자열 파싱** (`String`, `&str`)
* **제자리(in-place) 정렬 알고리즘** 이해 (삽입정렬)

각 소스 파일에 **주석**으로 핵심 개념을 간결히 설명해 두었습니다.

---

## 🤝 Contributing

> 작은 실험용 레포이지만 PR·Issue 언제든 환영합니다!

1. 포크(Fork) 후 브랜치 생성
2. 개선 사항 커밋 → Pull Request
3. 설명 간단히 작성 → 리뷰 후 병합

---

## 🪪 라이선스

MIT License © 2025 [hwkim3330](https://github.com/hwkim3330)

```

> 필요한 섹션(예: **Tests**, **Roadmap**)은 프로젝트가 커짐에 따라 자유롭게 추가하시면 됩니다!
```
