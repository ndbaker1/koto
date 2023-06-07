//! library implementing core logic and data structures for Koto

pub type ID = &'static str;

struct GameState {
    team_health: u32,
    enemy_health: u32,
    board: Vec<Vec<char>>,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            team_health: 0,
            enemy_health: 0,
            board: vec![vec!['a'; height]; width],
        }
    }

    pub fn get_dim(&self) -> (usize, usize) {
        (
            self.board.len(),
            self.board.get(0).map(|row| row.len()).unwrap_or(0),
        )
    }

    pub fn board_at(&mut self, x: usize, y: usize) -> Option<&mut char> {
        // perform bounds check
        let (width, height) = self.get_dim();
        if x < width && y < height {
            Some(&mut self.board[x][y])
        } else {
            None
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new(6, 6)
    }
}

#[derive(Default)]
pub struct Game {
    state: GameState,
}

#[derive(Clone)]
pub struct Pos(usize, usize);

#[derive(Clone)]
pub enum Action {
    Randomize(Pos),
    Swap { p1: Pos, p2: Pos },
    Exchange { pos: Pos, next: char },
}

enum Effects {
    Damage(u32),
    Heal(u32),
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn player_action(&mut self, _id: ID, _action: Action) -> Option<()> {
        match _action {
            Action::Swap {
                p1: Pos(x1, y1),
                p2: Pos(x2, y2),
            } => {
                let mut p1_copy = self.state.board_at(x1, y1)?.clone();
                let mut p2_copy = self.state.board_at(x2, y2)?.clone();

                self.state.board_at(x1, y1).replace(&mut p2_copy);
                self.state.board_at(x2, y2).replace(&mut p1_copy);
            }

            Action::Exchange {
                pos: Pos(x, y),
                next,
            } => {
                self.state.board_at(x, y).map(|c| *c = next);
            }

            Action::Randomize(Pos(x, y)) => {
                self.state.board_at(x, y).map(|c| *c = todo!());
            }
        };

        Some(())
    }

    /// submit a word and then replace the letters
    /// on the consumed tiles
    pub fn submit_word<'a>(&mut self, tiles: impl Iterator<Item = &'a Pos>) -> Result<(), ()> {
        let word = self.assemble_word(tiles);
        self.check_word(&word)?;

        for effect in self.extract_word_effects(&word) {
            match effect {
                Effects::Heal(heal) => self.state.team_health += heal,
                Effects::Damage(damage) => {
                    self.state.enemy_health = self.state.enemy_health.saturating_sub(damage)
                }
            }
        }

        Ok(())
    }

    /// Contruct a word based on the position of tiles on the game board
    fn assemble_word<'a>(&self, _tiles: impl Iterator<Item = &'a Pos>) -> String {
        todo!()
    }

    fn extract_word_effects(&self, _word: &str) -> Vec<Effects> {
        vec![Effects::Damage(2), Effects::Heal(4)];

        todo!()
    }

    /// Verify whether the word is valid and what kind of effect it will have
    fn check_word(&self, _word: &str) -> Result<(), ()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ID: ID = "me";

    #[test]
    fn test() {
        let mut game = Game::new();
        game.player_action(TEST_ID, Action::Randomize(Pos(0, 0)));
        game.player_action(
            TEST_ID,
            Action::Exchange {
                pos: Pos(1, 1),
                next: 's',
            },
        );
        game.player_action(
            TEST_ID,
            Action::Swap {
                p1: Pos(0, 0),
                p2: Pos(0, 0),
            },
        );

        game.submit_word(
            [
                Pos(0, 0),
                Pos(1, 0),
                Pos(2, 0),
                Pos(3, 0),
                Pos(3, 1),
                Pos(3, 2),
                Pos(2, 3),
            ]
            .iter(),
        )
        .unwrap();
    }
}
