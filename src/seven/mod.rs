use std::str::FromStr;
use std::cmp::{PartialOrd, PartialEq, Ordering};
use regex::Regex;
use std::collections::HashMap;

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug, Hash)]
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


#[derive(PartialOrd, Ord, Clone, Debug, Hash)]
struct Card {
    t: CType,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.t == other.t; //|| self.t == CType::Joker || other.t == CType::Joker;
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
                self.first.cmp(&other.first)
            } else if self.second != other.second {
                self.second.cmp(&other.second)
            } else if self.third != other.third {
                self.third.cmp(&other.third)
            } else if self.fourth != other.fourth {
                self.fourth.cmp(&other.fourth)
            } else if self.fifth != other.fifth {
                self.fifth.cmp(&other.fifth)
            } else {
                return Ordering::Equal;
            }
        } else {
            self.h_type.cmp(&other.h_type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
        let mut hashmap = HashMap::new();
        let mut h = Hand { first: fi.clone(), second: s.clone(), third: t.clone(), fourth: f.clone(), fifth: i.clone(), bid: bid, h_type: HType::HighCard };
        hashmap.insert(fi, 1);
        hashmap.entry(s).and_modify(|c| *c += 1).or_insert(1);
        hashmap.entry(t).and_modify(|c| *c += 1).or_insert(1);
        hashmap.entry(f).and_modify(|c| *c += 1).or_insert(1);
        hashmap.entry(i).and_modify(|c| *c += 1).or_insert(1);
        let mut vec: Vec<(Card, u32)> = hashmap.into_iter().collect();
        vec.sort_by(|a, b| a.0.cmp(&b.0));
        // println!("{:?}", vec);
        let len = vec.len();
        // println!("Vec len: {len}");
        // println!("vec[0].0.t {:?}", vec[0].0.t);
        match len {
            1 => h.h_type = HType::Five,
            2 => if vec[0].0.t == CType::Joker {
                    h.h_type = HType::Five;
                } else {
                    // println!("{:?} {:?}", vec[0],vec[1]);
                    if vec[0].1 == 1 || vec[1].1 == 1 {
                        // Four
                        h.h_type = HType::Four;
                    } else if vec[0].1 == 2 || vec[1].1 == 2 {
                        h.h_type = HType::FullHouse;
                    } else {
                        panic!();
                    }
                },
            3 => if vec[0].0.t == CType::Joker {
                    let joker_count = vec[0].1;
                    // println!("Joker count: {joker_count} {} {}", vec[1].1, vec[2].1);
                    if vec[1].1 > vec[2].1 {
                        // joker should expand to first
                        match vec[1].1 + joker_count {
                            3 => h.h_type = HType::Three,
                            4 => h.h_type = HType::Four,
                            _ => panic!("{}", vec[1].1 + joker_count),
                        }
                    } else if vec[2].1 > vec[1].1 {
                        // joker should expand to second
                        match vec[2].1 + joker_count {
                            3 => h.h_type = HType::Three,
                            4 => h.h_type = HType::Four,
                            _ => panic!("{}", vec[2].1 + joker_count),
                        }
                    } else {
                        // four of kind
                        if joker_count == 1 {
                            h.h_type = HType::FullHouse;
                        } else {
                            h.h_type = HType::Four;
                        }
                    }
                } else {
                    if vec[0].1 == 3 || vec[1].1 == 3 || vec[2].1 == 3 {
                        // Three
                        h.h_type = HType::Three;
                    } else {
                        // Two pair
                        h.h_type = HType::TwoPair;
                    }
                },
            4 => if vec[0].0.t == CType::Joker {
                    // Three
                    h.h_type = HType::Three;
                } else {
                    // Pair
                    h.h_type = HType::Pair;
                },
            5 => if vec[0].0.t == CType::Joker {
                    // pair
                    h.h_type = HType::Pair;
                } else {
                    h.h_type = HType::HighCard;
                },
            _ => panic!(),
        }
        Ok(h)
    }
}

