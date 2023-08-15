//Manage a game player's High Score list.
// Your task is to build a high-score component of the classic Frogger game, one of the highest selling
// and addictive games of all time, and a classic of the arcade era. Your task is to write methods that
// return the highest score from the list, the last added score and the three highest scores.
// Consider retaining a reference to scores in the struct - copying is not necessary. You will require some lifetime annotations, though.
#[derive(Debug)]
pub struct HighScores{
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let scores = scores.to_vec();
        HighScores{scores}
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
        //&self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        let mut s = self.scores.clone();
        s.pop()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut s = self.scores.clone();
        s.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut s = self.scores.clone();
        s.sort();
        s.reverse();
        let slice = &s[0..3];
        let mut v = Vec::new();
        slice.iter().for_each(|x| v.push(*x));
        v
    }
}


fn main() {
    println!("Hello, world!");
}
