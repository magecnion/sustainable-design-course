struct Game {
    frames: Vec<(u8, Option<u8>)>,
    score: usize,
}

impl Game {
    fn new() -> Self {
        Game {
            frames: Vec::new(),
            score: 0,
        }
    }
    fn add_frame(&mut self, pins: u8) -> Result<(), &'static str> {
        if self.frames.len() > 10 {
            self.frames.push((pins, None));
            return Ok(());
        }

        match self.frames.last_mut() {
            // TODO refactor create fn if second open
            Some((first, second)) if *first != 10 && second.is_none() => {
                if *first + pins > 10 {
                    return Err("Movement not allowed");
                }
                *second = Some(pins);
            }
            _ => {
                self.frames.push((pins, None));
            }
        }
        Ok(())
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
                    if prev_prev_frame.0 == 10 {
                        // there was a strike two frames ago
                        self.score += frame.0 as usize
                            + frame.1.unwrap_or(0) as usize
                            + prev_frame.0 as usize
                            + prev_frame.1.unwrap_or(0) as usize;
                    }
                    if prev_frame.0 + prev_frame.1.unwrap_or(0) == 10 {
                        // there was a spare in the previous frame
                        self.score += frame.0 as usize + frame.1.unwrap_or(0) as usize
                    }
                }
                [prev_frame, frame] => {
                    if prev_frame.0 + prev_frame.1.unwrap_or(0) == 10 {
                        // there was a spare in the previous frame
                        self.score += frame.0 as usize + frame.1.unwrap_or(0) as usize
                    }
                }
                _ => {}
            }
        }
    }

    fn roll(&mut self, pins: u8) -> Result<(), &'static str> {
        if self.game_is_over() {
            return Err("Game is over");
        }

        self.add_frame(pins)?;
        self.update_score(pins);

        Ok(())
    }

    fn is_last_done(&self) -> bool {
        self.frames.last().unwrap().1.is_some()
    }

    fn game_is_over(&self) -> bool {
        let last_frame = self.frames.last();
        if self.frames.len() == 10
            && !is_strike(last_frame)
            && !is_spare(last_frame)
            && self.is_last_done()
        {
            return true;
        } else if self.frames.len() == 11 && !is_strike(self.frames.get(9)) {
            return true;
        } else if self.frames.len() == 12 {
            return true;
        }
        self.frames.len() == 12
    }

    fn is_last_frame(&self) -> bool {
        self.frames.len() == 10
    }
}

fn is_spare(frame: Option<&(u8, Option<u8>)>) -> bool {
    !is_strike(frame) && frame.unwrap().0 + frame.unwrap().1.unwrap_or(0) == 10
}

fn is_strike(frame: Option<&(u8, Option<u8>)>) -> bool {
    frame.unwrap().0 == 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_game_state() {
        let game = Game::new();
        assert_eq!(0, game.score);
        assert_eq!(0, game.frames.len());
    }

    #[test]
    fn frames_are_increased_after_first_open() {
        let mut game = Game::new();
        assert_eq!(0, game.frames.len());
        game.roll(5).unwrap();
        assert_eq!(1, game.frames.len());
        assert_eq!(game.is_last_done(), false);
    }

    #[test]
    fn frames_are_not_increased_after_spare() {
        let mut game = Game::new();
        game.roll(5).unwrap();
        assert_eq!(1, game.frames.len());
        game.roll(5).unwrap();
        assert_eq!(1, game.frames.len());
    }

    #[test]
    fn frames_are_increased_after_strike() {
        let mut game = Game::new();
        assert_eq!(0, game.frames.len());
        game.roll(10).unwrap();
        assert_eq!(1, game.frames.len());
    }

    #[test]
    // TODO move this test to integration test outside
    fn second_attempt_cannot_sum_more_than_10() {
        let mut game = Game::new();
        game.roll(5).unwrap();
        assert_eq!(game.roll(6), Err("Movement not allowed"));
    }

    #[test]
    fn when_first_attempt_is_10_is_strike() {
        let mut game = Game::new();
        game.roll(10).unwrap();
        assert_eq!(is_strike(game.frames.last()), true);
        assert_eq!(is_spare(game.frames.last()), false);
    }

    #[test]
    fn when_second_attempt_sums_10_is_spare() {
        let mut game = Game::new();
        game.roll(1).unwrap();
        game.roll(9).unwrap();
        assert_eq!(is_strike(game.frames.last()), false);
        assert_eq!(is_spare(game.frames.last()), true);
    }

    #[test]
    // TODO move this test to integration test outside
    fn after_10_opens_game_is_over() {
        let mut game = Game::new();
        for _ in 0..10 {
            game.roll(0).unwrap();
            game.roll(0).unwrap();
        }
        assert_eq!(game.roll(10), Err("Game is over"));
    }

    #[test]
    // TODO move this test to integration test outside
    fn when_last_attempt_is_spare_you_gain_one_more_shot() {
        let mut game = Game::new();
        for _ in 0..9 {
            game.roll(0).unwrap();
            game.roll(0).unwrap();
        }
        game.roll(5).unwrap();
        game.roll(5).unwrap(); // spare // TODO wrap this func and the following as strike and spare as helper fn
        game.roll(5).unwrap(); // one more shot
        assert_eq!(game.roll(0), Err("Game is over"));
    }

    #[test]
    // TODO move this test to integration test outside
    fn when_last_attempt_is_strike_you_gain_two_more_shots() {
        let mut game = Game::new();
        for _ in 0..9 {
            game.roll(0).unwrap();
            game.roll(0).unwrap();
        }
        game.roll(10).unwrap(); // strike
        game.roll(1).unwrap(); // one more shot
        game.roll(5).unwrap(); // one more shot
        assert_eq!(game.roll(10), Err("Game is over"));
    }

    #[test]
    // TODO move this test to integration test outside
    fn after_open_frame_score_is_the_number_of_pins() {
        let mut game = Game::new();
        game.roll(5).unwrap();
        assert_eq!(5, game.score);
    }

    #[test]
    // TODO move this test to integration test outside
    fn after_sparse_next_roll_score_is_the_number_of_pins_knocked_down_plus_the_next_roll() {
        let mut game = Game::new();
        game.roll(5).unwrap();
        game.roll(5).unwrap();
        game.roll(3).unwrap();
        assert_eq!(16, game.score);
    }

    #[test]
    #[ignore]
    fn table_test_driven() {
        unimplemented!("TODO")
    }
}
