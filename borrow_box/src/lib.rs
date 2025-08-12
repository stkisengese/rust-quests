#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u32),
    pub p2: (String, u32),
    pub nb_games: u32,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u32) -> GameSession {
        GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        }
    }

    pub fn read_winner(&self) -> Option<&(String, u32)> {
        let threshold = (self.nb_games / 2) + 1;

        if self.p1.1 >= threshold {
            Some(&self.p1)
        } else if self.p2.1 >= threshold {
            Some(&self.p2)
        } else {
            None // It's a draw
        }
    }

    pub fn update_score(&mut self, user_name: &str) {
          // Check total games played so far
        let total_played = self.p1.1 + self.p2.1;
        if total_played >= self.nb_games {
            return;
        }

        // Update score for the player
        if self.p1.0 == user_name {
            self.p1.1 += 1;
        } else if self.p2.0 == user_name {
            self.p2.1 += 1;
        }
    }

    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}

