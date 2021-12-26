use std::time::Instant;

trait Dice {
    fn roll_count(&self) -> u32;
    fn roll(&mut self) -> u32;
}

struct DeterministicDice {
    dice_value: u32,
    roll_count: u32,
}

impl Default for DeterministicDice {
    fn default() -> DeterministicDice {
        DeterministicDice {
            dice_value: 0,
            roll_count: 0,
        }
    }
}

impl Dice for DeterministicDice {
    fn roll(&mut self) -> u32 {
        self.roll_count += 1;

        self.dice_value %= 100;
        self.dice_value += 1;
        return self.dice_value;
    }

    fn roll_count(&self) -> u32 {
        return self.roll_count;
    }
}

struct Player {
    score: u32,
    position: u32,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            score: 0,
            position: 1
        }
    }
}

fn play_turn(player:&mut Player, dice:&mut dyn Dice) {
    player.position -= 1;
    player.position += dice.roll();
    player.position += dice.roll();
    player.position += dice.roll();

    player.position %= 10;
    player.position += 1;
    player.score += player.position;
}

fn play_game(player_one:&mut Player, player_two:&mut Player, dice: &mut dyn Dice) -> bool {
    let mut player_ones_turn = true;
    while player_one.score < 1000 && player_two.score < 1000 {
        if player_ones_turn {
            play_turn(player_one, dice);
        } else {
            play_turn(player_two, dice);
        }

        player_ones_turn = !player_ones_turn;
    }

    if player_one.score < 1000 {
        return false;
    } else {
        return true;
    }
}

fn part1(player_one_start: u32, player_two_start: u32, dice: &mut dyn Dice) {
    let mut player_one = Player::default();
    player_one.position = player_one_start;

    let mut player_two = Player::default();
    player_two.position = player_two_start;

    let did_player_one_win = play_game(&mut player_one, &mut player_two, dice);

    let losing_player = if did_player_one_win { player_two } else { player_one };
    println!("{} rolls, Player {} lost, with a score of {}. {}", dice.roll_count(), if did_player_one_win { "two" } else {"one"}, losing_player.score, losing_player.score * dice.roll_count());
}

fn main() {
    let start = Instant::now();

    println!("Prep took {:?}", start.elapsed());

    let start = Instant::now();
    part1(5, 10, &mut DeterministicDice::default());
    println!("Part 1 took {:?}", start.elapsed());
}
