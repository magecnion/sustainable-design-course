// The game consists of 10 frames
// The score for the frame is the total number of pins knocked down, plus bonuses for strikes and spares.:
// - open frame: the score is the number of pins knocked down
// - A spare is when the player knocks down all 10 pins in two rolls. The bonus for that frame is the number of pins knocked down by the next roll.
// - A strike is when the player knocks down all 10 pins on his first roll. The frame is then completed with a single roll. The bonus for that frame is the value of the next two rolls.
// - In the tenth frame a player who rolls a spare or strike is allowed to roll the extra balls to complete the frame. However no more than three balls can be rolled in tenth frame.
// TODO test from outside so i can test how this game would be used
struct Game {
    frames: Vec<(Option<u8>, Option<u8>)>,
    extra_frames: [u8; 3],
    score: usize,
}

impl Game {
    fn new() -> Self {
        Game {
            frames: Vec::new(),
            extra_frames: [0; 3],
            score: 0,
        }
    }
    fn add_frame(&mut self, pins: u8) {
        match self.frames.last_mut() {
            // TODO refactor create fn if second open
            Some((first, second)) if *first != Some(10) && second.is_none() => {
                *second = Some(pins);
            }
            _ => {
                self.frames.push((Some(pins), None));
            }
        }
    }

    fn update_score(&mut self, pins: u8) {
        self.score += pins as usize;
        self.add_bonuses();
    }

    fn add_bonuses(&mut self) {
        let window_size = if self.frames.len() >= 3 { 3 } else { 2 };
        for window in self.frames.windows(window_size) {
            match window {
                [prev_prev_frame, prev_frame, frame] => {
                    if prev_prev_frame.0 == Some(10) {
                        // there was a strike two frames ago
                        self.score += frame.0.unwrap() as usize
                            + frame.1.unwrap_or(0) as usize
                            + prev_frame.0.unwrap() as usize
                            + prev_frame.1.unwrap_or(0) as usize;
                    }
                    if prev_frame.0.unwrap() + prev_frame.1.unwrap_or(0) == 10 {
                        // there was a spare in the previous frame
                        self.score += frame.0.unwrap() as usize + frame.1.unwrap_or(0) as usize
                    }
                }
                [prev_frame, frame] => {
                    if prev_frame.0.unwrap() + prev_frame.1.unwrap_or(0) == 10 {
                        // there was a spare in the previous frame
                        self.score += frame.0.unwrap() as usize + frame.1.unwrap_or(0) as usize
                    }
                }
                _ => {}
            }
        }
    }

    fn is_spare(&self) -> bool {
        // no es 10 pero es 10 con el anterior
        true
    }

    fn is_strike(&self) -> bool {
        true
        // es 10
    }

    fn is_last_frame(&self) -> bool {
        self.frames.len() == 10
    }

    fn roll(&mut self, pins: u8) -> Result<(), &'static str> {
        // TODO add AND there is no extra frame
        if self.is_last_frame() {
            return Err("Game is over");
        }

        self.add_frame(pins);
        self.update_score(pins);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO mod for the game
    // TODO use smt similar to beforeEach

    #[test]
    fn initial_game_state() {
        let game = Game::new();
        assert_eq!(0, game.score);
        assert_eq!(0, game.frames.len());
    }

    #[test]
    fn after_valid_sparse_command_roll_frame_left_decreases() {
        let mut game = Game::new();
        game.roll(5).unwrap();
        game.roll(5).unwrap();
        assert_eq!(1, game.frames.len());
    }

    #[test]
    fn after_valid_strike_command_roll_frame_left_decreases() {
        let mut game = Game::new();
        game.roll(10).unwrap();
        assert_eq!(1, game.frames.len());
    }

    #[test]
    fn after_valid_second_open_command_roll_frame_left_decreases() {
        let mut game = Game::new();
        game.roll(5).unwrap();
        game.roll(3).unwrap();
        assert_eq!(1, game.frames.len());
    }

    #[test]
    fn when_frame_left_is_zero_roll_returns_an_error() {
        let mut game = Game::new();
        for _ in 0..10 {
            let _ = game.roll(10);
        }
        assert_eq!(game.roll(10), Err("Game is over"));
    }

    #[test]
    fn after_open_frame_score_is_the_number_of_pins_knocked_down() {
        let mut game = Game::new();
        game.roll(5).unwrap();
        assert_eq!(5, game.score);
        assert_eq!(1, game.frames.len()); // TODO no se si deberia testear esto todo el rato
    }

    #[test]
    fn after_first_open_command_frames_left_does_not_decrease() {
        let mut game = Game::new();
        game.roll(5).unwrap();
    }

    #[test]
    fn spare_command_is_only_allow_after_open_command() {
        unimplemented!("TODO");
    }

    #[test]
    fn strike_command_is_not_allowed_after_open_command() {
        // TODO check enunciado
        let mut game = Game::new();
        game.roll(10).unwrap();
        game.roll(5).unwrap();
        assert_eq!(2, game.frames.len());
    }

    #[test]
    fn after_sparse_next_roll_score_is_the_number_of_pins_knocked_down_plus_the_next_roll() {
        let mut game = Game::new();
        game.roll(5).unwrap();
        game.roll(5).unwrap();
        game.roll(3).unwrap();
        assert_eq!(16, game.score);
    }

    #[test]
    fn table_test_driven() {
        unimplemented!("TODO")
    }
}
