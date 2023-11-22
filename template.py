import os

for i in range(1, 26):
    day = str(i).zfill(2)
    struct = f"Day{day}"
    test = f"day{day}_tests"

    with open(os.path.join(".", "src", "days.rs"), 'a') as f:
        f.write(f"mod day{day};\n")
        f.write(f"pub use day{day}::{struct};\n")

    with open(os.path.join(".", "src", "days", f"day{day}.rs"), 'w') as f:
        f.write(
f"""
use super::Solution;

pub struct {struct};

impl Solution for {struct} {{
    fn test_input() -> String {{
        String::from("")
    }}

    fn solve_part_1(input: String) -> String {{
        input
    }}

    fn solve_part_2(input: String) -> String {{
        input
    }}
}}

#[cfg(test)]
mod {test} {{
    use super::*;

    #[test]
    fn test_part_1() {{
        let input = {struct}::test_input();
        let ans = {struct}::solve_part_1(input);
        assert_eq!(ans, "");
    }}

    #[test]
    fn test_part_2() {{
        let input = {struct}::test_input();
        let ans = {struct}::solve_part_2(input);
        assert_eq!(ans, "");
    }}
}}
"""
        )
    