# Advent of Code 2023

벌써 2023도..

## Advent of Ocde

[대림절](https://en.wikipedia.org/wiki/Advent)은 기독교의 절기로, 크리스마스 4주 전부터 시작됩니다.  
매주 일요일에 촛불을 하나씩 켜다가, 크리스마스 당일에 가운데의 큰 양초를 점화하게 되는 설레는 절기입니다.

[Advent of Code](https://adventofcode.com/)는 이를 코딩 대회로 해석한 것인데요,  
12월 1일부터 매일 두 파트로 이루어진 문제가 하나씩 출제되고, 한 파트를 성공할 때마다 별을 하나 줍니다.  
크리스마스까지 모든 문제를 풀면 50개의 별을 다 모을 수 있어요!

[2022년](https://github.com/hwoongkang/adventofcode2022)에 이어 올해도 Rust로 풀 예정입니다.

## 구조

- Rust를 사용해 풀 예정입니다.

- Cargo가 필요합니다.

```Bash
curl https://sh.rustup.rs -sSf | sh
cargo run
```

- 각 날짜의 솔루션은 `src/days/day##.rs`에 있습니다.

- 입력은 일단 `Cargo.toml`과 같은 위계에 위치한 `input.txt`에서 받고 있습니다.  
  `Solution` trait 에서 입력 파일의 위치를 받고 있으니, 수정해서 사용할 수 있습니다.

- 솔루션에 관한 부가 설명은 PR description에 적을 예정입니다.
