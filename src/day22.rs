// also used day22.py to do some of part2
use itertools::Itertools;
use std::collections::VecDeque;

type Deck = VecDeque<u128>;

fn gen_cards(n: u128) -> Deck {
    (0..n).collect()
}

fn deal_into_new_stack(mut old_deck: Deck) -> Deck {
    let mut new_deck = Deck::new();
    while let Some(card) = old_deck.pop_front() {
        new_deck.push_front(card);
    }
    new_deck
}

fn cut_n_cards(mut old_deck: Deck, n: i128) -> Deck {
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
fn wrapping_add(x: u128, y: u128, num_cards: u128) -> u128 {
    let sum = x + y;
    let result = if sum < num_cards {
        sum
    } else {
        sum - num_cards
    };
    result
}

fn deal_with_increment(mut old_deck: Deck, n: u128) -> Deck {
    let mut new_deck: Vec<Option<u128>> = old_deck.iter().map(|_| None).collect();
    let mut i = 0;
    let num_cards = old_deck.len();
    while let Some(card) = old_deck.pop_front() {
        while new_deck[i as usize].is_some() {
            i = wrapping_add(i, 1, num_cards as u128);
        }
        new_deck[i as usize] = Some(card);
        i = wrapping_add(i, n, num_cards as u128);
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
    DealWithIncrement(u128),
    DealIntoNewStack,
    Cut(i128),
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
        .filter_map(|(index, card)| {
            if *card == 2019 {
                return Some(index);
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
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
fn apply_in_reverse(shuffle: Shuffle, deck_size: u128, target_index: u128) -> u128 {
    match shuffle {
        // deal into new stack is: d - t - 1
        Shuffle::DealIntoNewStack => deck_size - target_index - 1,
        // cut is: (d + c + t) % d
        Shuffle::Cut(mut cut) => {
            let target_index = target_index as i128;
            let deck_size = deck_size as i128;
            if cut < 0 {
                cut = deck_size + cut;
            }
            ((cut + target_index) % deck_size) as _
        }
        // deal with increment is: (t * ((i ** d-2) % d)) % d

        //badboy from https://stackoverflow.com/a/4798776
        Shuffle::DealWithIncrement(increment) => {
            (target_index * mod_pow(increment, deck_size - 2, deck_size)) % deck_size
        }
    }
}

// 1. t = 2020
// 2. t = (c1 + 2020) % d
// 3. t = (((c1 + 2020) % d) * ((i1 ** d-2) % d)) % d
// 4. t = ((c1 + 2020) * (i1 ** d-2)) % d
// 4- t = ((c1 + 2020) * (i1 ** d-2)) % d
// 5. t = (c2 + (((c1 + 2020) * (i1 ** d-2)) % d)) % d
// ^ that is confirmed working. can we simplify it?
// YES WE CAN, tested the following in ipython:
// 6. (((c2 + (c1 + 2020) * (pow(i1, d-2,d))) % d)) % d
// ......
// 8. d - 7649 - 1 => 2357
// 9. (5785 + (d - 7649 - 1) % d)
//10.

fn gen_many_from_part1(input: &str) -> Vec<u128> {
    let mut deck = gen_cards(10007);
    for line in input.trim().lines() {
        let shuffle = parse_shuffle(line);
        deck = exec_shuffle(deck, shuffle)
    }

    let mut found_cards = vec![2019];
    for _ in 0..8 {
        deck.iter()
            .enumerate()
            .find(|(index, card)| {
                return if *card == found_cards.last().unwrap() {
                    found_cards.push(*index as u128);
                    true
                } else {
                    false
                };
            })
            .unwrap();
    }
    found_cards
}

// y = m (x) + b

#[aoc(day22, part2)]
fn solve_part2(input: &str) -> u128 {
    println!("{:?}", gen_many_from_part1(input));

    let shuffles = input.trim().lines().map(parse_shuffle).collect_vec();
    let deck_size = 119315717514047;
    let orig_index = 4775;

    // next = (m * index) + b
    let mut m: i128 = 1;
    let mut b: i128 = 0;

    let mut index = orig_index;
    for shuffle in shuffles.iter().rev() {
        // old solution
        index = apply_in_reverse(*shuffle, deck_size, index);

        // simplified solution with mx + b
        match shuffle {
            Shuffle::DealWithIncrement(increment) => {
                b *= mod_pow(*increment, deck_size - 2, deck_size) as i128;
                b %= deck_size as i128;
                m *= mod_pow(*increment, deck_size - 2, deck_size) as i128;
                m %= deck_size as i128;
            }
            Shuffle::Cut(mut cut) => {
                if cut < 0 {
                    cut = deck_size as i128 + cut;
                }
                b += cut;
            }
            Shuffle::DealIntoNewStack => {
                m *= -1;
                b = -b + -1;
            }
        }
        let new = absmod((m * orig_index as i128) + b, deck_size);
        if index != new as u128 {
            panic!("not equal");
        }
    }
    println!("y = mx + b, m = {}, b = {}", m, b);
    123
}

fn absmod(i: i128, m: u128) -> u128 {
    let m = m as i128;
    ((i % m + m) % m) as u128
}
// [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10] cut 3
// [3, 4, 5, 6, 7, 8, 9, 10, 0, 1, 2] shuffle into new deck
// [2, 1, 0, 10, 9, 8, 7, 6, 5, 4, 3]

#[test]
fn new_stack() {
    let cards = gen_cards(119315717514047);
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