impl Hand {
    fn is_five_of_kind(&self) -> bool {
        return self.h_type == HType::Five;
    }

    fn is_four_of_kind(&self) -> bool {
        return self.h_type == HType::Four;
    }

    fn is_full_house(&self) -> bool {
        return self.h_type == HType::FullHouse;
    }

    fn is_three_of_kind(&self) -> bool {
        return self.h_type == HType::Three;
    }

    fn is_two_pair(&self) -> bool {
        return self.h_type == HType::TwoPair;
    }

    fn is_pair(&self) -> bool {
        return self.h_type == HType::Pair;
    }

    fn is_high_card(&self) -> bool {
        return self.h_type == HType::HighCard;
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
        // println!("i: {} bid: {} type: {:?}", i, hand.bid, hand.h_type);
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
        // println!("i: {} bid: {} type: {:?}, hand {:?}", i, hand.bid, hand.h_type, hand);
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
        let line = "TTQQQ 123";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(hand.h_type, HType::FullHouse);
        assert_eq!(true, hand.is_full_house());
    }

    #[test]
    fn test_is_four_of_kind () {
        let line = "TTTTQ 123";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(true, hand.is_four_of_kind());
    }

    #[test]
    fn test_is_three_of_kind () {
        let line = "TTTKQ 123";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(true, hand.is_three_of_kind());
    }

    #[test]
    fn test_full_house_is_not_three_of_kind () {
        let line = "JJQQJ 123";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(false, hand.is_three_of_kind());
    }

    #[test]
    fn test_is_two_pair () {
        let line = "JJKQQ 123";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(true, hand.is_two_pair());
    }

    #[test]
    fn test_full_house_is_not_two_pair () {
        let line = "JKJKJ 123";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(false, hand.is_two_pair());
    }

    #[test]
    fn test_is_pair () {
        let line = "JTJ98 123";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(true, hand.is_pair());
    }

    #[test]
    fn test_is_not_pair () {
        let line = "JJTT8 123";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(false, hand.is_pair());
    }

    #[test]
    fn test_eq_hands() {
        let line1 = "JJTT2 123";
        let line2 = "JJTT2 123";
        let hand1 = Hand::from_str(line1).unwrap();
        let hand2 = Hand::from_str(line2).unwrap();
        assert_eq!(hand1, hand2);
    }

    #[test]
    fn test_five_bigger_full_house() {
        let line1 = "JJJJJ 32";
        let line2 = "QQKKQ 123";
        let hand1 = Hand::from_str(line1).unwrap();
        let hand2 = Hand::from_str(line2).unwrap();
        assert_eq!(true, hand1 > hand2);
    }
    #[test]
    fn test_high_cards1() {
        let line1 = "23456 123";
        let line2 = "34567 43";
        let hand1 = Hand::from_str(line1).unwrap();
        let hand2 = Hand::from_str(line2).unwrap();
        assert_eq!(true, hand1 < hand2);
    }
    #[test]
    fn test_high_cards2() {
        let line1 = "23456 0";
        let line2 = "23567 0";
        let hand1 = Hand::from_str(line1).unwrap();
        let hand2 = Hand::from_str(line2).unwrap();
        assert_eq!(true, hand1 < hand2);
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
        assert_eq!(true, jk_hand < ten_hand);
    }

