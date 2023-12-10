use std::str::FromStr;
use std::cmp::{PartialOrd, PartialEq, Ordering};
use regex::Regex;

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
enum CType {
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
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
enum HType {
    HighCard,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}


#[derive(PartialOrd, Ord, Clone, Debug)]
struct Card {
    t: CType,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.t == other.t || self.t == CType::Joker || other.t == CType::Joker;
    }
}

impl Eq for Card {}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = match s {
            "A" => CType::Ace,
            "K" => CType::King,
            "Q" => CType::Queen,
            "J" => CType::Jack,
            "I" => CType::Joker,
            "T" => CType::Ten,
            "9" => CType::Nine,
            "8" => CType::Eight,
            "7" => CType::Seven,
            "6" => CType::Six,
            "5" => CType::Five,
            "4" => CType::Four,
            "3" => CType::Three,
            "2" => CType::Two,
            _ => todo!()
        };
        Ok(Card { t: t })
    }
}

#[derive(Debug)]
struct Hand {
    first: Card,
    second: Card,
    third: Card,
    fourth: Card,
    fifth: Card,
    bid: u128,
    h_type: HType,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.h_type == other.h_type &&
                self.first == other.first &&
                self.second == other.second &&
                self.third == other.third &&
                self.fourth == other.fourth &&
                self.fifth == other.fifth;
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.h_type == other.h_type {
            if self.first != other.first {
                if self.first > other.first {
                    println!("bigger1");
                    return Ordering::Greater;
                } else if self.first < other.first {
                    println!("smaller1");
                    return Ordering::Less;
                }
            } else {
                if self.first.t == CType::Joker {
                    return Ordering::Less;
                } else if other.first.t == CType::Joker {
                    return Ordering::Greater;
                }
            }
            if self.second != other.second {
                if self.second > other.second {
                    println!("bigger2");
                    return Ordering::Greater;
                } else if self.second < other.second {
                    println!("smaller2");
                    return Ordering::Less;
                }
            } else {
                if self.second.t == CType::Joker {
                    return Ordering::Less;
                } else if other.second.t == CType::Joker {
                    return Ordering::Greater;
                }
            }
            if self.third != other.third {
                if self.third > other.third {
                    println!("bigger3");
                    return Ordering::Greater;
                } else if self.third < other.third {
                    println!("smaller3");
                    return Ordering::Less;
                }
            } else {
                if self.third.t == CType::Joker {
                    println!("Joker3 less");
                    return Ordering::Less
                } else if other.third.t == CType::Joker {
                    println!("Joker3 greater");
                    return Ordering::Greater;
                }
            }
            if self.fourth != other.fourth {
                if self.fourth > other.fourth {
                    println!("bigger4");
                    return Ordering::Greater;
                } else if self.fourth < other.fourth {
                    println!("smaller4");
                    return Ordering::Less;
                }
            } else {
                if self.fourth.t == CType::Joker {
                    println!("Joker4 less");
                    return Ordering::Less;
                } else if other.fourth.t == CType::Joker {
                    println!("Joker4 greater");
                    return Ordering::Greater;
                }
            }
            if self.fifth != other.fifth {
                if self.fifth > other.fifth {
                    println!("bigger5");
                    return Ordering::Greater;
                } else if self.fifth < other.fifth {
                    println!("smaller5");
                    return Ordering::Less;
                } else {
                    return Ordering::Equal;
                }
            } else {
                if self.fifth.t == CType::Joker {
                    return Ordering::Less;
                } else if other.fifth.t == CType::Joker {
                    return Ordering::Greater;
                }
                return Ordering::Equal;
            }
            panic!();
        } else {
            return self.h_type.cmp(&other.h_type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?<fi>\w)(?<s>\w)(?<t>\w)(?<f>\w)(?<i>\w) (?<bid>\d+)").unwrap();
        let Some(caps) = re.captures(s) else {
            println!("ERROR");
            panic!();
        };
        let bid: u128 = caps["bid"].parse().unwrap();
        let fi = Card::from_str(&caps["fi"]).unwrap();
        let s = Card::from_str(&caps["s"]).unwrap();
        let t = Card::from_str(&caps["t"]).unwrap();
        let f = Card::from_str(&caps["f"]).unwrap();
        let i = Card::from_str(&caps["i"]).unwrap();
        let mut h = Hand { first: fi.clone(), second: s.clone(), third: t.clone(), fourth: f.clone(), fifth: i.clone(), bid: bid, h_type: HType::HighCard };
        if h.is_five_of_kind() {
            h.h_type = HType::Five;
        } else if h.is_full_house() {
            h.h_type = HType::FullHouse;
        } else if h.is_four_of_kind() {
            h.h_type = HType::Four;
        } else if h.is_three_of_kind() {
            h.h_type = HType::Three;
        } else if h.is_two_pair() {
            h.h_type = HType::TwoPair;
        } else if h.is_pair() {
            h.h_type = HType::Pair;
        } else {
            h.h_type = HType::HighCard;
        }
        // println!("{:?}", h.h_type);
        Ok(h)
    }
}

