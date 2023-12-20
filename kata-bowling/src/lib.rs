// The game consists of 10 frames
// The score for the frame is the total number of pins knocked down, plus bonuses for strikes and spares.:
// - open frame: the score is the number of pins knocked down
// - A spare is when the player knocks down all 10 pins in two rolls. The bonus for that frame is the number of pins knocked down by the next roll.
// - A strike is when the player knocks down all 10 pins on his first roll. The frame is then completed with a single roll. The bonus for that frame is the value of the next two rolls.
// - In the tenth frame a player who rolls a spare or strike is allowed to roll the extra balls to complete the frame. However no more than three balls can be rolled in tenth frame.

struct Game {
    frames_left: u8,
    score: usize,
}

impl Game {
    fn new() -> Self {
        Game {
            score: 0,
            frames_left: 10,
        }
    }

    fn get_score(&self) -> usize {
        self.score
    }

    fn get_frames_left(&self) -> u8 {
        self.frames_left
    }

    fn roll(&mut self) -> Result<u8, &'static str> {
        if self.frames_left == 0 {
            return Err("Game is over");
        }
        if let Some(frames_left) = self.frames_left.checked_sub(1) {
            self.frames_left = frames_left;
            return Ok(self.get_frames_left());
        }
        Err("Game is over")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_game_state() {
        let game = Game::new();
        assert_eq!(0, game.get_score());
        assert_eq!(10, game.get_frames_left());
    }

    #[test]
    fn after_roll_frame_left_decreases() {
        let mut game = Game::new();
        let games_left = game.roll().unwrap();
        assert_eq!(9, games_left);
        assert_eq!(9, game.get_frames_left());
    }

    #[test]
    fn when_frame_left_is_zero_roll_returns_an_error() {
        let mut game = Game::new();
        for _ in 0..10 {
            let _ = game.roll();
        }
        assert_eq!(game.roll(), Err("Game is over"));
    }
}