    #[test]
    fn check_joker_creates_full_house() {
        let line = "TTIQQ 123";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(hand.h_type, HType::FullHouse);
        assert_eq!(true, hand.is_full_house());
    }

//    #[test]
//    fn check_joker_is_equal_than_other() {
//        let jk = Card { t: CType::Joker };
//        let qu = Card { t: CType::Queen };
//        assert_eq!(true, jk == qu);
//    }

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
        assert_eq!(true, hand1 < hand2);
    }

    #[test]
    fn test_five_jokers_is_five_of_kind() {
        let line = "IIIII 123";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(true, hand.is_five_of_kind());
    }

    #[test]
    fn test_jokers_in_high_cards() {
        let line = "AIKQT 123";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(true, hand.is_pair());
    }

    #[test]
    fn test_jokers_in_high_cards_creates_bigger_pair() {
        let first = "KIQAT 123";
        let second = "KKQT9 123";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1 < hand2);
    }

    #[test]
    fn test_jokers_expands_pair_to_three() {
        let first = "KKIQT 123";
        let second = "KKKQT 123";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1.is_three_of_kind());
        assert_eq!(true, hand1 < hand2);
    }
    #[test]
    fn test_equality_of_hands() {
        let first = "AAAAA 123";
        let second = "AAAAA 345";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1 == hand2);
        assert_eq!(false, hand1 > hand2);
        assert_eq!(false, hand1 < hand2);
    }

    #[test]
    fn test_joker_pretends_from_example() {
        let first = "QIIQ2 123";
        let second = "IKKK2 32";
        let hand1 = Hand::from_str(first).unwrap();
        assert_eq!(true, hand1.is_four_of_kind());
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1 > hand2);
    }

    #[test]
    fn test_first_part_example_second_ordering_rule() {
        let first = "33332 23";
        let second = "2AAAA 123";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1 > hand2);
    }

    #[test]
    fn test_first_part_example_second_ordering_rule_2() {
        let first = "77888 123";
        let second = "77788 12";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1 > hand2);
    }

    #[test]
    fn test_three_jokers_is_not_considered_full_house() {
        let line = "IAIAI 123";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(false, hand.is_full_house());
    }

    #[test]
    fn test_is_three_of_kind_with_joker() {
        let line = "AKQIQ 213";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(true, hand.is_three_of_kind());
    }

    #[test]
    fn test_2_jokers_is_not_considered_two_pair() {
        let line = "IAKQI 231";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(false, hand.is_two_pair());
    }

    #[test]
    fn test_highcard_smaller_than_pair() {
        let first = "AKQT9 21";
        let second = "KKAQT 21";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1 < hand2);
    }

    #[test]
    fn test_pair_smaller_than_two_pair() {
        let first = "KIAQT 20";
        let second = "KKAAQ 20";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1 < hand2);
    }

    #[test]
    fn test_two_pair_smaller_than_three() {
        let first = "KKAAQ 23";
        let second = "KIIAQ 23";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1 < hand2);
    }

    #[test]
    fn test_three_smaller_than_full_house() {
        let first = "2333T 23";
        let second = "23332 23";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1 < hand2);
    }
    #[test]
    fn test_full_house_smaller_than_four() {
        let first = "23332 23";
        let second = "23333 23";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1 < hand2);
    }
    #[test]
    fn test_four_smaller_than_five() {
        let first = "23333 23";
        let second = "33333 23";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1 < hand2);
    }
    #[test]
    fn test_four_jokers_is_five_of_kind() {
        let line = "KIIII 23";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(true, hand.is_five_of_kind());
    }
    #[test]
    fn test_is_high_card() {
        let line = "KQA23 12";
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(true, hand.is_high_card());
    }
    #[test]
    fn test_last_decides() {
        let first = "AAAAJ 12";
        let second = "AAAAK 12";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1 < hand2);
    }
    #[test]
    fn test_three_of_kinds() {
        let first = "QAAAK 12";
        let second = "AQAKA 12";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1 < hand2);
    }
    #[test]
    fn test_three_of_kinds_with_jokers() {
        let first = "QAIAK 12";
        let second = "AQIKI 12";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1 < hand2);
    }

    #[test]
    fn test_is_four_of_kind_with_jokers() {
        let first = "QAIII 12";
        let second = "QIIIA 12";
        let hand1 = Hand::from_str(first).unwrap();
        let hand2 = Hand::from_str(second).unwrap();
        assert_eq!(true, hand1.is_four_of_kind());
        assert_eq!(true, hand2.is_four_of_kind());
        assert_eq!(true, hand2 < hand1);
    }
}
