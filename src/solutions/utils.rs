#[derive(Copy, Clone, Debug)]
pub struct Pos(pub usize, pub usize);

impl Pos {
    pub fn next(&self, clamp: &Pos) -> Vec<Pos> {
        let mut ans = vec![];

        let Pos(r, c) = *self;
        if r > 0 {
            ans.push(Pos(r - 1, c));
        }
        if c > 0 {
            ans.push(Pos(r, c - 1));
        }
        if r + 1 < clamp.0 {
            ans.push(Pos(r + 1, c));
        }
        if c + 1 < clamp.1 {
            ans.push(Pos(r, c + 1));
        }
        ans
    }
}