impl Hand {
    fn is_five_of_kind(&self) -> bool {
        return (&self.first == &self.second) &&
            (&self.first == &self.third) &&
            (&self.first == &self.fourth)  &&
            (&self.first == &self.fifth);
    }

    fn is_four_of_kind(&self) -> bool {
        let mut cards: Vec<Card> = vec![
            self.first.clone(),
            self.second.clone(),
            self.third.clone(),
            self.fourth.clone(),
            self.fifth.clone(),
        ];
        cards.sort();
        return cards[0] == cards[1] && cards[1] == cards[2] && cards[2] == cards[3] && cards[4] != cards[3] ||
            cards[0] != cards[1] && cards[1] == cards[2] && cards[2] == cards[3] && cards[3] == cards[4];
    }

    fn is_full_house(&self) -> bool {
        let mut cards: Vec<Card> = vec![
            self.first.clone(),
            self.second.clone(),
            self.third.clone(),
            self.fourth.clone(),
            self.fifth.clone(),
        ];
        cards.sort();
        println!("{:?}", cards);
        return (cards[0] == cards[1] && cards[1] == cards[2] && cards[3] == cards[4] ||
            cards[0] == cards[1] && cards[2] == cards[3] && cards[3] == cards[4]) && ! self.is_five_of_kind() && ! self.is_four_of_kind()
    }

    fn is_three_of_kind(&self) -> bool {
        let mut cards: Vec<Card> = vec![
            self.first.clone(),
            self.second.clone(),
            self.third.clone(),
            self.fourth.clone(),
            self.fifth.clone(),
        ];
        cards.sort();
        return (cards[0] == cards[1] && cards[1] == cards[2] ||
            cards[1] == cards[2] && cards[2] == cards[3] ||
            cards[2] == cards[3] && cards[3] == cards[4] ) && ! self.is_full_house() && ! self.is_four_of_kind() && ! self.is_five_of_kind()
    }

    fn is_two_pair(&self) -> bool {
        let mut cards: Vec<Card> = vec![
            self.first.clone(),
            self.second.clone(),
            self.third.clone(),
            self.fourth.clone(),
            self.fifth.clone(),
        ];
        cards.sort();
        return (cards[0] == cards[1] && cards[2] == cards[3] ||
                cards[1] == cards[2] && cards[3] == cards[4] ||
                cards[0] == cards[1] && cards[3] == cards[4]) && ! self.is_full_house() && !self.is_four_of_kind() && ! self.is_three_of_kind() && ! self.is_five_of_kind()
    }

    fn is_pair(&self) -> bool {
        let mut cards: Vec<Card> = vec![
            self.first.clone(),
            self.second.clone(),
            self.third.clone(),
            self.fourth.clone(),
            self.fifth.clone(),
        ];
        cards.sort();
        return !self.is_two_pair() && !self.is_three_of_kind() && !self.is_four_of_kind() && !self.is_five_of_kind() && !self.is_full_house() &&
            (cards[0] == cards[1] || cards[1] == cards[2] || cards[2] == cards[3] || cards[3] == cards[4]);
    }
}

fn solve1(input: &str) -> u128 {
    let mut sum = 0;
    let lines = input.lines();
    let mut hands: Vec<Hand> = vec![];
    for line in lines {
        hands.push(Hand::from_str(line).unwrap());
    }
    hands.sort();
    for (i, hand) in hands.iter().enumerate() {
        println!("i: {} bid: {} type: {:?}", i, hand.bid, hand.h_type);
        sum += hand.bid * (i+1) as u128;
    }
    sum
}

fn solve2(input: &str) -> u128 {
    let mut sum = 0;
    let lines = input.lines();
    let mut hands: Vec<Hand> = vec![];
    for line in lines {
        hands.push(Hand::from_str(&line.replace("J","I")).unwrap());
    }
    hands.sort();
    for (i, hand) in hands.iter().enumerate() {
        println!("i: {} bid: {} type: {:?}", i, hand.bid, hand.h_type);
        sum += hand.bid * (i+1) as u128;
    }
    sum
}

