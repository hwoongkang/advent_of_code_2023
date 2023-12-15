# 열다섯째 날

https://adventofcode.com/2023/day/15

`u8`의 유틸성 덕분에 쉽게 풀었습니다.

## Part 1

문자열을 0-255 사이의 값으로 바꿔야 합니다.

ASCII 값을 더하고, 17을 곱한 다음, 256으로 나눈 나머지를 구해줘야 하는데,

`u8::wrapping_add`와 `u8::wrapping_mul`을 사용하면 쉽게 구할 수 있습니다.

## Part 2

HashMap과 비스무리한 걸 구현해야 합니다.

웹 개발에서, immutable state 관리하듯이, filter_map, map 쓰니까 상태 관리하기가 쉬웠습니다.

move / clone 관련하여 조금 헷갈렸는데, `into_iter()` 가 아니라 `drain(..)` 쓰는 게 트릭이었습니다.
