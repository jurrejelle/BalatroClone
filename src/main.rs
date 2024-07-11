mod jokers;
mod cards;
mod suit;
mod modifiers;
mod game_stage;

use rand::Rng;

use std::sync::Arc;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use spin::Mutex;
use text_io::read;
use crate::jokers::*;
use crate::cards::*;
use crate::game_stage::GameStage;
use crate::suit::Suit;

struct GameState {
    money: i32,
    jokers: Vec<Arc<dyn Joker>>,
    hand: Vec<Arc<Card>>,
    played_hand: Vec<Arc<Card>>,
    deck: Vec<Arc<Card>>,
    blind_deck: Vec<Arc<Card>>,
    draw_size: usize,
    goal_chips: usize,
    blind: usize,
    ante: usize,
    chips: usize,
    total_hands: usize,
    total_discards: usize,
    hands: usize,
    discards: usize,
    game_stage: GameStage,
    nr_jokers_in_shop: usize,
    jokers_to_buy: Vec<Arc<dyn Joker>>,
    max_jokers: usize,
}

impl GameState{
    fn new() -> GameState {
        let mut gamestate = GameState { money: 0 ,
                                        jokers: Vec::new(),
                                        hand: Vec::new(),
                                        played_hand: Vec::new(),
                                        deck: Vec::new(),
                                        blind_deck: Vec::new(),
                                        draw_size: 8 ,
                                        goal_chips: 0,
                                        blind: 1,
                                        ante: 1,
                                        chips: 0,
                                        hands: 4,
                                        total_hands: 4,
                                        total_discards: 4,
                                        discards: 4,
                                        game_stage: GameStage::Playing,
                                        nr_jokers_in_shop: 2,
                                        jokers_to_buy: Vec::new(),
                                        max_jokers: 5 };

        for suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in 1..=13 {
                gamestate.deck.push(Arc::new(Card{ rank, suit: suit.clone() }));
            }
        }
        return gamestate;
    }

    // ===== Gameplay related functions =====
    fn print_blind_status(&self) {
        println!("\n\nBlind status: ");
        println!("Your jokers are:\n{}\n", format_jokers(self.jokers.clone()));
        println!("Cards in hand: {}", format_cards(self.hand.clone()));
        println!("Cards selected: {}", format_cards(self.played_hand.clone()));
    }

    fn blind_action(&mut self){
        loop {
            self.print_blind_status();
            println!("P      - Play hand");
            println!("D      - Discard hand");
            println!("S[0-{}] - Select card 0 to {}", self.hand.len() - 1, self.hand.len() - 1);
            let line: String = read!("{}\r\n");
            if line == "P" {
                println!("Playing cards");
                let result = self.play_hand();
                if result.is_err(){
                    println!("Could not play hand, error: {}", result.err().unwrap());
                    continue;
                }
                break;
            } else if line == "D" {
                println!("Discard");
                todo!("Discarding isn't implemented yet");
                break;
            } else if line.starts_with("S") {
                let suffix = &line[1..];
                if suffix.chars().all(char::is_numeric) {
                    let result = suffix.parse::<usize>();
                    if result.is_err() { continue };
                    let cardnr = result.unwrap();
                    if cardnr > self.hand.len() - 1 {
                        println!("Card number too high");
                        continue
                    }
                    let result = self.toggle_card_in_hand(cardnr);
                    if result.is_err(){
                        println!("Could not add/remove card, error: {}", result.err().unwrap());
                        continue;
                    }
                    break;
                }
            }
            println!("Please enter only one of the allowed values");
        }
    }

    fn shop_action(&mut self){
        println!("Welcome to the shop");
        loop {
            println!("You currently have ${}", self.money);
            if self.jokers_to_buy.len() > 0 {
                println!("The available jokers in the shop are: ");
                println!("{}", format_jokers(self.jokers_to_buy.clone()));
                println!("B[0-{}] - Buy joker 0-{}\n", self.jokers_to_buy.len() - 1, self.jokers_to_buy.len() - 1);
            }
            if self.jokers.len() > 0 {
                println!("The jokers currently in your hand are: ");
                println!("{}", format_jokers(self.jokers.clone()));
                println!("S[0-{}] - Sell joker 0-{}\n", self.jokers.len()-1, self.jokers.len()-1);
            }
            println!("C       - Continue to next ante");
            let line: String = read!("{}\r\n");
            if line == "C" {
                println!("Continuing to next blind");
                self.game_stage = GameStage::Playing;
                break;
            } else if line.starts_with("B") {
                if self.jokers.len() >= self.max_jokers + 1{
                    println!("No more free joker slots");
                }
                let suffix = &line[1..];
                if suffix.chars().all(char::is_numeric) {
                    let result = suffix.parse::<usize>();
                    if result.is_err() { continue };
                    let jokernr = result.unwrap();
                    if jokernr + 1 > self.jokers_to_buy.len() {
                        println!("Joker number too high");
                        continue
                    }
                    println!("Buying joker nr {}", jokernr);
                    let joker = self.jokers_to_buy.remove(jokernr);
                    if joker.get_cost() > self.money {
                        println!("Can't buy joker, insufficient funds");
                    }
                    self.money -= joker.get_cost();
                    self.jokers.push(joker);
                    break;
                }
            } else if line.starts_with("S") {
                let suffix = &line[1..];
                if suffix.chars().all(char::is_numeric) {
                    let result = suffix.parse::<usize>();
                    if result.is_err() { continue };
                    let jokernr = result.unwrap();
                    if jokernr > self.jokers.len() - 1 {
                        println!("Can't sell joker nr {} - too high", jokernr);
                        continue
                    }
                    println!("Selling joker nr {}", jokernr);
                    let joker = self.jokers.remove(jokernr);
                    // TODO: Implement realistic sell value
                    self.money += joker.get_cost();
                    break;
                }
            }
            println!("Please enter only one of the allowed values");
        }
    }

    // ===== Blind related functions =====
    fn start_blind(&mut self){
        self.blind_deck = self.deck.clone();
        self.played_hand = Vec::new();
        self.hand = Vec::new();
        self.start_turn();
    }

    fn start_turn(&mut self){
        while self.hand.len() < self.draw_size {
            let card = self.draw_card();
            self.hand.push(card);
        }
        self.hands = self.total_hands;
        self.discards = self.total_discards;
        self.goal_chips = self.calculate_goal_chips();
    }

    fn play_hand(&mut self) -> Result<(), String>{
        if self.played_hand.len() == 0 {
            return Err("Can't play empty hand".to_string());
        }

        let score = self.calculate_score();
        self.chips += score;
        println!("Scored {} out of {} to progress to next blind", self.chips, self.goal_chips);
        self.hands -= 1;
        if self.chips >= self.goal_chips {
            self.finish_blind();
            return Ok(());
        }
        if self.hands == 0 {
            self.game_over();
            return Err("Game over".to_string());
        }
        for card in self.played_hand.clone(){
            if let Some(i) = self.hand.iter()
                .position(|x| **x == *card){
                self.hand.remove(i);
                continue;
            }
            panic!("Card is in played_hand but not in hand after playing");
        }
        self.played_hand = Vec::new();
        self.start_turn();
        return Ok(());
    }

    fn finish_blind(&mut self){
        match self.blind {
            0 => self.money = self.money + 4 + (self.hands as i32),
            1 => self.money = self.money + 5 + (self.hands as i32),
            2 => self.money = self.money + 6 + (self.hands as i32),
            _ => panic!("Blind should be 0 to 2")
        }
        self.blind += 1;
        if self.blind == 3 {
            self.blind = 0;
            self.ante += 1;
        }
        println!("Moving on to ante {}, blind {}", self.ante, self.blind);
        self.game_stage = GameStage::Shop;
    }

    fn game_over(&mut self){
        self.game_stage = GameStage::NotPlaying;
        panic!("Game over, implement this later");
    }

    // ===== Shop logic ======
    fn start_shop(&mut self){
        self.populate_shop();
    }
    fn populate_shop(&mut self){
        let mut rng = rand::thread_rng();
        for _ in 0..self.nr_jokers_in_shop{
            let joker = ALL_JOKERS.choose(&mut rng).expect("Jokers should be available");
            self.jokers_to_buy.push(joker.clone());
        }
        // TODO: disallow repeat jokers
        // TODO: Add vouchers, packs
    }


    // ===== Card and state related functions =====
    fn add_joker(&mut self, joker: Arc<dyn Joker>){
        self.jokers.push(joker);
    }

    fn calculate_score(&self) -> usize {
        let mut chips : usize = 0;
        let mut mult : usize = 1;
        for card in &self.played_hand {
            chips = card.apply_chips(chips);
            mult = card.apply_mult(mult);
        }
        for joker in &self.jokers {
            chips = joker.apply_chips(chips);
            mult = joker.apply_mult(mult);
        }
        return chips * mult;
    }


    fn calculate_goal_chips(&self) -> usize {
        // TODO: Make accurate to balatro
        return (self.blind + self.ante * 3 - 1) * 100;
    }

    fn draw_card(&mut self) -> Arc<Card>{
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.blind_deck.len());
        return self.blind_deck.remove(index);
    }

    fn toggle_card_in_hand(&mut self, index: usize) -> Result<(), String>{
        // Try to remove card from hand
        if let Some(i) = self.played_hand.iter()
            .position(|x| **x == *self.hand[index]) {
            self.played_hand.remove(i);
            return Ok(());
        }

        // Try to add card to hand
        if self.played_hand.len() >= 5 {
            return Err("Cannot add card to hand, hand is full".to_string());
        }
        println!("Adding card {} to hand",self.hand[index].clone());
        self.played_hand.push(self.hand[index].clone());
        return Ok(());
    }
}
fn format_cards(cards : Vec<Arc<Card>>) -> String {
    let card_descriptions: Vec<String> = cards.iter().map(|card| card.to_string()).collect();
    format!("[{}]", card_descriptions.join(", "))
}

