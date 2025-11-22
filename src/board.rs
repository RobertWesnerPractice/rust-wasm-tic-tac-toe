use crate::player::Player;

pub trait BoardT {
    fn clear(&mut self);
    fn set(&mut self, x: usize, y: usize) -> Result<Player, ()>;
    fn has_winner(&self) -> bool;
    fn is_draw(&self) -> bool;
    fn get_winner(&self) -> &Player;
}

pub struct Board {
    pub current_player: Player,
    pub state: [[Player; 3]; 3],
}
impl BoardT for Board {
    fn clear(&mut self) {
        for y in 0..3 {
            for x in 0..3 {
                self.state[y][x] = Player::None;
            }
        }
        self.current_player = Player::X;
    }

    fn set(&mut self, x: usize, y: usize) -> Result<Player, ()> {
        if self.state[y][x] != Player::None {
            return Err(())
        }

        self.state[y][x] = self.current_player;

        if self.current_player == Player::X {
            self.current_player = Player::O;
        } else {
            self.current_player = Player::X;
        }

        Ok(self.state[y][x])
    }
    
    fn has_winner(&self) -> bool {
        self.get_winner() != &Player::None
    }

    fn is_draw(&self) -> bool {
        for y in 0..3 {
            for x in 0..3 {
                if self.state[y][x] == Player::None {
                    return false
                }
            }
        }

        true
    }

    fn get_winner(&self) -> &Player {
        for i in 0..3 {
            if self.state[i][0] != Player::None && self.state[i][0] == self.state[i][1] && self.state[i][1] == self.state[i][2] {
                return &self.state[i][0];
            }
            if self.state[0][i] != Player::None && self.state[0][i] == self.state[1][i] && self.state[1][i] == self.state[2][i] {
                return &self.state[0][i];
            }
        }
        for i in 0..2 {
            if self.state[1][1] != Player::None && self.state[0][i * 2] == self.state[1][1] && self.state[1][1] == self.state[2][2 - i * 2] {
                return &self.state[1][1];
            }
        }

        &Player::None
    }
}