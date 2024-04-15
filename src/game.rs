use crate::{card::Card, dealer::Dealer, player::Player};

#[derive(Debug)]
pub struct GameOptions {
    pub num_seats: usize,
    pub min_buyin: u64,
    pub max_buyin: u64,
    pub small_blind: u64,
    pub big_blind: u64,
}

#[derive(Debug)]
pub struct Game {
    dealer: Dealer,
    seats: Vec<Option<Player>>,
    board: Vec<Card>,
    button: usize,
    turn: usize,
    num_players: usize,
    pot: u64,
    options: GameOptions,
    // TODO: sidepots
}

impl Game {
    pub fn new(options: GameOptions) -> Result<Self, String> {
        if options.num_seats < 2 {
            return Err(String::from("Game must have at least two seats!"));
        };
        Ok(Self {
            dealer: Dealer::new(),
            seats: vec![None; options.num_seats],
            board: Vec::new(),
            num_players: 0,
            turn: 0,
            button: 0,
            pot: 0,
            options,
        })
    }

    pub fn seat_player(&mut self, position: usize, player: Player) -> Result<(), String> {
        if player.chips < self.options.min_buyin {
            return Err(format!(
                "Not enough chips to seat player. Min buyin is {}, player has {}",
                self.options.min_buyin, player.chips
            ));
        }
        if player.chips > self.options.max_buyin {
            return Err(format!(
                "Too many chips to seat player. Max buyin is {}, player has {}",
                self.options.max_buyin, player.chips
            ));
        }
        match self.seats.get(position) {
            Some(None) => (),
            Some(Some(_)) => return Err(format!("Seat {position} is full")),
            None => {
                return Err(format!(
                    "Seat {position} does not exist, there are only {} seats",
                    self.seats.len()
                ))
            }
        };
        self.seats[position] = Some(player);
        self.num_players += 1;
        if self.num_players == 1 {
            self.button = position;
            self.turn = position
        }
        Ok(())
    }

    pub fn remove_player(&mut self, position: usize) -> Result<(), String> {
        let player = match self.seats.get(position) {
            Some(Some(player)) => player,
            Some(None) => return Err(format!("Seat {position} is empty")),
            None => {
                return Err(format!(
                    "Seat {position} does not exist, there are only {} seats",
                    self.seats.len()
                ))
            }
        };
        self.seats[position] = None;
        self.num_players -= 1;
        Ok(())
    }

    pub fn deal_to_board(&mut self, num_cards: usize) {
        self.board.append(&mut self.dealer.deal(num_cards))
    }

    pub fn deal_to_players(&mut self, num_cards: usize) {
        for player_option in &mut self.seats {
            if let Some(player) = player_option {
                player.take_cards(self.dealer.deal(num_cards))
            }
        }
    }

    pub fn rake_bets(&mut self) {
        for player_option in &mut self.seats {
            if let Some(player) = player_option {
                self.pot += player.bet;
                player.bet = 0;
            }
        }
    }

    // MAKE AN ITERATOR OF INDICES INSTEAD??
    // PERHAPS INSREAD OF HANDING OUT MUTABLE REFERENCES - MAKE A FUNCTION TO DO THE MUTATION
    fn position_at_or_next_to(&mut self, position: usize) -> usize {
        if let Some(Some(_)) = self.seats.get(position) {
            return position;
        }
        self.position_next_to(position)
    }

    fn position_next_to(&mut self, mut position: usize) -> usize {
        let initial = position;
        loop {
            position = (position + 1) % self.seats.len();
            if let Some(Some(_)) = self.seats.get(position) {
                return position;
            }
            if initial == position {
                panic!("No players found!")
            }
        }
    }

    fn get_mut_player(&mut self, position: usize) -> &mut Player {
        self.seats.get_mut(position).unwrap().as_mut().unwrap()
    }

    pub fn start_hand(&mut self) -> Result<&mut Player, String> {
        let small_blind = self.options.small_blind;
        let big_blind = self.options.big_blind;
        if self.num_players < 2 {
            return Err(String::from("Not enough players"));
        }
        // Heads up
        let mut position = if self.num_players == 2 {
            self.position_at_or_next_to(self.button)
        } else {
            self.position_next_to(self.button)
        };

        let mut player = self.get_mut_player(position);
        player.bet(small_blind);

        position = self.position_next_to(position);
        player = self.get_mut_player(position);
        player.bet(big_blind);

        self.turn = self.position_next_to(position);
        Ok(self.get_mut_player(self.turn))
    }

    pub fn next_hand(&mut self) {
        self.dealer = Dealer::new();
        self.board.clear();
        self.advance_button();
        self.start_hand();
    }

    pub fn advance_button(&mut self) {
        let position = self.position_next_to(self.button);
        self.button = position;
    }

    pub fn next_turn(&mut self) -> &mut Player {
        let position = self.position_next_to(self.turn);
        self.turn = position;
        self.get_mut_player(position)
    }
}
