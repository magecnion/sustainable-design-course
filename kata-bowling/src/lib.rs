// The game consists of 10 frames
// The score for the frame is the total number of pins knocked down, plus bonuses for strikes and spares.:
// - open frame: the score is the number of pins knocked down
// - A spare is when the player knocks down all 10 pins in two rolls. The bonus for that frame is the number of pins knocked down by the next roll.
// - A strike is when the player knocks down all 10 pins on his first roll. The frame is then completed with a single roll. The bonus for that frame is the value of the next two rolls.
// - In the tenth frame a player who rolls a spare or strike is allowed to roll the extra balls to complete the frame. However no more than three balls can be rolled in tenth frame.

struct Game {
    score: usize,
}

impl Game {
    fn new() -> Self {
        Game { score: 0 }
    }

    fn get_score(&self) -> usize {
        self.score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_initial_score() {
        let game = Game::new();
        assert_eq!(0, game.get_score());
    }
}
