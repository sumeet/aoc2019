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

#[derive(Debug, Clone, Copy)]
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

fn mod_pow(mut base: usize, mut exp: usize, modulus: usize) -> usize {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

// returns previous index of card
fn apply_in_reverse(shuffle: Shuffle, deck_size: usize, target_index: usize) -> usize {
    match shuffle {
        // deal into new stack is: d - t - 1
        Shuffle::DealIntoNewStack => deck_size - target_index - 1,
        // cut is: (d + c + t) % d
        Shuffle::Cut(cut) => {
            let target_index = target_index as isize;
            let deck_size = deck_size as isize;
            ((deck_size + cut + target_index) % deck_size) as _
        }
        // deal with increment is: (t * ((i ** d-2) % d)) % d

        //badboy from https://stackoverflow.com/a/4798776
        Shuffle::DealWithIncrement(increment) => {
            (target_index * mod_pow(increment, deck_size - 2, deck_size)) % deck_size
        }
    }
}

#[aoc(day22, part2)]
fn solve_part2(input: &str) -> usize {
    let shuffles = input.trim().lines().map(parse_shuffle).collect_vec();
    //    let mut index = 4775;
    let mut index = 2020;
    for _ in 0..101741582076661usize {
        for shuffle in shuffles.iter().rev() {
            //        index = apply_in_reverse(shuffle, 10007, index);
            index = apply_in_reverse(*shuffle, 119315717514047, index);
        }
    }
    index
}

#[test]
fn new_stack() {
    let cards = gen_cards(10);
    let cards = deal_with_increment(cards, 3);
    println!("{:?}", cards);
}

#[test]
fn reverse_ops() {
    // CUTS (killer)

    // positive cuts
    // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] => cut 3 =>
    // [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]

    assert_eq!(apply_in_reverse(Shuffle::Cut(3), 10, 0), 3);
    assert_eq!(apply_in_reverse(Shuffle::Cut(3), 10, 1), 4);
    assert_eq!(apply_in_reverse(Shuffle::Cut(3), 10, 2), 5);
    assert_eq!(apply_in_reverse(Shuffle::Cut(3), 10, 3), 6);
    assert_eq!(apply_in_reverse(Shuffle::Cut(3), 10, 4), 7);
    assert_eq!(apply_in_reverse(Shuffle::Cut(3), 10, 5), 8);
    assert_eq!(apply_in_reverse(Shuffle::Cut(3), 10, 6), 9);
    assert_eq!(apply_in_reverse(Shuffle::Cut(3), 10, 7), 0);
    assert_eq!(apply_in_reverse(Shuffle::Cut(3), 10, 8), 1);
    assert_eq!(apply_in_reverse(Shuffle::Cut(3), 10, 9), 2);

    // negative cuts
    assert_eq!(apply_in_reverse(Shuffle::Cut(-4), 10, 0), 6);
    assert_eq!(apply_in_reverse(Shuffle::Cut(-4), 10, 1), 7);
    assert_eq!(apply_in_reverse(Shuffle::Cut(-4), 10, 2), 8);
    assert_eq!(apply_in_reverse(Shuffle::Cut(-4), 10, 3), 9);
    assert_eq!(apply_in_reverse(Shuffle::Cut(-4), 10, 4), 0);
    assert_eq!(apply_in_reverse(Shuffle::Cut(-4), 10, 5), 1);
    assert_eq!(apply_in_reverse(Shuffle::Cut(-4), 10, 6), 2);
    assert_eq!(apply_in_reverse(Shuffle::Cut(-4), 10, 7), 3);
    assert_eq!(apply_in_reverse(Shuffle::Cut(-4), 10, 8), 4);
    assert_eq!(apply_in_reverse(Shuffle::Cut(-4), 10, 9), 5);

    // deal into new stack
    // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] => deal into new stack =>
    // [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 0), 9);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 1), 8);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 2), 7);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 3), 6);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 4), 5);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 5), 4);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 6), 3);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 7), 2);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 8), 1);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 9), 0);

    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 9), 0);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 8), 1);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 7), 2);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 6), 3);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 5), 4);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 4), 5);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 3), 6);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 2), 7);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 1), 8);
    assert_eq!(apply_in_reverse(Shuffle::DealIntoNewStack, 10, 0), 9);
    // deal with increment
    // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10] => deal with increment 3 =>
    // [0, 4, 8, 1, 5, 9, 2, 6, 10, 3, 7]
    assert_eq!(apply_in_reverse(Shuffle::DealWithIncrement(3), 11, 0), 0);
    assert_eq!(apply_in_reverse(Shuffle::DealWithIncrement(3), 11, 1), 4);
    assert_eq!(apply_in_reverse(Shuffle::DealWithIncrement(3), 11, 2), 8);
    assert_eq!(apply_in_reverse(Shuffle::DealWithIncrement(3), 11, 3), 1);
    assert_eq!(apply_in_reverse(Shuffle::DealWithIncrement(3), 11, 4), 5);
    assert_eq!(apply_in_reverse(Shuffle::DealWithIncrement(3), 11, 5), 9);
    assert_eq!(apply_in_reverse(Shuffle::DealWithIncrement(3), 11, 6), 2);
    assert_eq!(apply_in_reverse(Shuffle::DealWithIncrement(3), 11, 7), 6);
    assert_eq!(apply_in_reverse(Shuffle::DealWithIncrement(3), 11, 8), 10);
    assert_eq!(apply_in_reverse(Shuffle::DealWithIncrement(3), 11, 9), 3);
    assert_eq!(apply_in_reverse(Shuffle::DealWithIncrement(3), 11, 10), 7);
}