pub fn solve() {
    let input = include_str!("input");
    println!("Solution 1: {}", solve1(input));
    println!("Solution 2: {}", solve2(input));
}

#[cfg(test)]
mod tests {
    use crate::seven::*;

    const SAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    const SAMPLE2: &str = "32T3K 765
T55I5 684
KK677 28
KTIIT 220
QQQIA 483";

    #[test]
    fn one_solved() {
        let res = solve1(SAMPLE);
        assert_eq!(res, 6440);
    }

    #[test]
    fn two_solved() {
        let res = solve2(SAMPLE);
        assert_eq!(res, 5905);
    }

    #[test]
    fn test_is_full_house () {
        let hand = Hand {
            first: Card { t: CType::Jack },
            second: Card { t: CType::Queen },
            third: Card { t: CType::Queen },
            fourth: Card { t: CType::Jack },
            fifth: Card { t: CType::Jack },
            bid: 0,
            h_type: HType::HighCard,
        };
        assert_eq!(true, hand.is_full_house());
    }

    #[test]
    fn test_is_four_of_kind () {
        let hand = Hand {
            first: Card { t: CType::Jack },
            second: Card { t: CType::Jack },
            third: Card { t: CType::Queen },
            fourth: Card { t: CType::Jack },
            fifth: Card { t: CType::Jack },
            bid: 0,
            h_type: HType::HighCard,
        };
        assert_eq!(true, hand.is_four_of_kind());
    }

    #[test]
    fn test_is_three_of_kind () {
        let hand = Hand {
            first: Card { t: CType::Jack },
            second: Card { t: CType::Jack },
            third: Card { t: CType::Queen },
            fourth: Card { t: CType::Ten },
            fifth: Card { t: CType::Jack },
            bid: 0,
            h_type: HType::HighCard,
        };
        assert_eq!(true, hand.is_three_of_kind());
    }

    #[test]
    fn test_is_not_three_of_kind () {
        let hand = Hand {
            first: Card { t: CType::Jack },
            second: Card { t: CType::Jack },
            third: Card { t: CType::Queen },
            fourth: Card { t: CType::Queen },
            fifth: Card { t: CType::Jack },
            bid: 0,
            h_type: HType::HighCard,
        };
        assert_eq!(false, hand.is_three_of_kind());
    }

    #[test]
    fn test_is_two_pair () {
        let hand = Hand {
            first: Card { t: CType::Jack },
            second: Card { t: CType::Jack },
            third: Card { t: CType::Queen },
            fourth: Card { t: CType::Queen },
            fifth: Card { t: CType::Ten },
            bid: 0,
            h_type: HType::HighCard,
        };
        assert_eq!(true, hand.is_two_pair());
    }

    #[test]
    fn test_is_not_two_pair () {
        let hand = Hand {
            first: Card { t: CType::Jack },
            second: Card { t: CType::Jack },
            third: Card { t: CType::Queen },
            fourth: Card { t: CType::Queen },
            fifth: Card { t: CType::Jack },
            bid: 0,
            h_type: HType::HighCard,
        };
        assert_eq!(false, hand.is_two_pair());
    }

    #[test]
    fn test_is_pair () {
        let hand = Hand {
            first: Card { t: CType::Jack },
            second: Card { t: CType::Jack },
            third: Card { t: CType::Ten },
            fourth: Card { t: CType::Nine },
            fifth: Card { t: CType::Eight },
            bid: 0,
            h_type: HType::HighCard,
        };
        assert_eq!(true, hand.is_pair());
    }

    #[test]
    fn test_is_not_pair () {
        let hand = Hand {
            first: Card { t: CType::Jack },
            second: Card { t: CType::Jack },
            third: Card { t: CType::Ten },
            fourth: Card { t: CType::Ten },
            fifth: Card { t: CType::Eight },
            bid: 0,
            h_type: HType::HighCard,
        };
        assert_eq!(false, hand.is_pair());
    }

    #[test]
    fn test_eq_hands() {
        let hand_one = Hand {
            first: Card { t: CType::Jack },
            second: Card { t: CType::Jack },
            third: Card { t: CType::Ten },
            fourth: Card { t: CType::Ten },
            fifth: Card { t: CType::Eight },
            bid: 0,
            h_type: HType::HighCard,
        };
        let hand_two = Hand {
            first: Card { t: CType::Jack },
            second: Card { t: CType::Jack },
            third: Card { t: CType::Ten },
            fourth: Card { t: CType::Ten },
            fifth: Card { t: CType::Eight },
            bid: 0,
            h_type: HType::HighCard,
        };
        assert_eq!(hand_one, hand_two);
    }

