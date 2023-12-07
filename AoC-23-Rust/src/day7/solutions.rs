use crate::file_utility::file_utility::get_file_lines;

#[derive(Clone)]
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
                Err(_) => match self_cards[index] {
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
        panic!("Compare Failed!");
    }
}

#[derive(Clone)]
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
    let mut file_reader = get_file_lines("sample.txt", 7);

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
        let size = group.len();
        custom_quicksort(&mut group, 0, size - 1 as usize);
        for hand in group {
            winnings = hand.bid * rank;
            rank += 1;
        }
    }
    println!("{}", winnings);
}

fn determine_hand_type(hand: &str) -> HandType {
    let mut hand_split: Vec<&str> = hand.split("").collect();
    let mut card_holder: Vec<&str> = vec![hand_split.pop().unwrap()];
    let mut card_amount: Vec<i32> = vec![1];

    // Identify unique and duplicate cards in the hand
    for card in hand_split {
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

fn custom_quicksort(hand_vec: &mut Vec<Hand>, low_index: usize, high_index: usize) {
    if low_index < high_index {
        let pivot = partition(hand_vec, low_index, high_index);
        custom_quicksort(hand_vec, low_index, pivot - 1);
        custom_quicksort(hand_vec, pivot + 1, high_index);
    }
}

fn partition(hand_vec: &mut Vec<Hand>, low_index: usize, high_index: usize) -> usize {
    let mut i = low_index - 1;

    let mut j = low_index;
    while j < high_index {
        let pivot: String = hand_vec[high_index].cards.clone();
        if hand_vec[j].compare(pivot) == -1 {
            i += 1;
            hand_vec.swap(i, j);
        }
        j += 1;
    }
    hand_vec.swap(i + 1, high_index);
    return i + 1;
}
