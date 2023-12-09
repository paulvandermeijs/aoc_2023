use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Hand {
    value: String,
    hand_type: HandType,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        Self {
            value: value.into(),
            hand_type: value.into(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::Equal;

        let hand_type_result = self.hand_type.cmp(&other.hand_type);

        if Equal != hand_type_result {
            return hand_type_result;
        }

        for (a, b) in std::iter::zip(self.value.chars(), other.value.chars()) {
            let a: Card = a.into();
            let b: Card = b.into();

            let card_result = a.cmp(&b);

            if Equal != card_result {
                return card_result;
            }
        }

        Equal
    }
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Joker,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Unknown card"),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let s = *self as u8;
        let other = *other as u8;

        s.cmp(&other)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&str> for HandType {
    fn from(value: &str) -> Self {
        let mut count =
            value
                .chars()
                .into_iter()
                .fold(HashMap::<char, u32>::new(), |mut map, ch| {
                    map.entry(ch)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                    map
                });

        let joker_count = *count.get(&'J').unwrap_or(&0);

        count.remove(&'J');

        let mut count = count.into_values().collect::<Vec<u32>>();

        count.sort();
        count.reverse();

        let mut count_padded: [u32; 5] = [0; 5];

        count_padded[..count.len()].copy_from_slice(&count);

        count_padded[0] += joker_count;

        match count_padded {
            [5, _, _, _, _] => Self::FiveOfAKind,
            [4, 1, _, _, _] => Self::FourOfAKind,
            [3, 2, _, _, _] => Self::FullHouse,
            [3, _, _, _, _] => Self::ThreeOfAKind,
            [2, 2, _, _, _] => Self::TwoPair,
            [2, _, _, _, _] => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let s = *self as u8;
        let other = *other as u8;

        s.cmp(&other)
    }
}

fn main() {
    let mut hands = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (hand, bid) = line.split_once(' ').unwrap();

            (hand.into(), bid.parse::<u32>().unwrap())
        })
        .collect::<Vec<(Hand, u32)>>();

    hands.sort_by(|(a, _), (b, _)| a.cmp(b));

    let result = hands
        .into_iter()
        .enumerate()
        .fold(0, |r, (i, (_, v))| r + ((i as u32 + 1) * v));

    println!("{:?}", result);
}