    #[test]
    fn test_five_bigger_full_house() {
        let hand_one = Hand {
            first: Card { t: CType::Jack },
            second: Card { t: CType::Jack },
            third: Card { t: CType::Jack },
            fourth: Card { t: CType::Jack },
            fifth: Card { t: CType::Jack },
            bid: 0,
            h_type: HType::Five,
        };
        let hand_two = Hand {
            first: Card { t: CType::Queen },
            second: Card { t: CType::Queen },
            third: Card { t: CType::King },
            fourth: Card { t: CType::King },
            fifth: Card { t: CType::Queen },
            bid: 0,
            h_type: HType::FullHouse,
        };
        assert_eq!(true, hand_one > hand_two);
    }
    #[test]
    fn test_high_cards1() {
        let hand_one = Hand {
            first: Card { t: CType::Two },
            second: Card { t: CType::Three },
            third: Card { t: CType::Four },
            fourth: Card { t: CType::Five },
            fifth: Card { t: CType::Six },
            bid: 0,
            h_type: HType::HighCard,
        };
        let hand_two = Hand {
            first: Card { t: CType::Three },
            second: Card { t: CType::Four },
            third: Card { t: CType::Five },
            fourth: Card { t: CType::Six },
            fifth: Card { t: CType::Seven },
            bid: 0,
            h_type: HType::HighCard,
        };
        assert_eq!(true, hand_one < hand_two);
    }
    #[test]
    fn test_high_cards2() {
        let hand_one = Hand {
            first: Card { t: CType::Two },
            second: Card { t: CType::Three },
            third: Card { t: CType::Four },
            fourth: Card { t: CType::Five },
            fifth: Card { t: CType::Six },
            bid: 0,
            h_type: HType::HighCard,
        };
        let hand_two = Hand {
            first: Card { t: CType::Two },
            second: Card { t: CType::Three },
            third: Card { t: CType::Five },
            fourth: Card { t: CType::Six },
            fifth: Card { t: CType::Seven },
            bid: 0,
            h_type: HType::HighCard,
        };
        assert_eq!(true, hand_one < hand_two);
    }

    #[test]
    fn check_is_four_of_kind2() {
        let lines = SAMPLE2.lines();
        let res;
        res = Hand::from_str(lines.last().unwrap()).unwrap();
        assert_eq!(true, res.is_four_of_kind())
    }

    #[test]
    fn check_all_jokers_is_less_than_all_tens() {
        let jk_line = "IIIII 123";
        let ten_line = "TTTTT 32";
        let jk_hand = Hand::from_str(jk_line).unwrap();
        let ten_hand = Hand::from_str(ten_line).unwrap();
        println!("{:?}", jk_hand);
        println!("{:?}", ten_hand);
        assert_eq!(true, jk_hand < ten_hand);
    }

    #[test]
    fn check_joker_creates_full_house() {
        let line = "TTIQQ 123";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(hand.h_type, HType::FullHouse);
        assert_eq!(true, hand.is_full_house());
    }

    #[test]
    fn check_joker_is_equal_than_other() {
        let jk = Card { t: CType::Joker };
        let qu = Card { t: CType::Queen };
        assert_eq!(true, jk == qu);
    }

    #[test]
    fn check_joker_is_smaller_than_other() {
        let jk = Card { t: CType::Joker };
        let tw = Card { t: CType::Two };
        assert_eq!(true, jk < tw);
    }

    #[test]
    fn check_queen_smaller_than_king() {
        let qu = Card { t: CType::Queen };
        let ki = Card { t: CType::King };
        assert_eq!(true, qu < ki);
    }

    #[test]
    fn check_queen_eq_queen() {
        let qu1 = Card { t: CType::Queen };
        let qu2 = Card { t: CType::Queen };
        assert_eq!(qu1, qu2);
    }

    #[test]
    fn check_transitivity_present() {
        let qu1 = Card { t: CType::Queen };
        let qu2 = Card { t: CType::Queen };
        let ki = Card { t: CType::King };
        assert_eq!(false, qu1 == qu2 && qu2 < ki && qu2 > ki);
    }
    #[test]
    fn check_hand_third_is_i_fourth_decides() {
        let first = "TTIQQ 123";
        let second = "TTIKK 23";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1 > hand2);
    }
}
