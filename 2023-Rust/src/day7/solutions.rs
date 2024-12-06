use crate::file_utility::file_utility::get_file_lines;

#[derive(Clone, Debug)]
struct Hand {
    cards: String,
    bid: i32,
    hand_type: HandType,
}

impl Hand {
    fn compare(&self, other: String) -> i32 {
        let self_cards: Vec<&str> = self.cards.split("").collect();
        let other_cards: Vec<&str> = other.split("").collect();

        for (index, card) in self_cards.iter().enumerate() {
            if other_cards[index] == *card {
                continue;
            }

            // Cards are different, one will be higher rank than the other
            let self_card = match self_cards[index].parse::<i32>() {
                Ok(number) => number,
                Err(_) => match self_cards[index] {
                    "A" => 14,
                    "K" => 13,
                    "Q" => 12,
                    "J" => 11,
                    "T" => 10,
                    _ => panic!("Card is not valid"),
                },
            };

            let other_card = match other_cards[index].parse::<i32>() {
                Ok(number) => number,
                Err(_) => match other_cards[index] {
                    "A" => 14,
                    "K" => 13,
                    "Q" => 12,
                    "J" => 11,
                    "T" => 10,
                    _ => panic!("Card is not valid"),
                },
            };

            if self_card > other_card {
                return 1;
            } else if self_card < other_card {
                return -1;
            }
            panic!("Hands were not unique!");
        }
        return 0;
    }

    fn compare2(&self, other: String) -> i32 {
        let self_cards: Vec<&str> = self.cards.split("").collect();
        let other_cards: Vec<&str> = other.split("").collect();

        for (index, card) in self_cards.iter().enumerate() {
            if other_cards[index] == *card {
                continue;
            }

            // Cards are different, one will be higher rank than the other
            let self_card = match self_cards[index].parse::<i32>() {
                Ok(number) => number,
                Err(_) => match self_cards[index] {
                    "A" => 14,
                    "K" => 13,
                    "Q" => 12,
                    "J" => 1,
                    "T" => 10,
                    _ => panic!("Card is not valid"),
                },
            };

            let other_card = match other_cards[index].parse::<i32>() {
                Ok(number) => number,
                Err(_) => match other_cards[index] {
                    "A" => 14,
                    "K" => 13,
                    "Q" => 12,
                    "J" => 1,
                    "T" => 10,
                    _ => panic!("Card is not valid"),
                },
            };

            if self_card > other_card {
                return 1;
            } else if self_card < other_card {
                return -1;
            }
            panic!("Hands were not unique!");
        }
        return 0;
    }
}

#[derive(Clone, Debug)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

pub fn solution_part1() {
    let mut file_reader = get_file_lines("input.txt", 7);

    // Vector for each type of hand
    let mut hands: Vec<Vec<Hand>> = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];

    // Group all the hands by hand type
    while file_reader.peek().is_some() {
        let line = file_reader.next().unwrap().unwrap();
        let line_split: Vec<&str> = line.split_ascii_whitespace().collect();

        let hand = Hand {
            cards: String::from(line_split[0]),
            bid: line_split[1].parse::<i32>().unwrap(),
            hand_type: determine_hand_type(line_split[0]),
        };

        let clone = hand.clone();
        hands[clone.hand_type as usize].push(hand);
    }

    let mut rank = 1;
    let mut winnings = 0;
    // Now sort the groups and get their ranks
    for mut group in hands {
        group = custom_quicksort(group);

        for hand in group {
            winnings += hand.bid * rank;
            rank += 1;
        }
    }
    println!("{}", winnings);
}

//     253362743
pub fn solution_part2() {
    let mut file_reader = get_file_lines("input.txt", 7);

    // Vector for each type of hand
    let mut hands: Vec<Vec<Hand>> = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];

    // Group all the hands by hand type
    while file_reader.peek().is_some() {
        let line = file_reader.next().unwrap().unwrap();
        let line_split: Vec<&str> = line.split_ascii_whitespace().collect();

        let hand = Hand {
            cards: String::from(line_split[0]),
            bid: line_split[1].parse::<i32>().unwrap(),
            hand_type: determine_hand_type2(line_split[0]),
        };

        let clone = hand.clone();
        hands[clone.hand_type as usize].push(hand);
    }

    let mut rank = 1;
    let mut winnings = 0;
    // Now sort the groups and get their ranks
    for mut group in hands {
        group = custom_quicksort2(group);

        for hand in group {
            if hand.cards.contains("J") {
                println!("{:?}", hand);
            }

            winnings += hand.bid * rank;
            rank += 1;
        }
    }
    println!("{}", winnings);
}

