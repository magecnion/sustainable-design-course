struct Game {
    frames: Vec<Frame>, // TODO check why option; maybe is better unwrap_or
}

struct Frame {
    first: u8,
    second: Option<u8>,
    score: u8,
}

impl Frame {
    fn is_spare(&self) -> bool {
        !self.is_strike() && self.first + self.second.unwrap_or(0) == 10
    }

    fn is_strike(&self) -> bool {
        self.first == 10
    }
}

impl Game {
    const MAX_SCORE_PER_FRAME: u8 = 10;
    const MAX_FRAMES: usize = 10;

    fn new() -> Self {
        Game { frames: vec![] }
    }
    fn add_frame(&mut self, pins: u8) {
        self.frames.push(Frame {
            first: pins,
            second: None,
            score: pins,
        });
    }

    fn is_last_frame_in_progress(&self) -> bool {
        match self.frames.last() {
            Some(frame) => frame.first != Game::MAX_SCORE_PER_FRAME && frame.second.is_none(),
            None => return false,
        }
    }

    fn update_frame(&mut self, pins: u8) -> Result<(), &'static str> {
        if self.frames.last().unwrap().first + pins > Game::MAX_SCORE_PER_FRAME {
            return Err("Movement not allowed");
        }
        self.frames.last_mut().unwrap().second = Some(pins);
        self.frames.last_mut().unwrap().score += pins;
        Ok(())
    }

    fn is_extra_frame(&self) -> bool {
        self.frames.len() > Game::MAX_FRAMES
    }

    fn calculate_score(&self) -> usize {
        let score: usize = self
            .frames
            .iter()
            .take(Game::MAX_FRAMES)
            .map(|frame| frame.score as usize)
            .sum();
        score + self.get_bonuses()
    }

    // TODO refactor
    fn get_bonuses(&self) -> usize {
        let mut bonus = 0;
        for i in 0..self.frames.len().min(Game::MAX_FRAMES) {
            let current_frame = &self.frames[i];
            let next_roll = self.frames.get(i + 1).map_or(0, |frame| frame.first);
            let next_next_roll = self.frames.get(i + 1).map_or(0, |frame| {
                frame
                    .second
                    .unwrap_or_else(|| self.frames.get(i + 2).map_or(0, |frame| frame.first))
            });
            if current_frame.is_strike() {
                bonus += next_roll as usize + next_next_roll as usize;
            } else if current_frame.is_spare() {
                bonus += next_roll as usize;
            }
        }
        bonus
    }

    fn roll(&mut self, pins: u8) -> Result<(), &'static str> {
        if self.game_is_over() {
            return Err("Game is over");
        }

        if !self.is_extra_frame() && self.is_last_frame_in_progress() {
            self.update_frame(pins)?;
        } else {
            self.add_frame(pins);
        }

        Ok(())
    }

    fn game_is_over(&self) -> bool {
        match self.frames.len() {
            Game::MAX_FRAMES => {
                let last_frame = self.frames.last().unwrap();
                !last_frame.is_strike()
                    && !last_frame.is_spare()
                    && !self.is_last_frame_in_progress()
            }
            first_extra_roll if first_extra_roll == Game::MAX_FRAMES + 1 => {
                !self.frames.get(Game::MAX_FRAMES - 1).unwrap().is_strike()
            }
            second_extra_roll if second_extra_roll == Game::MAX_FRAMES + 2 => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO move frames test to frame mod

    fn roll_many(game: &mut Game, pins: u8, times: u8) {
        for _ in 0..times {
            game.roll(pins).unwrap();
        }
    }

    fn roll_spare(game: &mut Game) {
        game.roll(5).unwrap();
        game.roll(5).unwrap();
    }

    fn roll_strike(game: &mut Game) {
        game.roll(10).unwrap();
    }

    #[test]
    fn initial_game_state() {
        let game = Game::new();
        assert_eq!(0, game.calculate_score());
        assert_eq!(0, game.frames.len());
    }

    #[test]
    fn frames_are_increased_after_first_open() {
        let mut game = Game::new();
        assert_eq!(0, game.frames.len());
        game.roll(5).unwrap();
        assert_eq!(1, game.frames.len());
        assert_eq!(game.is_last_frame_in_progress(), true);
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
        roll_strike(&mut game);
        assert_eq!(1, game.frames.len());
    }

    #[test]
    fn when_first_attempt_is_10_is_strike() {
        let mut game = Game::new();
        roll_strike(&mut game);
        assert_eq!(game.frames.last().unwrap().is_strike(), true);
        assert_eq!(game.frames.last().unwrap().is_spare(), false);
    }

    #[test]
    fn when_second_attempt_sums_10_is_spare() {
        let mut game = Game::new();
        roll_spare(&mut game);
        assert_eq!(game.frames.last().unwrap().is_strike(), false);
        assert_eq!(game.frames.last().unwrap().is_spare(), true);
    }

    // TODO move all de aqui para abajo a integration test
    #[test]
    fn second_attempt_cannot_sum_more_than_10() {
        let mut game = Game::new();
        game.roll(5).unwrap();
        assert_eq!(game.roll(6), Err("Movement not allowed"));
    }

    #[test]
    fn after_10_opens_game_is_over() {
        let mut game = Game::new();
        roll_many(&mut game, 0, 20);
        assert_eq!(game.roll(10), Err("Game is over"));
    }

    #[test]
    fn when_last_attempt_is_spare_you_gain_one_more_shot() {
        let mut game = Game::new();
        roll_many(&mut game, 0, 18);
        roll_spare(&mut game);
        game.roll(5).unwrap(); // one more shot
        assert_eq!(game.roll(0), Err("Game is over"));
    }

    #[test]
    fn when_last_attempt_is_strike_you_gain_two_more_shots() {
        let mut game = Game::new();
        roll_many(&mut game, 0, 18);
        roll_strike(&mut game);
        game.roll(1).unwrap(); // one more shot
        game.roll(5).unwrap(); // one more shot
        assert_eq!(game.roll(10), Err("Game is over"));
    }

    #[test]
    fn after_open_frame_score_is_the_number_of_pins() {
        let mut game = Game::new();
        game.roll(5).unwrap();
        assert_eq!(5, game.calculate_score());
    }

    #[test]
    fn calculates_score_for_a_given_gutter_game() {
        let mut game = Game::new();
        roll_many(&mut game, 0, 20);
        assert_eq!(0, game.calculate_score());
    }
    #[test]
    fn calculates_the_score_for_a_given_all_ones_game() {
        let mut game = Game::new();
        roll_many(&mut game, 1, 20);
        assert_eq!(20, game.calculate_score());
    }

    #[test]
    fn calculates_score_for_a_given_one_spare_and_extra_roll() {
        let mut game = Game::new();
        roll_spare(&mut game);
        game.roll(5).unwrap();
        assert_eq!(20, game.calculate_score());
    }

    #[test]
    fn calculates_the_score_for_a_given_one_strike_and_some_extra_rolls() {
        let mut game = Game::new();
        roll_strike(&mut game);
        game.roll(2).unwrap();
        game.roll(3).unwrap();
        assert_eq!(20, game.calculate_score());
    }

    #[test]
    fn calculates_the_score_for_a_given_perfect_game() {
        let mut game = Game::new();
        roll_many(&mut game, 10, 12);
        assert_eq!(300, game.calculate_score());
    }

    #[test]
    fn calculates_the_score_for_a_given_all_spares_game() {
        let mut game = Game::new();
        roll_many(&mut game, 5, 20);
        game.roll(10).unwrap();
        assert_eq!(155, game.calculate_score());
    }

    #[test]
    fn calculates_the_score_for_a_given_all_spares_game_different_pattern_2() {
        let mut game = Game::new();
        roll_many(&mut game, 5, 21);
        assert_eq!(150, game.calculate_score());
    }

    #[test]
    fn calculates_the_score_for_a_given_all_spares_game_different_pattern_1() {
        let mut game = Game::new();
        for _ in 0..10 {
            game.roll(8).unwrap();
            game.roll(2).unwrap();
        }
        game.roll(8).unwrap();
        assert_eq!(180, game.calculate_score());
    }

    #[test]
    #[ignore]
    fn table_test_driven() {
        unimplemented!("TODO")
    }
}
