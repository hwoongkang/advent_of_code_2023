# 다섯째 날

https://adventofcode.com/2023/day/5

[작년의 4번 문제](https://github.com/hwoongkang/adventofcode2022/pull/4)와 비슷하게, Range를 사용해야 합니다.

슬슬 어려워지네요.

## Part 1

"번역 방식"이 Range로 주어집니다.

Source: 50, Destination: 52, Range: 48로 주어지면,
(50..98) => (52..100)으로 번역하는 식입니다.

Source range에 해당하지 않는 숫자(위의 예에서 49)가 주어지면 번역하지 않고 그대로 49를 출력합니다.

처음에는 그냥 참조 Array를 만드려고 했는데, Input의 숫자 범위가 거의 2^64는 되겠더라구요.

그래서 Range 검사하는 로직으로 짰습니다.

## Part 2

여기서 이제 Range를 써야 합니다.

위의 예시에서는 번역 대상 숫자를 49만 넣었지만, 이제는 번역 대상 숫자도 40..50 뭐 이런 식으로 들어갈 수 있다는 뜻입니다.

따라서, Source Range에 어느 정도의 숫자가 걸리는지 판단해서, 걸쳐진 Range는 미리 번역하고, 안 걸쳐진 Range는 다음 predicate에 넣어보는 식으로 연속 거름망을 사용했습니다.

즉 위의 예시에서 source: 50, destination: 52, range: 48인데 input: 40..80 이었다면,

source range에 걸리는 50..80은 52..82로 미리 번역하고, 남은 40..50은 다음 predicate을 위해 남겨놓는 방식으로 풀었습니다.
