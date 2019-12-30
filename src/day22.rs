use itertools::Itertools;
use std::collections::VecDeque;

type Deck = VecDeque<usize>;

fn gen_cards(n: usize) -> Deck {
    (0..n).collect()
}

fn deal_into_new_stack(mut old_deck: Deck) -> Deck {
    let mut new_deck = Deck::new();
    while let Some(card) = old_deck.pop_front() {
        new_deck.push_front(card);
    }
    new_deck
}

fn cut_n_cards(mut old_deck: Deck, n: isize) -> Deck {
    if n >= 0 {
        for _ in 0..n {
            let card = old_deck.pop_front().unwrap();
            old_deck.push_back(card);
        }
    } else {
        let n = n.abs();
        for _ in (0..n).rev() {
            let card = old_deck.pop_back().unwrap();
            old_deck.push_front(card);
        }
    }
    old_deck
}

// 0 1 2 3 4 5
// at 4, and want to add 2
// should get 0
fn wrapping_add(x: usize, y: usize, num_cards: usize) -> usize {
    let sum = x + y;
    let result = if sum < num_cards {
        sum
    } else {
        sum - num_cards
    };
    result
}

fn deal_with_increment(mut old_deck: Deck, n: usize) -> Deck {
    let mut new_deck: Vec<Option<usize>> = old_deck.iter().map(|_| None).collect();
    let mut i = 0;
    let num_cards = old_deck.len();
    while let Some(card) = old_deck.pop_front() {
        while new_deck[i].is_some() {
            i = wrapping_add(i, 1, num_cards);
        }
        new_deck[i] = Some(card);
        i = wrapping_add(i, n, num_cards);
    }
    new_deck.into_iter().filter_map(|d| d).collect()
}

fn exec_shuffle(deck: Deck, shuffle: Shuffle) -> Deck {
    match shuffle {
        Shuffle::DealWithIncrement(n) => deal_with_increment(deck, n),
        Shuffle::DealIntoNewStack => deal_into_new_stack(deck),
        Shuffle::Cut(n) => cut_n_cards(deck, n),
    }
}

#[derive(Debug)]
enum Shuffle {
    DealWithIncrement(usize),
    DealIntoNewStack,
    Cut(isize),
}

fn parse_shuffle(input: &str) -> Shuffle {
    if input.starts_with("cut") {
        let num = input.chars().skip("cut ".len()).join("").parse().unwrap();
        Shuffle::Cut(num)
    } else if input.starts_with("deal with increment") {
        let num = input
            .chars()
            .skip("deal with increment ".len())
            .join("")
            .parse()
            .unwrap();
        Shuffle::DealWithIncrement(num)
    } else if input == "deal into new stack" {
        Shuffle::DealIntoNewStack
    } else {
        panic!(format!("couldn't parse {}", input))
    }
}

#[aoc(day22, part1)]
fn solve_part1(input: &str) -> usize {
    let mut deck = gen_cards(10007);
    for line in input.trim().lines() {
        let shuffle = parse_shuffle(line);
        println!("executing {:?}", shuffle);
        deck = exec_shuffle(deck, shuffle)
    }
    deck.iter()
        .enumerate()
        .filter_map(|(i, card)| {
            if *card == 2019 {
                return Some(i);
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

#[aoc(day22, part2)]
fn solve_part2(input: &str) -> usize {
    let mut deck = gen_cards(119315717514047);
    for line in input.trim().lines() {
        let shuffle = parse_shuffle(line);
        println!("executing {:?}", shuffle);
        deck = exec_shuffle(deck, shuffle)
    }
    deck.iter()
        .enumerate()
        .filter_map(|(i, card)| {
            if *card == 2019 {
                return Some(i);
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

#[test]
fn new_stack() {
    let cards = gen_cards(10);
    let cards = deal_with_increment(cards, 3);
    println!("{:?}", cards);
}