fn determine_hand_type(hand: &str) -> HandType {
    let mut hand_split: Vec<&str> = hand.split("").collect();
    hand_split.pop(); // The ends are always blank when spliting each character
    let mut card_holder: Vec<&str> = vec![hand_split.pop().unwrap()];
    let mut card_amount: Vec<i32> = vec![1];

    // Identify unique and duplicate cards in the hand
    for card in hand_split {
        if card == "" {
            continue;
        }
        match card_holder.iter().position(|&c| c == card) {
            // We have this card in our hand already, so increment the amount we have.
            Some(index) => {
                card_amount[index] += 1;
            }
            // We have not seen this card in this hand. Add it to the holder and add a new amount.
            None => {
                card_holder.push(card);
                card_amount.push(1);
            }
        }
    }

    match card_holder.len() {
        1 => {
            return HandType::FiveOfAKind;
        }
        2 => {
            if card_amount.contains(&4) {
                return HandType::FourOfAKind;
            }
            return HandType::FullHouse;
        }
        3 => {
            if card_amount.contains(&3) {
                return HandType::ThreeOfAKind;
            }
            return HandType::TwoPair;
        }
        4 => {
            return HandType::OnePair;
        }
        5 => {
            return HandType::HighCard;
        }
        _ => {
            panic!("Player cheated!");
        }
    }
}

fn determine_hand_type2(hand: &str) -> HandType {
    let mut hand_split: Vec<&str> = hand.split("").collect();
    hand_split.pop(); // The ends are always blank when spliting each character
    let mut card_holder: Vec<&str> = vec![hand_split.pop().unwrap()];
    let mut card_amount: Vec<i32> = vec![1];
    let mut jokers = 0;
    if card_holder[0] == "J" {
        jokers += 1;
    }

    // Identify unique and duplicate cards in the hand
    for card in hand_split {
        if card == "" {
            continue;
        }
        if card == "J" {
            jokers += 1;
        }
        match card_holder.iter().position(|&c| c == card) {
            // We have this card in our hand already, so increment the amount we have.
            Some(index) => {
                card_amount[index] += 1;
            }
            // We have not seen this card in this hand. Add it to the holder and add a new amount.
            None => {
                card_holder.push(card);
                card_amount.push(1);
            }
        }
    }

    match card_holder.len() {
        1 => {
            return HandType::FiveOfAKind;
        }
        2 => {
            if card_amount.contains(&4) {
                if jokers == 1 || jokers == 4 {
                    return HandType::FiveOfAKind;
                }
                return HandType::FourOfAKind;
            }
            if jokers == 2 || jokers == 3 {
                return HandType::FiveOfAKind;
            }
            return HandType::FullHouse;
        }
        3 => {
            if card_amount.contains(&3) {
                if jokers == 3 || jokers == 1 {
                    return HandType::FourOfAKind;
                }
                return HandType::ThreeOfAKind;
            }
            if jokers == 2 {
                return HandType::FourOfAKind;
            }
            if jokers == 1 {
                return HandType::FullHouse;
            }
            return HandType::TwoPair;
        }
        4 => {
            if jokers == 2 || jokers == 1 {
                return HandType::ThreeOfAKind;
            }
            return HandType::OnePair;
        }
        5 => {
            if jokers == 1 {
                return HandType::OnePair;
            }
            return HandType::HighCard;
        }
        _ => {
            panic!("Player cheated!");
        }
    }
}

fn custom_quicksort(mut hand_vec: Vec<Hand>) -> Vec<Hand> {
    if hand_vec.len() <= 1 {
        return hand_vec;
    }

    let pivot = hand_vec.remove(0);
    let mut left = vec![];
    let mut right = vec![];

    for item in hand_vec {
        let p_cards = pivot.clone().cards;
        if item.compare(p_cards) == -1 {
            left.push(item);
        } else {
            right.push(item);
        }
    }

    let mut sorted_left = custom_quicksort(left);
    let mut sorted_right = custom_quicksort(right);

    sorted_left.push(pivot);
    sorted_left.append(&mut sorted_right);
    return sorted_left;
}

fn custom_quicksort2(mut hand_vec: Vec<Hand>) -> Vec<Hand> {
    if hand_vec.len() <= 1 {
        return hand_vec;
    }

    let pivot = hand_vec.remove(0);
    let mut left = vec![];
    let mut right = vec![];

    for item in hand_vec {
        let p_cards = pivot.clone().cards;
        if item.compare2(p_cards) == -1 {
            left.push(item);
        } else {
            right.push(item);
        }
    }

    let mut sorted_left = custom_quicksort2(left);
    let mut sorted_right = custom_quicksort2(right);

    sorted_left.push(pivot);
    sorted_left.append(&mut sorted_right);
    return sorted_left;
}
