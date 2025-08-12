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

    pub fn read_winner(&self) -> Option<(String, u32)> {
        let (p1_name, p1_score) = &self.p1;
        let (p2_name, p2_score) = &self.p2;

        if p1_score > p2_score {
            Some((p1_name.clone(), *p1_score))
        } else if p2_score > p1_score {
            Some((p2_name.clone(), *p2_score))
        } else {
            None // No winner yet
        }
    }

    pub fn update_score(&mut self, user_name: &str) {
        // Determine the winning threshold
        let threshold = (self.nb_games / 2) + 1;
        
        // Don't allow updates if someone has already reached the winning threshold
        if self.p1.1 >= threshold || self.p2.1 >= threshold {
            return;
        }
        
        // Don't allow updates if all games have been played
        let total_played = self.p1.1 + self.p2.1;
        if total_played >= self.nb_games {
            return;
        }

        // Update the score for the matching player
        if self.p1.0 == user_name {
            self.p1.1 += 1;
        } else if self.p2.0 == user_name {
            self.p2.1 += 1;
        }
        // If user_name doesn't match either player, ignore the update
    }

    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}