fn format_jokers(jokers: Vec<Arc<dyn Joker>>) -> String {
    format!("* {}", jokers.iter()
                        .map(|card| card.get_shop_description())
                        .collect::<Vec<String>>()
                        .join("\n* "))
}

lazy_static! {
    static ref GAME_STATE: Mutex<GameState> = Mutex::new(
        GameState::new()
    );
}



fn main() {
    let mut game_state = GAME_STATE.lock();
    let mut rng = rand::thread_rng();
    game_state.add_joker( Arc::new( JokerMult4::default() ) );
    game_state.add_joker( Arc::new( JokerMult4::default() ) );
    // Gameplay loop:

    while game_state.game_stage != GameStage::NotPlaying {
        // Game_state playing
        game_state.start_blind();
        while game_state.game_stage == GameStage::Playing {
            game_state.blind_action();
        }
        game_state.start_shop();
        while game_state.game_stage == GameStage::Shop {
            game_state.shop_action();
        }
    }
    game_state.start_blind();
    println!("{}", format_cards(game_state.hand.clone()));
    for _ in 0..5 {
        let card_index = rng.gen_range(0..game_state.hand.len());
        _ = game_state.toggle_card_in_hand(card_index);
    }
    println!("{}", format_cards(game_state.played_hand.clone()));

    println!("The score is {}", game_state.calculate_score());
}
