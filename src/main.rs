#![allow(dead_code)]

use core::fmt::{Debug, Display};
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::mem::swap;
use std::time::Duration;

fn problem1ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/1.in").unwrap();

    let mut sum_a = 0;
    let mut sum_b = 0;

    let mut line_start = 0;

    while line_start < data.len() {
        // println!("j={}, data[j]={:?}",j,data[j] as char);

        let mut first_a = Default::default();
        let mut last_a = Default::default();
        let mut first_b = Default::default();
        let mut last_b = Default::default();

        let mut found_b = |n| {
            if first_b == Default::default() {
                first_b = n;
            }
            last_b = n;
        };
        let mut found_a = |n| {
            if first_a == Default::default() {
                first_a = n;
            }
            last_a = n;
        };

        let mut i = line_start;
        while data[i] as char != '\n' {
            let c = data[i] as char;
            if c >= '1' && c <= '9' {
                found_a(c as i32 - '0' as i32);
                found_b(c as i32 - '0' as i32);
                i += 1;

                continue;
            }

            macro_rules! chars_to_int {
                ($c1:expr,$c2:expr,$c3:expr) => {
                    (($c1 as u32) << 16) + (($c2 as u32) << 8) + ($c3 as u32)
                };
                ($c1:expr,$c2:expr,$c3:expr,$c4:expr) => {
                    (($c1 as u32) << 24) + (($c2 as u32) << 16) + (($c3 as u32) << 8) + ($c4 as u32)
                };
            }

            if i + 3 < data.len() {
                // if do_print { println!("chars: {}, {}, {} ]", c, data[i+1] as char, data[i+2] as char); }
                let val = chars_to_int!(c, data[i + 1], data[i + 2]);

                // if do_print { println!("val: {}", val); }
                if val == chars_to_int!('o', 'n', 'e') {
                    found_b(1);
                    i += 2;
                    continue;
                }
                if val == chars_to_int!('t', 'w', 'o') {
                    found_b(2);
                    i += 2;
                    continue;
                }
                if val == chars_to_int!('s', 'i', 'x') {
                    found_b(6);
                    i += 3;
                    continue;
                }
                if i + 4 < data.len() {
                    let val = (val << 8) + data[i + 3] as u32;
                    if val == chars_to_int!('f', 'o', 'u', 'r') {
                        found_b(4);
                        i += 4;
                        continue;
                    }
                    if val == chars_to_int!('f', 'i', 'v', 'e') {
                        found_b(5);
                        i += 3;
                        continue;
                    }
                    if val == chars_to_int!('n', 'i', 'n', 'e') {
                        found_b(9);
                        i += 3;
                        continue;
                    }
                    if i + 5 < data.len() {
                        if val == chars_to_int!('t', 'h', 'r', 'e') && data[i + 4] as char == 'e' {
                            found_b(3);
                            i += 4;
                            continue;
                        }
                        if val == chars_to_int!('s', 'e', 'v', 'e') && data[i + 4] as char == 'n' {
                            found_b(7);
                            i += 4;
                            continue;
                        }
                        if val == chars_to_int!('e', 'i', 'g', 'h') && data[i + 4] as char == 't' {
                            found_b(8);
                            i += 4;
                            continue;
                        }
                    }
                }
            }

            // if c == 'o' && i+3 <= data.len() && data[i+1] as char == 'n' && data[i+2] as char == 'e' { found_b(1); i+=2; continue; }
            // if c == 't' && i+3 <= data.len() && data[i+1] as char == 'w' && data[i+2] as char == 'o' { found_b(2); i+=2; continue; }
            // if c == 't' && i+5 <= data.len() && data[i+1] as char == 'h' && data[i+2] as char == 'r' && data[i+3] as char == 'e' && data[i+4] as char == 'e' { found_b(3); i+=4; continue; }
            // if c == 'f' && i+4 <= data.len() && data[i+1] as char == 'o' && data[i+2] as char == 'u' && data[i+3] as char == 'r' { found_b(4); i+=4; continue; }
            // if c == 'f' && i+4 <= data.len() && data[i+1] as char == 'i' && data[i+2] as char == 'v' && data[i+3] as char == 'e' { found_b(5); i+=3; continue; }
            // if c == 's' && i+3 <= data.len() && data[i+1] as char == 'i' && data[i+2] as char == 'x' { found_b(6); i+=3; continue; }
            // if c == 's' && i+5 <= data.len() && data[i+1] as char == 'e' && data[i+2] as char == 'v' && data[i+3] as char == 'e' && data[i+4] as char == 'n' { found_b(7); i+=4; continue; }
            // if c == 'e' && i+5 <= data.len() && data[i+1] as char == 'i' && data[i+2] as char == 'g' && data[i+3] as char == 'h' && data[i+4] as char == 't' { found_b(8); i+=4; continue; }
            // if c == 'n' && i+4 <= data.len() && data[i+1] as char == 'i' && data[i+2] as char == 'n' && data[i+3] as char == 'e' { found_b(9); i+=3; continue; }
            i += 1;
        }

        sum_b += first_b * 10 + last_b;
        sum_a += first_a * 10 + last_a;

        line_start = i + 1; // past 1 of line end
    }

    if do_print {
        println!("Problem 1 A: {}", sum_a);
        println!("Problem 1 B: {}", sum_b);
    }
}

fn problem2ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/2.in").unwrap();

    let mut id_sum = 0;
    let mut power_sum = 0;

    let max_reds = 12;
    let max_greens = 13;
    let max_blues = 14;

    let mut check = |r, g, b, id| {
        // if do_print {
        //     println!("Got R={},G={},B={} at ID={}",r,g,b,id);
        // }
        if r <= max_reds && g <= max_greens && b <= max_blues {
            // if do_print { println!(" OK {}",id); }
            id_sum += id;
        }
    };

    let mut word_begin = 0;
    let mut words_on_line = 0;

    let mut last_num = 0;
    let mut reds = 0;
    let mut greens = 0;
    let mut blues = 0;

    let mut id = 1;
    for i in 0..data.len() {
        if data[i] as char == ' ' || data[i] as char == '\n' {
            let word = &data[word_begin..i];
            word_begin = i + 1;
            words_on_line += 1;

            if words_on_line <= 2 {
                continue;
            } // Skip Game and ID:

            // if do_print {
            //     println!("word={}", word.iter().map(|b| *b as char).collect::<String>());
            // }

            if last_num == 0 {
                for digit in word {
                    last_num = last_num * 10 + *digit as i32 - '0' as i32;
                }
                // if do_print {
                //     println!(" Parsed num: {}", last_num);
                // }
            } else {
                match word[0] as char {
                    'r' => reds = max(reds, last_num),
                    'g' => greens = max(greens, last_num),
                    'b' => blues = max(blues, last_num),
                    _ => panic!("Got {:?} as word :(", word),
                }
                last_num = 0;
            }
        }

        if data[i] as char == '\n' {
            let power = reds * greens * blues;
            power_sum += power;

            check(reds, greens, blues, id);

            words_on_line = 0;
            word_begin = i + 1;

            last_num = 0;
            reds = 0;
            greens = 0;
            blues = 0;

            id += 1;
            continue;
        }
    }

    if do_print {
        println!("Problem 2 A: {}", id_sum);
        println!("Problem 2 B: {}", power_sum);
    }
}

fn problem3ab(do_print: bool, folder: &str) {
    let flat_map = std::fs::read(folder.to_owned() + "/3.in").unwrap();

    let mut line_length = 0;

    for i in 0..flat_map.len() {
        let c = flat_map[i] as char;

        if c == '\n' {
            line_length = i as i32 + 1;
            break;
        }
    }

    #[derive(Debug)]
    enum Star {
        Alone,
        OneNumber(i32),
        Gear(i32, i32),
        TooMany,
    }

    // if do_print {
    //     println!("Line length is {}", line_length);
    // }

    let l = flat_map.len();
    let mut next_to_symbol = vec![false; l];
    let mut owner_star = vec![0; l];
    let mut stars = Vec::with_capacity(line_length as usize);

    let is_symbol = |c_code| {
        let c = c_code as char;
        return (c < '0' || c > '9') && c != '.' && c != '\n';
    };

    let adjacent_offsets = [
        1,
        line_length,
        line_length - 1,
        line_length + 1,
        -1,
        -line_length,
        -line_length + 1,
        -line_length - 1,
    ];

    for i in 0..l {
        if is_symbol(flat_map[i]) {
            for o in adjacent_offsets {
                if i as i32 >= o && (i as i32) < l as i32 + o {
                    next_to_symbol[(i as i32 - o) as usize] = true;
                }
            }
        }
    }

    for i in 0..l {
        if flat_map[i] as char == '*' {
            stars.push(Star::Alone);
            let star_id = stars.len();

            for o in adjacent_offsets {
                if i as i32 >= o && (i as i32) < l as i32 + o {
                    owner_star[(i as i32 - o) as usize] = star_id;
                }
            }
        }
    }

    // if do_print {
    //     for i in 0..l {
    //         if flat_map[i] as char == '\n' {
    //             println!("{}", next_to_symbol[i] as i32);
    //         } else {
    //             print!("{}", next_to_symbol[i] as i32);
    //         }
    //     }
    // }

    // if do_print {
    //     for i in 0..l {
    //         if flat_map[i] as char == '\n' {
    //             println!("{}", owner_star[i] as i32);
    //         } else {
    //             print!("{}", owner_star[i] as i32);
    //         }
    //     }
    // }

    let mut sum_a = 0;
    let mut num = 0;
    let mut is_adjacent = false;
    let mut linked_star_id = 0;

    for i in 0..l {
        let c = flat_map[i] as char;
        if c >= '0' && c <= '9' {
            num = num * 10 + (c as i32 - '0' as i32);
            is_adjacent = is_adjacent || next_to_symbol[i];

            // if owner_star[i] != 0 {
            //     // if linked_star_id != 0 && linked_star_id != owner_star[i] {
            //     //     panic!("WARNING: Found a number with two adjacent stars at {}", i);
            //     // }
            //     linked_star_id = owner_star[i];
            // }

            // Assume max 1 star / number
            linked_star_id |= owner_star[i];
        } else {
            // if do_print {
            //     if num > 0 {
            //         println!("Found a number: {}, is it adjacent: {}", num, is_adjacent);
            //     }
            // }
            if is_adjacent {
                sum_a += num;
                if linked_star_id > 0 {
                    let new_star = match stars[linked_star_id - 1] {
                        Star::Alone => Star::OneNumber(num),
                        Star::OneNumber(k) => Star::Gear(k, num),
                        _ => Star::TooMany,
                    };

                    // if do_print {
                    //     println!("Updated star {} to {:?}", linked_star_id, new_star);
                    // }

                    stars[linked_star_id - 1] = new_star;
                }
            }

            num = 0;
            is_adjacent = false;
            linked_star_id = 0;
        }
    }

    let mut sum_b = 0;

    for star in stars {
        if let Star::Gear(a, b) = star {
            sum_b += a * b;
        }
    }

    if do_print {
        println!("Problem 3 A: {}", sum_a);
        println!("Problem 3 B: {}", sum_b);
    }
}

fn problem4ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/4.in").unwrap();

    let mut sum_a = 0;

    let mut wins = Vec::with_capacity(210);

    let mut i = 0;

    while i < data.len() {
        while data[i] as char != ':' {
            i = i + 1;
        }
        i = i + 1; // colon (:)
        i = i + 1; // space ( )

        // if do_print { println!("Card counts: {:?}", card_counts); }

        let mut have_winners = 0;

        let mut winners: u128 = 0;

        while data[i] as char != '|' {
            let mut num = 0;

            if data[i] as char != ' ' {
                num += 10 * (data[i] as u128 - '0' as u128);
            }
            num += data[i + 1] as u128 - '0' as u128;

            winners |= 1 << num;

            i = i + 3;
        }
        i = i + 1; // pipe (|)

        while data[i] as char != '\n' {
            let mut num = 0;

            if data[i + 1] as char != ' ' {
                num += 10 * (data[i + 1] as u128 - '0' as u128);
            }
            num += data[i + 2] as u128 - '0' as u128;

            if winners & (1 << num) != 0 {
                have_winners += 1;
            }

            i = i + 3;
        }
        i = i + 1; // linefeed (\n)

        // if do_print {
        //     println!(" -> Got {} winning numbers", have_winners);
        // }

        if have_winners > 0 {
            sum_a += 1 << (have_winners - 1);
        }

        wins.push(have_winners);
    }

    // if do_print { println!("wins is {:?}",wins); }

    let mut card_count = vec![1; wins.len()];

    for i in 0..wins.len() {
        for j in i + 1..i + wins[i] + 1 {
            if j < wins.len() {
                card_count[j] += card_count[i];
            }
        }
    }

    // for i in 0..wins.len() {
    //     for j in i+1..min(wins.len(), i+wins[i]+1) {
    //         card_count[j] += card_count[i];
    //     }
    // }

    // card_counts.resize(line_id, 0);
    // if line_id+have_winners+1 >= card_counts.len() {
    //             card_counts.resize(line_id+have_winners+1, 1);
    //         }

    //         for j in line_id+1..=line_id+have_winners {
    //             card_counts[j] += card_counts[line_id];
    //         }

    // if do_print { println!("Final card counts: {:?}", card_counts); }

    if do_print {
        println!("Problem 4 A: {}", sum_a);
        println!("Problem 4 B: {}", card_count.into_iter().sum::<usize>());
    }
}

#[derive(Clone)]
struct Range {
    begin: u32,
    end: u32,
}

impl Range {
    pub fn intersects(&self, other: &Range) -> bool {
        max(self.begin, other.begin) < min(self.end, other.end)
    }

    pub fn new() -> Range {
        Range { begin: 0, end: 0 }
    }

    pub fn offset(self, delta: u32) -> Range {
        Range {
            begin: self.begin + delta,
            end: self.end + delta,
        }
    }

    pub fn empty(&self) -> bool {
        self.end <= self.begin
    }

    pub fn contains(&self, other: &Range) -> bool {
        self.begin <= other.begin && self.end >= other.end
    }

    // assumes intersection and no containment
    pub fn split(&mut self, other: &Range) -> Range {
        if self.begin <= other.begin {
            let r = Range {
                begin: other.begin,
                end: self.end,
            };
            self.end = other.begin;
            return r;
        }

        let r = Range {
            begin: self.begin,
            end: other.end,
        };
        self.begin = other.end;
        return r;
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.begin, self.end)
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.begin, self.end)
    }
}

fn problem5ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/5.in").unwrap();

    let mut i = 0;
    while data[i] as char != ':' {
        i += 1;
    }
    i += 1; // colon (:)

    // part 1
    {
        let mut i = i;

        let mut seeds = Vec::with_capacity(20);

        while data[i] as char != '\n' {
            i += 1; // space ( )

            let mut num = 0;
            while data[i] as char >= '0' && data[i] as char <= '9' {
                num = num * 10 + data[i] as u32 - '0' as u32;
                i += 1;
            }

            seeds.push(num);
        }
        i += 1; // linefeed (\n)

        while i < data.len() {
            while data[i] as char != ':' {
                i += 1;
            }
            i += 2; // colon (:) + linefeed (\n)

            // if do_print { println!("i after : finding = {}",i); }

            let mut new_seeds = seeds.clone();

            // loop until empty line
            while i < data.len() && data[i] as char != '\n' {
                let mut a = 0;
                let mut b = 0;
                let mut c = 0;
                while data[i] as char != ' ' {
                    a = a * 10 + data[i] as u32 - '0' as u32;
                    i += 1;
                }
                i += 1; // space ( )
                while data[i] as char != ' ' {
                    b = b * 10 + data[i] as u32 - '0' as u32;
                    i += 1;
                }
                i += 1; // space ( )
                while data[i] as char != '\n' {
                    c = c * 10 + data[i] as u32 - '0' as u32;
                    i += 1;
                }
                i += 1; // linefeed (\n)

                // if do_print { println!("a,b,c = {} {} {}",a,b,c); }

                for j in 0..seeds.len() {
                    if seeds[j] >= b && seeds[j] < b + c {
                        new_seeds[j] = a + seeds[j] - b;
                    }
                }
            }
            i += 1; // linefeed (\n)

            seeds = new_seeds;

            // if do_print { println!("Seeds = {:?}", seeds); }
            // if do_print { println!(); }
        }

        if do_print {
            println!("Problem 5 A: {}", seeds.into_iter().min().unwrap());
        }
    }

    // part 2
    let mut seed_ranges = Vec::with_capacity(40);

    while data[i] as char != '\n' {
        i += 1; // space ( )

        let mut begin = 0;
        while data[i] as char != ' ' {
            begin = begin * 10 + data[i] as u32 - '0' as u32;
            i += 1;
        }
        i += 1; // space ( )

        let mut len = 0;
        while data[i] as char >= '0' && data[i] as char <= '9' {
            len = len * 10 + data[i] as u32 - '0' as u32;
            i += 1;
        }

        seed_ranges.push(Range {
            begin,
            end: begin + len,
        });
    }
    i += 1; // linefeed (\n)

    // if do_print {
    //     println!("seed_ranges = {:?}", seed_ranges);
    // }

    while i < data.len() {
        while data[i] as char != ':' {
            i += 1;
        }
        i += 2; // colon (:) + linefeed (\n)

        // if do_print { println!("i after : finding = {}",i); }

        let mut new_seed_ranges = Vec::new();

        let scan_number_until = |delim: char, i: &mut usize| {
            let mut num = 0;
            while data[*i] as char != delim {
                num = num * 10 + data[*i] as u32 - '0' as u32;
                *i += 1;
            }
            *i += 1; // delim (space or LF)

            return num;
        };

        // loop until empty line
        while i < data.len() && data[i] as char != '\n' {
            let b = scan_number_until(' ', &mut i);
            let a = scan_number_until(' ', &mut i);
            let c = scan_number_until('\n', &mut i);

            // if do_print { println!("a,b,c = {} {} {}",a,b,c); }

            let cut_range = Range {
                begin: a,
                end: a + c,
            };

            let mut j = 0;
            while j < seed_ranges.len() {
                let mut r = seed_ranges[j].clone();

                if !r.empty() && r.intersects(&cut_range) {
                    // if do_print {
                    //     println!("Intersection: {:?} and {:?}", r, cut_range);
                    // }

                    if cut_range.contains(&r) {
                        // if do_print {
                        //     println!(
                        //         "r is in cut_range, adding {} -> {}",
                        //         r,
                        //         r.clone().offset(b - a)
                        //     );
                        // }

                        new_seed_ranges.push(r.clone().offset(b - a));
                        r = Range::new();
                    } else if r.contains(&cut_range) {
                        let left_sub = Range {
                            begin: r.begin,
                            end: cut_range.begin,
                        };
                        let right_sub = Range {
                            begin: cut_range.end,
                            end: r.end,
                        };

                        // if do_print {
                        //     println!(
                        //         "cut_range is in r :( adding {} -> {} and keeping {} and {}",
                        //         cut_range.clone(),
                        //         cut_range.clone().offset(b - a),
                        //         left_sub,
                        //         right_sub
                        //     );
                        // }

                        new_seed_ranges.push(cut_range.clone().offset(b - a));
                        r = left_sub;
                        seed_ranges.push(right_sub);
                    } else {
                        let new_range = r.split(&cut_range);
                        // if do_print {
                        //     println!(
                        //         "r is not entirely in cut_range adding {} -> {} keeping {}",
                        //         new_range,
                        //         new_range.clone().offset(b - a),
                        //         r
                        //     );
                        // }

                        new_seed_ranges.push(new_range.offset(b - a));
                    }
                }

                seed_ranges[j] = r;
                j += 1;
            }

            // if do_print {
            //     println!("new_seed_ranges: {:?}", new_seed_ranges);
            // }
        }
        i += 1; // linefeed (\n)

        for r in seed_ranges.into_iter() {
            if !r.empty() {
                new_seed_ranges.push(r);
            }
        }

        seed_ranges = new_seed_ranges;

        // if do_print {
        //     println!("Seed ranges len = {}", seed_ranges.len());
        // }

        // if do_print {
        //     println!("Seed ranges = {:?}", seed_ranges);
        // }
        // if do_print {
        //     println!();
        // }
    }

    let minimum = seed_ranges
        .into_iter()
        .filter(|r| !r.empty())
        .map(|r| r.begin)
        .min()
        .unwrap();

    if do_print {
        println!("Problem 5 B: {}", minimum);
    }
}

fn problem6ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/6.in").unwrap();

    let mut times = Vec::new();
    let mut distances = Vec::new();
    let mut num = 0;
    let mut doing_times = true;

    let mut time_b: i64 = 0;
    let mut distance_b: i64 = 0;

    for b in data {
        let c = b as char;

        if c >= '0' && c <= '9' {
            num = num * 10 + c as i32 - '0' as i32;
            if doing_times {
                time_b = time_b * 10 + c as i64 - '0' as i64;
            } else {
                distance_b = distance_b * 10 + c as i64 - '0' as i64;
            }
        }

        if (c == ' ' || c == '\n') && num > 0 {
            if doing_times {
                times.push(num);
                if c == '\n' {
                    doing_times = false;
                }
            } else {
                distances.push(num);
            }
            num = 0;
        }
    }

    // (t-x)*x > d
    // -x**2 + t*x - d > 0
    // x = (-t +/- sqrt(t**2 - 4*-1*-d)) / 2*-1
    // x = (t +/- sqrt(t**2 - 4*d)) / 2

    let solve_case = |t: f64, d: f64| -> i32 {
        let sqrt_d = (t * t - 4.0 * d).sqrt();
        let x0 = (t - sqrt_d) / 2.0;
        let x1 = (t + sqrt_d) / 2.0;

        let t0 = x0.floor() as i32 + 1;
        let t1 = x1.ceil() as i32 - 1;

        let ts = t1 - t0 + 1;

        // if do_print {
        //     println!("x0={}, x1={}, t0={}, t1={}, ts={}", x0, x1, t0, t1, ts);
        // }

        return ts;
    };

    let mut prod = 1;

    for i in 0..times.len() {
        let t = times[i] as f64;
        let d = distances[i] as f64;

        prod *= solve_case(t, d);
    }

    let solution_b = solve_case(time_b as f64, distance_b as f64);

    if do_print {
        println!("Problem 6 A: {}", prod);
        println!("Problem 6 B: {}", solution_b);
    }
}

fn problem7ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/7.in").unwrap();

    let card_value = |card: u8| -> u8 {
        let card = card as char;

        if card <= '9' && card >= '2' {
            return card as u8 - '2' as u8 + 1;
        }

        match card as char {
            'T' => 9,
            'J' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => 0,
        }
    };

    let mut all_hands_a: [Vec<_>; 7] = Default::default();
    let mut all_hands_b: [Vec<_>; 7] = Default::default();

    let mut i = 0;
    while i < data.len() {
        let power_a;
        let power_b;
        {
            let cv0 = card_value(data[i + 0]) as usize;
            let cv1 = card_value(data[i + 1]) as usize;
            let cv2 = card_value(data[i + 2]) as usize;
            let cv3 = card_value(data[i + 3]) as usize;
            let cv4 = card_value(data[i + 4]) as usize;

            power_a = ((cv0 as u32) << 16)
                + ((cv1 as u32) << 12)
                + ((cv2 as u32) << 8)
                + ((cv3 as u32) << 4)
                + ((cv4 as u32) << 0);

            let cv0 = if cv0 == 10 { 0 } else { cv0 };
            let cv1 = if cv1 == 10 { 0 } else { cv1 };
            let cv2 = if cv2 == 10 { 0 } else { cv2 };
            let cv3 = if cv3 == 10 { 0 } else { cv3 };
            let cv4 = if cv4 == 10 { 0 } else { cv4 };

            power_b = ((cv0 as u32) << 16)
                + ((cv1 as u32) << 12)
                + ((cv2 as u32) << 8)
                + ((cv3 as u32) << 4)
                + ((cv4 as u32) << 0);
        }

        let o0 = 1
            + (data[i + 0] == data[i + 1]) as u32
            + (data[i + 0] == data[i + 2]) as u32
            + (data[i + 0] == data[i + 3]) as u32
            + (data[i + 0] == data[i + 4]) as u32;

        let o1 = 1
            + (data[i + 1] == data[i + 0]) as u32
            + (data[i + 1] == data[i + 2]) as u32
            + (data[i + 1] == data[i + 3]) as u32
            + (data[i + 1] == data[i + 4]) as u32;

        let o2 = 1
            + (data[i + 2] == data[i + 0]) as u32
            + (data[i + 2] == data[i + 1]) as u32
            + (data[i + 2] == data[i + 3]) as u32
            + (data[i + 2] == data[i + 4]) as u32;

        let o3 = 1
            + (data[i + 3] == data[i + 0]) as u32
            + (data[i + 3] == data[i + 2]) as u32
            + (data[i + 3] == data[i + 1]) as u32
            + (data[i + 3] == data[i + 4]) as u32;

        let o4 = 1
            + (data[i + 4] == data[i + 0]) as u32
            + (data[i + 4] == data[i + 2]) as u32
            + (data[i + 4] == data[i + 3]) as u32
            + (data[i + 4] == data[i + 1]) as u32;

        let js = (data[i + 0] == 'J' as u8) as u32
            + (data[i + 1] == 'J' as u8) as u32
            + (data[i + 2] == 'J' as u8) as u32
            + (data[i + 3] == 'J' as u8) as u32
            + (data[i + 4] == 'J' as u8) as u32;

        let mut freqs = [0; 6];
        freqs[o0 as usize] += 1;
        freqs[o1 as usize] += 1;
        freqs[o2 as usize] += 1;
        freqs[o3 as usize] += 1;
        freqs[o4 as usize] += 1;

        const FIVE_OF_A_KIND: usize = 6;
        const FOUR_OF_A_KIND: usize = 5;
        const FULL_HOUSE: usize = 4;
        const THREE_OF_A_KIND: usize = 3;
        const TWO_PAIR: usize = 2;
        const ONE_PAIR: usize = 1;
        const HIGH_CARD: usize = 0;

        let strength_a = if freqs[5] == 5 {
            FIVE_OF_A_KIND
        } else if freqs[4] == 4 {
            FOUR_OF_A_KIND
        } else if freqs[3] == 3 && freqs[2] == 2 {
            FULL_HOUSE
        } else if freqs[3] == 3 {
            THREE_OF_A_KIND
        } else if freqs[2] == 4 {
            TWO_PAIR
        } else if freqs[2] == 2 {
            ONE_PAIR
        } else {
            HIGH_CARD
        };

        let strength_b = if js == 0 {
            strength_a
        } else if strength_a >= FULL_HOUSE {
            FIVE_OF_A_KIND
        } else if strength_a == THREE_OF_A_KIND {
            FOUR_OF_A_KIND
        } else if strength_a == ONE_PAIR {
            THREE_OF_A_KIND
        } else if strength_a == HIGH_CARD {
            ONE_PAIR
        } else {
            if js == 2 {
                FOUR_OF_A_KIND
            } else {
                FULL_HOUSE
            }
        };

        i += 6;

        let mut bet = 0;

        while data[i] as char != '\n' {
            bet = bet * 10 + data[i] as u32 - '0' as u32;
            i += 1;
        }
        i += 1; // linefeed \n

        all_hands_a[strength_a].push((power_a << 12) + bet);
        all_hands_b[strength_b].push((power_b << 12) + bet);

        // if do_print {
        //     println!("Hand: {:?}, Occurrences: {:?}, Power: {:#020b}, Strength: {}, Bet: {}",
        //         hand.iter().map(|u|*u as char).collect::<String>(),
        //         occurrences,
        //         power,
        //         strength,
        //         bet);
        // }
    }

    // if do_print {
    //     println!("{:?}", all_hands_a);
    //     println!("{:?}", all_hands_b);
    // }

    // if do_print {
    //     println!("{:?}", all_hands_a.iter_mut().map(|h|h.len()).collect::<Vec<_>>());
    // }

    let mut sum_a = 0;

    let mut multiplier = 1u32;
    for hands in all_hands_a.iter_mut() {
        hands.sort();

        for bet_and_power in hands {
            let bet = *bet_and_power & 0xFFFu32;
            sum_a += bet * multiplier;
            multiplier += 1;
        }
    }

    let mut sum_b = 0;

    let mut multiplier = 1u32;
    for hands in all_hands_b.iter_mut() {
        hands.sort();

        for bet_and_power in hands {
            let bet = *bet_and_power & 0xFFFu32;
            sum_b += bet * multiplier;
            multiplier += 1;
        }
    }

    if do_print {
        println!("Problem 7 A: {}", sum_a);
        println!("Problem 7 B: {}", sum_b);
    }
}

fn problem8ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/8.in").unwrap();

    let mut bitvec: Vec<u8> = Vec::new();

    let mut i = 0;
    while data[i] as char != '\n' {
        bitvec.push((data[i] as char == 'L') as _);
        i += 1;
    }
    i += 2;

    macro_rules! read_id {
        ($arr:expr,$i:expr) => {
            ($arr[$i + 0] as usize - 'A' as usize) * 26 * 26
                + ($arr[$i + 1] as usize - 'A' as usize) * 26
                + ($arr[$i + 2] as usize - 'A' as usize)
        };
    }

    #[allow(unused_macros)]
    macro_rules! val_into_str {
        ($value:expr) => {
            [
                ($value / 26 / 26 + 'A' as usize) as u8 as char,
                (($value / 26) % 26 + 'A' as usize) as u8 as char,
                (($value) % 26 + 'A' as usize) as u8 as char,
            ]
            .into_iter()
            .collect::<String>()
        };
    }

    let mut ids = [Default::default(); 26 * 26 * 26];
    let node_count;

    {
        let mut j = i;
        let mut next_id: u32 = 0;

        while j + 16 < data.len() {
            let a = read_id!(data, j);
            j += 17;

            ids[a] = next_id;
            // if do_print { println!("{} got id {}", val_into_str!(a), next_id); }

            next_id += 1;
        }

        node_count = next_id as usize;
    }

    #[derive(Debug, Eq, PartialEq)]
    enum NodeType {
        Start,
        End,
        Normal,
    }

    let mut children = Vec::with_capacity(node_count);
    let mut node_types = Vec::with_capacity(node_count);

    {
        let mut j = i;

        while j + 16 < data.len() {
            let a = read_id!(data, j);
            let b = read_id!(data, j + 7);
            let c = read_id!(data, j + 12);
            j += 17;

            let ntype = match a % 26 {
                0 => NodeType::Start,
                25 => NodeType::End,
                _ => NodeType::Normal,
            };

            // if do_print {
            //     println!("A={}, type={:?}", val_into_str!(a), ntype);
            // }

            let b = ids[b];
            let c = ids[c];
            children.push((b, c));
            node_types.push(ntype);
        }
    }

    fn gcd(mut m: u64, mut n: u64) -> u64 {
        while m != 0 {
            let old_m = m;
            m = n % m;
            n = old_m;
        }
        n
    }

    let mut least_total_dist = 1;
    let mut answer_a = 0;

    for i in 0..node_count {
        if node_types[i] == NodeType::Start {
            let mut t = 0;
            let mut p = i;
            while node_types[p] != NodeType::End {
                let left = bitvec[t % bitvec.len()] > 0;
                let kids = &children[p];
                p = if left { kids.0 } else { kids.1 } as usize;
                t += 1;
            }

            least_total_dist = least_total_dist * t as u64 / gcd(least_total_dist, t as u64);

            if i == ids[0] as usize {
                answer_a = t;
            }

            // if do_print {
            //     println!("Start: {} len: {}", i, t);
            // }
        }
    }

    if do_print {
        println!("Problem 8 A: {}", answer_a);
        println!("Problem 8 B: {}", least_total_dist);
    }
}

fn problem9ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/9.in").unwrap();

    let mut extrapolated = Vec::with_capacity(200);
    let mut extrapolated_b = Vec::with_capacity(200);

    let mut i = 0;
    while i < data.len() {
        let mut nums = Vec::with_capacity(40);
        let mut num = 0;
        let mut neg = false;

        while data[i] as char != '\n' {
            if data[i] as char == ' ' {
                nums.push(num * if neg { -1 } else { 1 });
                num = 0;
                neg = false;
            } else if data[i] as char == '-' {
                neg = true;
            } else {
                num = num * 10 + data[i] as i64 - '0' as i64;
            }
            i += 1;
        }
        i += 1;

        nums.push(num * if neg { -1 } else { 1 });

        // if do_print {
        //     println!("Line contains numbers: {:?}", nums);
        // }

        let mut nums_b = nums.clone();

        let len = nums.len();
        let mut n = len - 1;
        while n >= 1 {
            for j in 0..n {
                nums[j] = nums[j + 1] - nums[j];
                nums_b[len - 1 - j] = nums_b[len - 1 - j] - nums_b[len - 2 - j];
            }

            // if do_print {
            //     println!(" -> {:?}", &nums[..n]);
            // }

            // if do_print {
            //     println!(" <- {:?}", &nums_b[len-n..]);
            // }

            n -= 1;
        }

        // if do_print {
        //     println!("Num_b is {:?}", nums_b);
        // }

        let mut n = len - 2;
        loop {
            nums_b[n] = nums_b[n] - nums_b[n + 1];
            if n == 0 {
                break;
            }
            n -= 1;
        }

        // if do_print {
        //     println!("In the end nums_b is {:?}", nums_b);
        //     println!(" <= {}", nums_b[0]);
        // }

        extrapolated.push(nums.into_iter().sum::<i64>());
        extrapolated_b.push(nums_b[0]);
    }

    let answer_a = extrapolated.into_iter().sum::<i64>();
    let answer_b = extrapolated_b.into_iter().sum::<i64>();

    if do_print {
        println!("Problem 9 A: {}", answer_a);
        println!("Problem 9 B: {}", answer_b);
    }
}

fn problem10ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/10.in").unwrap();

    let mut w = 0;
    while data[w] as char != '\n' {
        w += 1;
    }
    let w = w;

    let h = data.len() / (w + 1);

    // if do_print {
    //     println!("WxH = {}x{}", w, h);
    // }

    #[derive(Debug, Copy, Clone)]
    #[repr(u32)]
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    struct Location(i32, i32);

    let at = |x, y| data[y * (w + 1) + x] as char;

    let mut s = Location(0, 0);

    's_finding: for i in 0..w {
        for j in 0..h {
            if at(i, j) == 'S' {
                s = Location(i as i32, j as i32);
                break 's_finding;
            }
        }
    }
    let s = s;

    // if do_print {
    //     println!("Found S at {:?}", s);
    // }

    let mut cursors = Vec::new();

    let at_safe = |x: i32, y: i32| {
        if x < 0 || x >= w as i32 || y < 0 || y >= h as i32 {
            '.'
        } else {
            data[y as usize * (w + 1) + x as usize] as char
        }
    };

    match at_safe(s.0 - 1, s.1) {
        '-' | 'L' | 'F' => cursors.push((Location(s.0 - 1, s.1), Direction::Left)),
        _ => (),
    };
    match at_safe(s.0 + 1, s.1) {
        '-' | 'J' | '7' => cursors.push((Location(s.0 + 1, s.1), Direction::Right)),
        _ => (),
    };
    match at_safe(s.0, s.1 - 1) {
        '|' | 'F' | '7' => cursors.push((Location(s.0, s.1 - 1), Direction::Up)),
        _ => (),
    };
    match at_safe(s.0, s.1 + 1) {
        '|' | 'L' | 'J' => cursors.push((Location(s.0, s.1 + 1), Direction::Down)),
        _ => (),
    };

    let step = |state: (Location, Direction)| {
        let Location(i, j) = state.0;

        match (state.1, at(i as usize, j as usize)) {
            (Direction::Left, '-') => (Location(i - 1, j), Direction::Left),
            (Direction::Left, 'F') => (Location(i, j + 1), Direction::Down),
            (Direction::Left, 'L') => (Location(i, j - 1), Direction::Up),

            (Direction::Right, '-') => (Location(i + 1, j), Direction::Right),
            (Direction::Right, 'J') => (Location(i, j - 1), Direction::Up),
            (Direction::Right, '7') => (Location(i, j + 1), Direction::Down),

            (Direction::Down, '|') => (Location(i, j + 1), Direction::Down),
            (Direction::Down, 'L') => (Location(i + 1, j), Direction::Right),
            (Direction::Down, 'J') => (Location(i - 1, j), Direction::Left),

            (Direction::Up, '|') => (Location(i, j - 1), Direction::Up),
            (Direction::Up, 'F') => (Location(i + 1, j), Direction::Right),
            (Direction::Up, '7') => (Location(i - 1, j), Direction::Left),

            // _ => panic!(
            //     "Run into a corner :( here: {:?} c={}",
            //     state,
            //     at(i as usize, j as usize)
            // ),
            _ => state,
        }
    };

    let mut c1 = cursors[0];
    let mut c2 = cursors[1];

    let real_s = match (c1.1, c2.1) {
        (Direction::Left, Direction::Up) => 'J',
        (Direction::Left, Direction::Right) => '-',
        (Direction::Left, Direction::Down) => '7',
        (Direction::Right, Direction::Up) => 'L',
        (Direction::Right, Direction::Left) => '-',
        (Direction::Right, Direction::Down) => 'F',
        (Direction::Up, Direction::Left) => 'J',
        (Direction::Up, Direction::Down) => '|',
        (Direction::Up, Direction::Right) => 'L',
        (Direction::Down, Direction::Left) => '7',
        (Direction::Down, Direction::Up) => '|',
        (Direction::Down, Direction::Right) => 'F',
        _ => panic!("Found weird start: {:?}, {:?}", c1, c2),
    };

    // if do_print {
    //     println!("Replaced S with {}", real_s);
    // }

    // if do_print {
    //     println!("c1 starts at {:?}", c1);
    //     println!("c2 starts at {:?}", c2);
    // }

    let mut is_edge = vec![false; w * h];

    let mut set_edge = |x, y| {
        is_edge[y as usize * w + x as usize] = true;
    };

    set_edge(s.0, s.1);
    set_edge(c1.0 .0, c1.0 .1);
    set_edge(c2.0 .0, c2.0 .1);

    let mut t = 1;
    while c1.0 != c2.0 {
        // if do_print {
        //     println!("At {:?}, {}, {:?}, {}", c1, at(c1.0.0, c1.0.1), c2, at(c2.0.0, c2.0.1));
        // }

        c1 = step(c1);
        set_edge(c1.0 .0, c1.0 .1);

        if c1.0 == c2.0 {
            break;
        }

        c2 = step(c2);
        set_edge(c2.0 .0, c2.0 .1);

        t += 1;
    }

    // if do_print {
    //     for j in 0..h {
    //         for i in 0..w {
    //             print!("{}", is_edge[j*w + i] as i32);
    //         }
    //         println!();
    //     }
    // }

    let mut area = 0;

    for j in 0..h {
        let mut last_turn = None;
        let mut inside = false;

        for i in 0..w {
            let mut c = at(i, j);
            if !is_edge[j * w + i] {
                c = '.';
            } else if i == s.0 as usize && j == s.1 as usize {
                c = real_s;
            }

            match (last_turn, c) {
                (_, '|') => inside = !inside,
                (None, 'F') | (None, 'L') | (None, 'J') | (None, '7') => last_turn = Some(c),
                (None, _) => (),
                (Some('F'), 'J') | (Some('L'), '7') => {
                    inside = !inside;
                    last_turn = None;
                }
                (Some('L'), 'J') | (Some('F'), '7') => {
                    last_turn = None;
                }
                (Some('L'), '-') | (Some('F'), '-') => (),
                // _ => panic!("My logic did not expect this: {:?}, {}", last_turn, c),
                _ => (),
            };

            if !is_edge[j * w + i] && inside {
                area += 1;
            }

            // if do_print {
            //     if is_edge[j*w + i] {
            //         print!("*");
            //     } else {
            //         if inside { print!("I"); }
            //         else { print!("."); }
            //     }
            // }
        }
        // if do_print { println!(); }
    }

    if do_print {
        println!("Problem 10 A: {}", t);
        println!("Problem 10 B: {}", area);
    }
}

fn problem11ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/11.in").unwrap();

    let mut w = 0;
    while data[w] as char != '\n' {
        w += 1;
    }
    let w = w;

    let mut xs = Vec::with_capacity(w * 4);
    let mut ys = Vec::with_capacity(data.len() / w * 4);

    // if do_print {
    //     println!("W = {}", w);
    // }

    let mut y: u32 = 0;

    for i in 0..data.len() {
        if data[i] as char == '\n' {
            y += 1;
            continue;
        }

        let x: u32 = (i % (w + 1)) as u32;

        if data[i] as char == '#' {
            xs.push(x);
            ys.push(y);

            // if do_print {
            //     println!("Found a galaxy at {},{}", x, y);
            // }
        }
    }

    let sum_along_1d = |arr: &Vec<u32>, expansion_rate: u32| -> u64 {
        let mut expected = 0;
        let mut expansion = 0;

        let mut expanded_arr = Vec::with_capacity(arr.len());
        let mut sum: u64 = 0;

        for i in 0..arr.len() {
            if arr[i] > expected {
                expansion += arr[i] - expected;
            }
            expected = arr[i] + 1;

            let v = arr[i] + expansion * (expansion_rate - 1);
            expanded_arr.push(v);
            sum += v as u64;

            // if do_print {
            //     println!(
            //         "X={}, expansion={}, expected={}",
            //         arr[i], expansion, expected
            //     );
            // }
        }

        let mut right_sum = sum;
        let mut right_count = arr.len() as u64;

        let mut distance_sum = 0;

        for v in expanded_arr {
            right_sum -= v as u64;
            right_count -= 1;
            let left_sum = sum - right_sum;
            let left_count = arr.len() as u64 - right_count;

            distance_sum += left_count * v as u64 - left_sum + right_sum - right_count * v as u64;
        }

        distance_sum
    };

    // if do_print {
    //     println!("X: {:?}", xs);
    //     println!("Y: {:?}", ys);
    // }

    xs.sort();

    let total_distance_sum_a = (sum_along_1d(&xs, 2) + sum_along_1d(&ys, 2)) / 2;
    let total_distance_sum_b = (sum_along_1d(&xs, 1000000) + sum_along_1d(&ys, 1000000)) / 2;
 
    if do_print {
        println!("Problem 11 A: {}", total_distance_sum_a);
        println!("Problem 11 B: {}", total_distance_sum_b);
    }
}

fn problem12ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/12.in").unwrap();
    
    let mut sum_ways = 0;
    let mut sum_ways5 = 0;
    
    let mut i = 0;
    while i < data.len() {
        let mut l = i;
        while data[l] as char != ' ' { l += 1; }
        
        let symbols = &data[i..l];
        i = l + 1;
        
        let mut seq = Vec::new();
        
        let mut num = 0;
        while data[i] as char != '\n' {
            if data[i] as char == ',' {
                seq.push(num);
                num = 0;
            } else {
                num = num*10 + (data[i] as usize - '0' as usize);
            }
            i += 1;
        }
        seq.push(num);
        i += 1;
        
        // if do_print {
        //     println!("Symbols: {:?}", symbols.iter().map(|b|*b as char).collect::<String>());
        //     println!("Seq: {:?}", seq);
        // }
        
        let solve = |symbols: &Vec<u8>, seq: &Vec<usize>| {
            let s = symbols.len();
            let l = seq.len();
            
            let mut ways = vec![0u64; (s+2)*(l+1)];
            
            macro_rules! flat_index {
                ($symbol_index:expr, $seq_index:expr) => {
                    $symbol_index * (l+1) + $seq_index
                };
            }

            let mut next_dot_dist = vec![0; symbols.len()];
            let mut current_dist = symbols.len();
            for k in (0..symbols.len()).rev() {
                if symbols[k] as char == '.' {
                    current_dist = 0;
                } else {
                    current_dist += 1;
                }
                next_dot_dist[k] = current_dist;
            }
            let next_dot_dist = next_dot_dist;
            
            for i in (0..s+2).rev() {
                if i < s && symbols[i] as char == '#' { break; }
                
                ways[flat_index!(i, l)] = 1;
            }
            
            for i in (0..s).rev() {
                for j in (0..l).rev() {
                    let mut combinations = 0;
                    
                    // if do_print { println!("i={}, j={}",i,j); }
                    // if do_print { println!(" -> symbols={:?}, seq={:?}",&symbols[i..].iter().map(|b|*b as char).collect::<String>(),&seq[j..]); }
                    
                    if i+1 < s && symbols[i] as char != '#' {
                        // if do_print { println!(" -> Can skip one ahead"); }
                        combinations += ways[flat_index!(i+1, j)];
                    }
                    
                    if i + seq[j] <= s {
                        // if do_print { println!(" -> seq of len {} might fit", seq[j]); }
                        
                        let mut valid = i + seq[j] == s || symbols[i + seq[j]] as char != '#';
                        
                        // if do_print { if !valid { 
                        //     println!("  -> Sadly there's a # afterwards thats blocking it"); 
                        // } }
                        
                        if next_dot_dist[i] < seq[j] {
                            valid = false;
                        }
                        
                        if valid {
                            combinations += ways[flat_index!(i + seq[j] + 1, j+1)];
                        }
                    }
                    
                    ways[flat_index!(i, j)] = combinations;
                    
                    // if do_print { println!(" -> combinations={}",combinations); }
                }
            }
                
            // if do_print { println!("ways={:?}", ways); }
            
            // if do_print {
            //     println!("Total ways: {}", ways[flat_index!(0, 0)]);
            // }
            
            ways[flat_index!(0, 0)]
        };
        
        let symbols = symbols.to_vec();
        let mut symbols5 = Vec::new();
        for k in 0..5 {
            if k > 0 { symbols5.push('?' as u8); }
            for sym in symbols.iter() { symbols5.push(*sym); }
        }
        let seq5 = seq.repeat(5);
        
        sum_ways += solve(&symbols, &seq);
        sum_ways5 += solve(&symbols5, &seq5);
    }
        
    if do_print {
        println!("Problem 12 A: {}", sum_ways);
        println!("Problem 12 B: {}", sum_ways5);
    }
}

fn problem13ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/13.in").unwrap();

    let mut sum = 0;
    let mut sum_b = 0;

    let mut i = 0;
    while i < data.len() {
        let mut cols: Vec<u32> = Vec::new();
        let mut rows: Vec<u32> = Vec::new();

        while i < data.len() && data[i] as char != '\n' {
            let mut coli = 0;
            let mut row = 0;
            while data[i] as char != '\n' {
                let on = data[i] as char == '#';
                if coli == cols.len() {
                    cols.push(0);
                }
                cols[coli] = (cols[coli] << 1) + on as u32;
                row = (row << 1) + on as u32;
                
                coli += 1;
                i += 1;
            }
            rows.push(row);
            i += 1;
        }
        i += 1;

        // if do_print {
        //     print!("Rows_b: "); for v in rows.iter() { print!("{:#011b} ", v); } println!();
        //     print!("Cols_b: "); for v in cols.iter() { print!("{:#011b} ", v); } println!();
            
        //     println!("Rows_v: {:?}", rows);
        //     println!("Cols_v: {:?}", cols);
        // }

        let check_reflection_a = |vals: &Vec<u32>| -> Option<u32> {
            for k in 0..vals.len()-1 {
                let mut symmetric = true;
                let mut offset = 0;
                while symmetric && offset <= k && k+offset+1 < vals.len() {
                    if vals[k-offset] != vals[k+offset+1] {
                        symmetric = false;
                    }
                    offset += 1;
                }
                if symmetric {
                    return Some(k as u32);
                }
            }
            None
        };

        let check_reflection_b = |vals: &Vec<u32>| -> Option<u32> {
            for k in 0..vals.len()-1 {
                let mut symmetric = true;
                let mut smudge = false;
                let mut offset = 0;
                while symmetric && offset <= k && k+offset+1 < vals.len() {
                    let a = vals[k-offset];
                    let b = vals[k+offset+1];
                    
                    if a != b {
                        if !smudge && (a^b).count_ones() == 1 {
                            smudge = true;
                        } else {
                            symmetric = false;
                        }
                    }
                    offset += 1;
                }
                if symmetric && smudge {
                    return Some(k as u32);
                }
            }
            None
        };

        // if do_print {
        //     println!(" -> reflection in rowsA: {:?}", check_reflection_a(&rows));
        //     println!(" -> reflection in colsA: {:?}", check_reflection_a(&cols));
        // }

        // if do_print {
        //     println!(" -> reflection in rowsB: {:?}", check_reflection_b(&rows));
        //     println!(" -> reflection in colsB: {:?}", check_reflection_b(&cols));
        // }

        if let Some(r) = check_reflection_a(&rows) {
            sum += 100 * (r+1);
        }
        else if let Some(c) = check_reflection_a(&cols) {
            sum += c+1;
        }

        if let Some(r) = check_reflection_b(&rows) {
            sum_b += 100 * (r+1);
        }
        else if let Some(c) = check_reflection_b(&cols) {
            sum_b += c+1;
        }

        // if do_print { println!(); }
    }

    if do_print {
        println!("Problem 13 A: {}", sum);
        println!("Problem 13 B: {}", sum_b);
    }
}

fn problem14ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/14.in").unwrap();

    let mut w = 0;
    while data[w] as char != '\n' { w += 1; }
    let w = w;

    let h = data.len() / (w+1);

    // if do_print {
    //     println!("W={}, H={}",w,h);
    // }

    let mut distance_left = vec![0u32; w*h];
    let mut distance_right = vec![0u32; w*h];
    let mut distance_up = vec![0u32; w*h];
    let mut distance_down = vec![0u32; w*h];

    for i in 0..h {
        let mut prev = '#';
        for j in 0..w {
            let c = data[i*(w+1) + j] as char;
            if prev != '#' && c != '#' {
                distance_left[i*w + j] = distance_left[i*w + j - 1] + 1;
            }
            prev = c;
        }
    }

    for i in 0..h {
        let mut prev = '#';
        for j in (0..w).rev() {
            let c = data[i*(w+1) + j] as char;
            if prev != '#' && c != '#' {
                distance_right[i*w + j] = distance_right[i*w + j + 1] + 1;
            }
            prev = c;
        }
    }

    for j in 0..w {
        let mut prev = '#';
        for i in 0..h {
            let c = data[i*(w+1) + j] as char;
            if prev != '#' && c != '#' {
                distance_up[i*w + j] = distance_up[(i-1)*w + j] + 1;
            }
            prev = c;
        }
    }

    for j in 0..w {
        let mut prev = '#';
        for i in (0..h).rev() {
            let c = data[i*(w+1) + j] as char;
            if prev != '#' && c != '#' {
                distance_down[i*w + j] = distance_down[(i+1)*w + j] + 1;
            }
            prev = c;
        }
    }

    #[derive(Debug, Clone)]
    struct Stone {
        x: u32,
        y: u32,
    }

    impl Stone {
        fn index(&self, w: usize) -> usize {
            self.y as usize * w + self.x as usize
        }
    }

    let mut stones = vec![];

    for i in 0..h {
        for j in 0..w {
            let c = data[i*(w+1) + j] as char;
            if c == 'O' {
                stones.push(Stone{x:j as u32, y:i as u32});
            }
        }
    }

    // if do_print {
    //     for (name, arr) in [("Left", &distance_left),
    //                                              ("Right", &distance_right),
    //                                              ("Up", &distance_up),
    //                                              ("Down", &distance_down)] {
    //         println!("Distance {}:", name);
    //         for i in 0..h {
    //             for j in 0..w {
    //                 print!(" {:>2}", arr[i*w + j]);
    //             }
    //             println!()
    //         }
    //         println!()
    //     }

    //     println!("Stones: {:?}", stones);
    // }

    let roll_up = |stones: &mut Vec<Stone>, occupied: &mut Vec<bool>| {
        for o in stones.iter_mut() {
            let steps = distance_up[o.index(w)];
            occupied[o.index(w)] = false;
            
            o.y -= steps;

            while occupied[o.index(w)] { o.y += 1; }
            occupied[o.index(w)] = true;
        }
    };

    let roll_down = |stones: &mut Vec<Stone>, occupied: &mut Vec<bool>| {
        for o in stones.iter_mut() {
            let steps = distance_down[o.index(w)];
            occupied[o.index(w)] = false;
            
            o.y += steps;

            while occupied[o.index(w)] { o.y -= 1; }
            occupied[o.index(w)] = true;
        }
    };

    let roll_left = |stones: &mut Vec<Stone>, occupied: &mut Vec<bool>| {
        for o in stones.iter_mut() {
            let steps = distance_left[o.index(w)];
            occupied[o.index(w)] = false;
            
            o.x -= steps;

            while occupied[o.index(w)] { o.x += 1; }
            occupied[o.index(w)] = true;
        }
    };

    let roll_right = |stones: &mut Vec<Stone>, occupied: &mut Vec<bool>| {
        for o in stones.iter_mut() {
            let steps = distance_right[o.index(w)];
            occupied[o.index(w)] = false;
            
            o.x += steps;

            while occupied[o.index(w)] { o.x -= 1; }
            occupied[o.index(w)] = true;
        }
    };

    #[allow(unused)]
    let print_map = |occupied: &Vec<bool>| {
        for i in 0..h {
            for j in 0..w {
                let c = if occupied[i*w+j] {'O'} else
                              if data[i*(w+1)+j] as char == '#' {'#'} else {'.'};
                print!("{}",c);
            }
            println!()
        }
        println!()
    };

    let evaluate = |stones: &Vec<Stone>| {
        let mut total_score = 0;

        for Stone{ x:_, y } in stones.iter() {
            total_score += h - *y as usize;
        }

        total_score
    };

    let mut is_occupied = vec![false; w*h];
    for i in 0..h {
        for j in 0..w {
            let c = data[i*(w+1) + j] as char;
            is_occupied[i*w + j] = c == 'O';
        }
    }

    // if do_print {
    //     for _ in 0..3 {
    //         roll_up(&mut stones, &mut is_occupied);
    //         roll_left(&mut stones, &mut is_occupied);
    //         roll_down(&mut stones, &mut is_occupied);
    //         roll_right(&mut stones, &mut is_occupied);
    //         print_map(&is_occupied);
    //     }
    //     roll_up(&mut stones, &mut is_occupied);
    //     print_map(&is_occupied);
    //     roll_left(&mut stones, &mut is_occupied);
    //     print_map(&is_occupied);
    //     roll_down(&mut stones, &mut is_occupied);
    //     print_map(&is_occupied);
    //     roll_right(&mut stones, &mut is_occupied);
    //     print_map(&is_occupied);
    // }

    let mut occupied_a = is_occupied.clone();
    let mut stones_a = stones.clone();
    roll_up(&mut stones_a, &mut occupied_a);
    let score_a = evaluate(&stones_a);

    // if do_print {
    //     println!("Rolled up 1x score={}", score_a);
    //     print_map(&occupied_a);
    // }

    let mut values = Vec::new();
    let mut i = 0;

    let step = |stones: &mut Vec<Stone>, occupied: &mut Vec<bool>| {
        roll_up(stones, occupied);
        roll_left(stones, occupied);
        roll_down(stones, occupied);
        roll_right(stones, occupied);
    };

    loop {
        let val = evaluate(&stones);
        values.push(val);
        // if do_print && i < 4 {
        //     println!("It {} val={}",i,val);
        //     print_field(&field);
        //     println!();
        // }
        if i > 9 && val == values[i/2] 
                 && values[values.len()-1-1] == values[i/2-1] 
                 && values[values.len()-1-2] == values[i/2-2] 
                 && values[values.len()-1-3] == values[i/2-3] 
                 && values[values.len()-1-4] == values[i/2-4] {
            break;
        }
        step(&mut stones, &mut is_occupied);
        i += 1;
    };

    let period = i - i/2;
    const N: usize = 1000000000;
    let n = (N - i/2) % period + i/2;
    
    // if do_print {
    //     println!("Found a loop: {} -> {}. N maps to {}", i, i/2, n);
    //     // println!("values: {:?}", values);
    // }

    let score_b = values[n];

    if do_print {
        println!("Problem 14 A: {}", score_a);
        println!("Problem 14 B: {}", score_b);
    }

}

fn problem15ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/15.in").unwrap();

    let mut sum_a = 0;
    let mut acc: u32 = 0;
    for i in 0..data.len() {
        if data[i] as char == ',' {
            // if do_print {
            //     println!("Val: {}", acc);
            // }
            sum_a += acc;
            acc = 0;
        } else if data[i] as char != '\n' {
            acc += data[i] as u32;
            acc *= 17;
            acc %= 256;
        }
    }
    sum_a += acc;

    #[derive(Clone)]
    struct Entry(Vec<u8>, i32);

    let mut boxes: Vec<Vec<Entry>> = vec![Vec::with_capacity(8); 256];

    let mut i = 0;
    while i < data.len() {
        let mut acc: u32 = 0;
        
        let mut j = i;
        while data[j] as char >= 'a' { // ,-=\n are all smaller than 'a'
            acc += data[j] as u32;
            acc *= 17;
            acc %= 256;
            j += 1;
        }

        let id = data[i..j].to_owned();

        let the_box = &mut boxes[acc as usize];

        if data[j] as char == '-' {
            for Entry(eid, eval) in the_box.iter_mut() {
                if *eval >= 0 && *eid == id {
                    *eval = -1;
                    break;
                }
            }
            j += 2;
        } else {
            let val = data[j+1] as i32 - '0' as i32;
            let mut new_entry = true;
            for Entry(eid, eval) in the_box.iter_mut().rev() {
                if *eid == id {
                    if *eval > 0 {
                        *eval = val;
                        new_entry = false;
                    }
                    break;
                }
            }
            if new_entry {
                the_box.push(Entry(id, val));
            }
            j += 3;
        }

        i = j;

        // if do_print {
        //     for k in 0..256 {
        //         if boxes[k].len() > 0 {
        //             print!("Box {}:", k);
        //             for e in boxes[k].iter() {
        //                 print!(" [{}, {}]", e.0.iter().map(|b|*b as char).collect::<String>(), e.1);
        //             }
        //             println!();
        //         }
        //     }
        //     println!();
        // }
    }

    // if do_print {
    //     for k in 0..256 {
    //         if boxes[k].len() > 0 {
    //             print!("Box {}:", k);
    //             for e in boxes[k].iter() {
    //                 print!(" [{}, {}]", e.0.iter().map(|b|*b as char).collect::<String>(), e.1);
    //             }
    //             println!();
    //         }
    //     }
    //     println!();
    // }

    let mut sum_b = 0;
    for k in 0..boxes.len() {
        let mut slot = 1;
        for Entry(_, val) in boxes[k].iter() {
            if *val >= 0 {
                sum_b += (k as i32+1) * val * slot;
                slot += 1;
            }
        }
    }


    if do_print {
        println!("Problem 15 A: {}", sum_a);
        println!("Problem 15 B: {}", sum_b);
    }
}

fn problem16ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/16.in").unwrap();

    let mut w = 0;
    while data[w] as char != '\n' {w += 1;}
    let w = w;
    let h = data.len() / (w+1);

    // if do_print {
    //     println!("W={} H={}", w, h);
    // }

    #[derive(Eq, PartialEq, Hash, Clone)]
    struct Cursor {
        val: u32
    }

    impl Cursor {
        fn x(&self) -> i32 {(self.val & 0x00FF) as i32}
        fn y(&self) -> i32 {((self.val & 0xFF00) >> 8) as i32}
        fn dx(&self) -> i32 {((self.val >> 16) & 0b0011) as i32 - 1}
        fn dy(&self) -> i32 {(((self.val >> 16) & 0b1100) >> 2) as i32 - 1}

        fn new(x: i32, y: i32, dx: i32, dy: i32) -> Cursor {
            Cursor {
                val: (((dy+1) as u32) << 18) | (((dx+1) as u32) << 16) | ((y as u32) << 8) | (x as u32)
            }
        }

        fn index(&self, w: usize) -> usize {
            self.y() as usize * (w+1) + self.x() as usize
        }

        fn exit_index(&self) -> usize {
            let hor = self.horizontal();

            // println!("me: {:?}, hor: {}, dir: {}, xy: {}", self, hor, (self.direction() as usize), ((if hor {self.x()} else {self.y()}) as usize));

            (self.direction() as usize) |
            (((if hor {self.y()} else {self.x()}) as usize) << 2)
        }

        fn next(&self, w: usize, h: usize) -> Option<Cursor> {
            let nx = self.x() + self.dx();
            let ny = self.y() + self.dy();

            if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 {
                None
            } else {
                Some(Cursor::new(nx, ny, self.dx(), self.dy()))
            }
        }

        fn left(&self) -> Cursor {
            Cursor::new(self.x(), self.y(), self.dy(), -self.dx())
        }

        fn right(&self) -> Cursor {
            Cursor::new(self.x(), self.y(), -self.dy(), self.dx())
        }

        fn direction(&self) -> u32 {
            if self.dx() != 0 { ((self.dx() + 1)/2) as u32 }
            else { ((self.dy() + 1)/2 + 3) as u32 }
        }

        fn direction_bit(&self) -> u32 {
            1 << self.direction()
        }

        fn horizontal(&self) -> bool { self.dx() != 0 }

        fn back(&self) -> Cursor {
            Cursor::new(self.x(), self.y(), -self.dx(), -self.dy())
        }
    }

    impl Debug for Cursor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Cursor")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("dx", &self.dx())
            .field("dy", &self.dy())
            .finish()
        }
    }

    let mut found_exit = vec![false; w*4+h*4];

    let mut count_visited = |start: Cursor| -> u32 {

        if found_exit[start.back().exit_index()] {
            // if do_print {println!("Already found exit for {:?}: exit index is {}", start, start.back().exit_index())}
            return 0;
        }

        let mut visited = vec![0; (w+1)*h];

        let mut queue = Vec::with_capacity(64);
        queue.push(start);

        while let Some(p) = queue.pop() {
            let pind = p.index(w);
            let pdir = p.direction_bit();

            // if do_print {
            //     println!("At: {:?}, c = {}, hor = {}, visited = {}, dir = {}", 
            //              p, data[pind] as char, p.horizontal(), visited[pind], pdir);
            // }

            if visited[pind] & pdir > 0 {
                // if do_print { println!(" > Already been here, ignoring"); }
                continue;
            }

            visited[pind] |= pdir;

            match (data[pind] as char, p.horizontal()) {
                ('.', _) | ('|', false) | ('-', true) => {
                    if let Some(p_new) = p.next(w, h) {
                        // if do_print {println!(" > Adding {:?} to queue", p_new)}
                        queue.push(p_new);
                    } else { found_exit[p.clone().exit_index()] = true; }
                },
                ('|', true) | ('-', false) => {
                    if let Some(l2) = p.left().next(w, h) {
                        // if do_print {println!(" > Adding {:?} to queue", l2)}
                        queue.push(l2);
                    } else { found_exit[p.left().exit_index()] = true; }
                    
                    if let Some(r2) = p.right().next(w, h) {
                        // if do_print {println!(" > Adding {:?} to queue", r2)}
                        queue.push(r2);
                    } else { found_exit[p.right().exit_index()] = true; }
                }
                ('\\', true) | ('/', false) => {
                    if let Some(r2) = p.right().next(w, h) {
                        // if do_print {println!(" > Adding {:?} to queue", r2)}
                        queue.push(r2);
                    } else { found_exit[p.right().exit_index()] = true; }
                }
                ('\\', false) | ('/', true) => {
                    if let Some(l2) = p.left().next(w, h) {
                        // if do_print {println!(" > Adding {:?} to queue", l2)}
                        queue.push(l2);
                    } else { found_exit[p.left().exit_index()] = true; }
                }
                _ => ()
            }

            // if do_print {
            //     for i in 0..h {
            //         for j in 0..w {
            //             print!("{}", if visited[i*(w+1)+j] > 0 {'#'} else {'.'});
            //         }
            //         println!();
            //     }
            //     println!();
            // }
        }

        // if do_print {
        //     for i in 0..h {
        //         for j in 0..w {
        //             print!("{}", if visited[i*(w+1)+j] > 0 {'#'} else {'.'});
        //         }
        //         println!();
        //     }
        //     println!();
        // }

        let mut number_of_visited = 0;

        for i in 0..h {
            for j in 0..w {
                if visited[i*(w+1)+j] > 0 {
                    number_of_visited += 1;
                }
            }
        }

        number_of_visited
    };

    let energized = count_visited(Cursor::new(0, 0, 1, 0));

    let mut max_energized = 0;
    for i in 0..w {
        max_energized = max(max_energized,count_visited(Cursor::new(i as i32,0,0,1)));
        max_energized = max(max_energized, count_visited(Cursor::new(i as i32,h as i32-1,0,-1)));
    }
    for i in 0..h {
        max_energized = max(max_energized,count_visited(Cursor::new(0, i as i32,1,0)));
        max_energized = max(max_energized, count_visited(Cursor::new(w as i32-1, i as i32,-1,0)));
    }


    if do_print {
        println!("Problem 16 A: {}", energized);
        println!("Problem 16 B: {}", max_energized);
    }

}

fn problem17ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/17.in").unwrap();

    let mut w = 0;
    while data[w] as char != '\n' {w += 1;}
    let w = w;
    let h = data.len() / (w+1);


    let mut cost = Vec::with_capacity(w*h);
    for i in 0..h {
        for j in 0..w {
            cost.push(data[i*(w+1) + j] as u32 - '0' as u32);
        }
    }
    let cost = cost;

    // if do_print {
    //     for i in 0..h {
    //         for j in 0..w {
    //             print!("{}", cost[i*w+j]);
    //         }
    //         println!()
    //     }
    // }

    
    const INFINITE: u32 = 0xFFFFFFFF;

    let find_minimum_loss = |min_step: u32, max_step: u32| {
        let mut mlosses = vec![INFINITE; w*h*2];

        let mut qs = vec![vec![]; 91];
        qs[0].push(0);
        qs[0].push(1);

        mlosses[0] = 0;
        mlosses[1] = 0;

        let consider_loss = |q: u32, next_hor: u32, loss: u32, qs: &mut Vec<Vec<u32>>, mlosses: &mut Vec<u32>| {
            // if do_print {println!("Considering q={} with loss={} hor={}", q, loss, next_hor)}
            
            let q = q*2 + next_hor;

            if mlosses[q as usize] > loss {
                mlosses[q as usize] = loss;
                qs[loss as usize % 91].push(q);
            }
        };

        let mut loss: u32 = 0;
        
        loop {
            while let Some(p) = qs[loss as usize % 91].pop() {
                let horizontal = (p&1) == 0;
                let next_horizontal = 1 - (p&1);
                let p = p>>1;
    
                if p == (w*h-1) as u32 {
                    break;
                }
    
                // if do_print {
                //     println!("Processing State(p: {}, loss: {}, hor: {})", p, loss, horizontal);
                // }
    
                // if do_print {
                //     for i in 0..h {
                //         for j in 0..w {
                //             let v = min(mlosses[(i*w + j)*2+0], mlosses[(i*w + j)*2+1]);
                //             if v == INFINITE {print!(" [] ");} else {print!("{:>3} ", v);}
                //         }
                //         println!();
                //     }
                //     println!();
                // }
    
                let step = if horizontal {1} else {w as u32};
                let col = p % w as u32;
    
                let mut q = p;
                let mut new_loss = loss;
                for i in 1..=max_step {
                    q += step;
    
                    if q >= (w*h) as u32 || (horizontal && col+i >= w as u32) {
                        break;
                    }
                    
                    new_loss += cost[q as usize];
    
                    if i >= min_step {
                        consider_loss(q, next_horizontal, new_loss, &mut qs, &mut mlosses);
                    }
                }
                
                let mut q = p;
                let mut new_loss = loss;
                for i in 1..=max_step {
                    if q < step || (horizontal && col < i) {
                        break;
                    }
    
                    q -= step;
                    new_loss += cost[q as usize];
    
                    if i >= min_step {
                        consider_loss(q, next_horizontal, new_loss, &mut qs, &mut mlosses);
                    }
                }
    
            }
            loss += 1;

            let final_loss = min(mlosses[2*(w*h-1)+0], mlosses[2*(w*h-1)+1]);

            if final_loss != INFINITE {
                return final_loss;
            }
        }
    };



    // #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    // struct State {
    //     heat_loss: i32,
    //     x: i32,
    //     y: i32,
    //     next_step_hor: bool,
    // }

    // impl State {
    //     fn map_index(&self, w: usize) -> usize {
    //         self.y as usize * (w+1) + self.x as usize
    //     }
    //     fn visited_index(&self, w: usize) -> usize {
    //         self.map_index(w) * 2 + self.next_step_hor as usize
    //     }
    // }

    // let find_minimum_loss = |min_step: i32, max_step: i32| {
    //     let mut mlosses = vec![-1000000000; (w+1)*h*2];

    //     let mut pq = BinaryHeap::new();
    //     pq.push(State { heat_loss: 0, x: 0, y: 0, next_step_hor: true});
    //     pq.push(State { heat_loss: 0, x: 0, y: 0, next_step_hor: false});

    //     mlosses[State { heat_loss: 0, x: 0, y: 0, next_step_hor: true}.visited_index(w)] = 0;
    //     mlosses[State { heat_loss: 0, x: 0, y: 0, next_step_hor: false}.visited_index(w)] = 0;

    //     let mut minimum_loss = -1000000000;

    //     while let Some(s) = pq.pop() {
    //         if s.x as usize == w-1 && s.y as usize == h-1 {
    //             minimum_loss = s.heat_loss;
    //             break;
    //         }

    //         // if do_print {
    //         //     println!("Discovered minimum heat loss to {},{},{}. It is: {}", s.x, s.y, if s.next_step_hor {'H'} else {'V'}, s.heat_loss);
    //         // }
    //         // if do_print {
    //         //     for i in 0..h {
    //         //         for j in 0..w {
    //         //             let v = max(mlosses[(i*(w+1) + j)*2+0], mlosses[(i*(w+1) + j)*2+1]);
    //         //             if v == -1000000000 {print!(" [] ");} else {print!("{:>3} ", -v);}
    //         //         }
    //         //         println!();
    //         //     }
    //         //     println!();
    //         // }

    //         if s.next_step_hor {
    //             let mut s2 = s.clone();
    //             s2.next_step_hor = false;

    //             while s.x+max_step > s2.x && s2.x+1 < w as i32 {
    //                 s2.x += 1;
    //                 s2.heat_loss -= data[s2.map_index(w)] as i32 - '0' as i32;
    //                 if s2.x - s.x >= min_step {
    //                     if mlosses[s2.visited_index(w)] < s2.heat_loss {
    //                         mlosses[s2.visited_index(w)] = s2.heat_loss;
    //                         pq.push(s2);
    //                     }
    //                 }
    //             }

    //             s2.x = s.x;
    //             s2.heat_loss = s.heat_loss;

    //             while s.x < s2.x+max_step && s2.x > 0 {
    //                 s2.x -= 1;
    //                 s2.heat_loss -= data[s2.map_index(w)] as i32 - '0' as i32;
    //                 if s.x - s2.x >= min_step {
    //                     if mlosses[s2.visited_index(w)] < s2.heat_loss {
    //                         mlosses[s2.visited_index(w)] = s2.heat_loss;
    //                         pq.push(s2);
    //                     }
    //                 }
    //             }
    //         } else {
    //             let mut s2 = s.clone();
    //             s2.next_step_hor = true;

    //             while s.y+max_step > s2.y && s2.y+1 < h as i32 {
    //                 s2.y += 1;
    //                 s2.heat_loss -= data[s2.map_index(w)] as i32 - '0' as i32;
    //                 if s2.y - s.y >= min_step {
    //                     if mlosses[s2.visited_index(w)] < s2.heat_loss {
    //                         mlosses[s2.visited_index(w)] = s2.heat_loss;
    //                         pq.push(s2);
    //                     }
    //                 }
    //             }

    //             s2.y = s.y;
    //             s2.heat_loss = s.heat_loss;

    //             while s.y <= s2.y+max_step && s2.y > 0 {
    //                 s2.y -= 1;
    //                 s2.heat_loss -= data[s2.map_index(w)] as i32 - '0' as i32;
    //                 if s.y - s2.y >= min_step {
    //                     if mlosses[s2.visited_index(w)] < s2.heat_loss {
    //                         mlosses[s2.visited_index(w)] = s2.heat_loss;
    //                         pq.push(s2);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     -minimum_loss
    // };

    let answer_a = find_minimum_loss(1, 3);
    let answer_b = find_minimum_loss(4, 10);

    if do_print {
        println!("Problem 17 A: {}", answer_a);
        println!("Problem 17 B: {}", answer_b);
    }

}

fn problem18ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/18.in").unwrap();

    let mut i = 0;

    struct AreaTracker {
        x: i64,
        y: i64,
        area: i64,
        first_dir: char,
        last_dir: char,
        outer_corners: i64,
        inner_corners: i64,
        edges: i64,
    }

    impl AreaTracker {
        fn new() -> AreaTracker {
            AreaTracker {
                x: 0,
                y: 0,
                area: 0,
                first_dir: 'S',
                last_dir: 'S',
                outer_corners: 0,
                inner_corners: 0,
                edges: 0,
            }
        }

        fn note_corner(&mut self, last_dir: char, direction: char) {
            match (last_dir, direction) {
                ('R', 'U') | ('U', 'L') | ('L', 'D') | ('D', 'R') => {
                    self.outer_corners += 1;
                }
                ('U', 'R') | ('R', 'D') | ('D', 'L') | ('L', 'U') => {
                    self.inner_corners += 1;
                }
                _ => ()
            };
        }
        
        fn process_step(&mut self, direction: char, num: i64)
        {
            self.note_corner(self.last_dir, direction);
            self.last_dir = direction;

            if self.first_dir == 'S' {
                self.first_dir = direction;
            }

            self.edges += num + 1;

            match direction {
                'U' => {
                    self.y += num;
                }
                'D' => {
                    self.y -= num;
                }
                'L' => {
                    self.x -= num;
                    self.area -= self.y*num;
                },
                'R' => {
                    self.x += num;
                    self.area += self.y*num;
                },
                // _ => panic!("Unexpected direction: {}", direction),
                _ => ()
            };
        }

        fn close_loop(&mut self) {
            self.note_corner(self.last_dir, self.first_dir);
            
            if self.inner_corners > self.outer_corners {
                std::mem::swap(&mut self.inner_corners, &mut self.outer_corners);
            }
        }

        fn calc_area(&self) -> i64 {
            (self.area*4 + self.edges*2 - self.inner_corners * 3 - self.outer_corners) / 4
        }
    }

    let mut tracker_a = AreaTracker::new();
    let mut tracker_b = AreaTracker::new();

    while i < data.len() {
        let direction = data[i] as char;

        // assume at most 2 digit numbers
        let mut num = data[i+2] as i32 - '0' as i32;
        if data[i+3] as char != ' ' {
            num = num * 10 + data[i+3] as i32 - '0' as i32;
            i += 1;
        }
        i += 6; // D N (#
        
        let mut num_b = 0;
        for _ in 0..5 {
            num_b = (num_b << 4) + data[i] as i32;
            
            if data[i] as char > '9' {
                num_b -= 'a' as i32 - 10;
            } else {
                num_b -= '0' as i32;
            }
            i += 1;
        }

        let direction_b = match data[i] as char {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => '_',
        };
        i += 3;

        // if do_print {
        //     println!("Dir: {}, amount: {}", direction, num);
        //     println!("Dir_b: {}, amount_b: {}", direction_b, num_b);
        // }

        tracker_a.process_step(direction, num as i64);
        tracker_b.process_step(direction_b, num_b as i64);
    }
    tracker_a.close_loop();
    tracker_b.close_loop();

    let area_a = tracker_a.calc_area();
    let area_b = tracker_b.calc_area();

    if do_print {
        println!("Problem 18 A: {}", area_a);
        println!("Problem 18 B: {}", area_b);
    }
}

fn problem19ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/19.in").unwrap();

    let mut state_name_to_id: HashMap<String, u32> = Default::default();

    const INPUT_STATE: u32 = 0;
    const ACCPET_STATE: u32 = 1;
    const REJECT_STATE: u32 = 2;

    state_name_to_id.insert("in".to_owned(), INPUT_STATE);
    state_name_to_id.insert("A".to_owned(), ACCPET_STATE);
    state_name_to_id.insert("R".to_owned(), REJECT_STATE);

    let byte_slice_to_string = |slice: &[u8]| -> String {
        slice.iter().map(|s| *s as char).collect::<String>()
    };

    let read_name = |i: &mut usize| {
        let beg = *i;
        // excludes {},:<>
        while ('A'..='z').contains(&(data[*i] as char)) { *i+=1; }

        byte_slice_to_string(&data[beg..*i])
    };

    let read_number = |i: &mut usize| {
        let mut num = 0;
        while ('0'..='9').contains(&(data[*i] as char)) {
            num = num * 10 + data[*i] as u32 - '0' as u32;
            *i+=1; 
        }

        num
    };

    struct Transition {
        to_id: u32,
        limit: u32,
        variable: u32,
        greater: bool,
    }

    struct Node {
        edges_out: Vec<Transition>,
        next_id: u32,
    }

    let get_id_of_name = |name: String, map: &mut HashMap<String, u32>, nodes: &mut Vec<Node>| -> u32 {
        if let Some(id) = map.get(&name) {*id}
        else {
            let id = map.len() as u32;
            map.insert(name, id);
            nodes.push(Node{edges_out: vec![], next_id: 0});
            id
        }
    };

    let name_to_variable = |name: char| -> u32 {
        match name {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!("Unrecognised name: {}", name),
        }
    };

    let mut nodes = Vec::new();
    nodes.push(Node{edges_out: vec![], next_id: 0});
    nodes.push(Node{edges_out: vec![], next_id: 0});
    nodes.push(Node{edges_out: vec![], next_id: 0});

    let mut i = 0;
    while data[i] as char != '\n' {
        let state_name = read_name(&mut i);
        let from_id = get_id_of_name(state_name.clone(), &mut state_name_to_id, &mut nodes);

        // if do_print {println!("Reading specs for {}", state_name)}

        if data[i] as char != '{' {panic!("Expected {{ at {}", i);}

        i += 1;

        loop {
            let name = read_name(&mut i);
            match data[i] as char {
                '<' | '>' => {
                    let greater = data[i] as char == '>';
                    i += 1;

                    let limit = read_number(&mut i);
                    if data[i] as char != ':' {panic!("Expected : at {}", i)}
                    i += 1;
                    
                    let target = read_name(&mut i);
                    i += 1;
                    
                    // if do_print { println!(" -> Found condition: {}, {}, {}, {}", name, greater, limit, target) };
                    let to_id = get_id_of_name(target, &mut state_name_to_id, &mut nodes);

                    let variable = name_to_variable(name.chars().next().unwrap());

                    nodes[from_id as usize].edges_out.push(Transition{to_id, limit, variable, greater});
                }
                '}' => {
                    // if do_print { println!(" -> Found end: {}", name) };
                    let to_id = get_id_of_name(name, &mut state_name_to_id, &mut nodes);

                    nodes[from_id as usize].next_id = to_id;
                    break;
                }
                _ => panic!("Unexpected token at {}: {}", i, data[i] as char),
            };
        }
        i += 2;
    }
    i += 1;

    // if do_print {
    //     for i in 0..nodes.len() {
    //         print!("{} -> ",i);
    //         for t in nodes[i].edges_out.iter() {
    //             print!("{},{},{},{} ; ", t.to_id, t.variable, t.greater, t.limit);
    //         }
    //         println!("{}", nodes[i].next_id);
    //     }
    // }

    let mut sum = 0;

    while i < data.len() {
        i += 3;
        let x = read_number(&mut i);
        i += 3;
        let m = read_number(&mut i);
        i += 3;
        let a = read_number(&mut i);
        i += 3;
        let s = read_number(&mut i);
        i += 2;

        let xmas = x+m+a+s;

        let nums = [x,m,a,s];
        let mut state = INPUT_STATE;

        while state != ACCPET_STATE && state != REJECT_STATE {
            let node = &nodes[state as usize];
            let mut found = false;
            
            for t in node.edges_out.iter() {
                if (t.greater  && nums[t.variable as usize] > t.limit) || 
                (!t.greater && nums[t.variable as usize] < t.limit) {
                    state = t.to_id;
                    found = true;
                    break;
                }
            }
            
            if !found {
                state = node.next_id;
            }

            // if do_print { print!(" -> {}", state);}
        }
        // if do_print {println!()}

        if state == ACCPET_STATE {
            sum += xmas;
        }
    }

    let mut states = vec![(INPUT_STATE, [[0,4001]; 4])];

    let mut accepted_volume = 0;

    'state_advancing: while let Some((s, mut props)) = states.pop() {
        // if do_print {
        //     println!("Processing: {}, {:?}", s, props);
        //     if s == REJECT_STATE {
        //         println!(" -> REJECTED");
        //     }
        //     if s == ACCPET_STATE {
        //         println!(" -> Accepted viii");
        //     }
        // }

        if s == ACCPET_STATE {
            accepted_volume += props.into_iter().map(|l| l[1] as i64 - l[0] as i64 - 1).product::<i64>();
            continue 'state_advancing;
        }

        if s == REJECT_STATE {
            continue 'state_advancing;
        }

        for t in nodes[s as usize].edges_out.iter() {
            let mut props_pass = props.clone();
            let var = t.variable as usize;

            if t.greater {
                props_pass[var][0] = t.limit;
                props[var][1] = t.limit+1;
            } else {
                props_pass[var][1] = t.limit;
                props[var][0] = t.limit-1;
            }
            if props_pass[var][0] < props_pass[var][1] {
                states.push((t.to_id, props_pass));
            }
            if props[var][0] >= props[var][1] {
                continue 'state_advancing;
            }
        }
        states.push((nodes[s as usize].next_id, props));
    }

    if do_print {
        println!("Problem 19 A: {}", sum);
        println!("Problem 19 B: {}", accepted_volume);
    }
}


fn problem20ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/20.in").unwrap();

    #[derive(Copy, Clone, Debug)]
    #[repr(i32)]
    enum Type {
        FlipFlop(u8),
        Conjunction(u8),
    }

    const INFINITE: u32 = 0xFFFFFFFF;

    let get_id = |name: usize, name_to_id: &mut Vec<u32>, next_id: &mut u32| {
        if name_to_id[name] == INFINITE {
            name_to_id[name] = *next_id;
            *next_id += 1;
        }

        name_to_id[name]
    };

    let get_name = |i: usize| {
        (data[i] as usize - 'a' as usize) * 26 + (data[i+1] as usize - 'a' as usize)
    };

    let read_out_edges = |i: &mut usize, name_to_id: &mut Vec<u32>, next_id: &mut u32, in_edges: &mut Vec<i32>| -> Vec<u32> {
        let mut edges = Vec::new();

        while data[*i] as char != '\n' {
            *i += 2;
            let name = get_name(*i);
            let id = get_id(name, name_to_id, next_id);

            edges.push(id);
            in_edges[id as usize] += 1;
            *i += 2;
        }

        edges
    };

    let mut next_id = 0;
    let mut name_to_id = vec![INFINITE; 26*26];
    let mut broadcasted = Vec::new();
    let mut out_edges = vec![Vec::new(); 26*26];
    let mut node_types = vec![Type::FlipFlop(0); 26*26];
    let mut in_edges = vec![0; 26*26];

    let mut i = 0;
    while i < data.len() {
        let c = data[i] as char;
        if c == 'b' {
            i += 13;
            broadcasted = read_out_edges(&mut i, &mut name_to_id, &mut next_id, &mut in_edges);
        }
        if c == '%' || c == '&' {
            let id = get_id(get_name(i+1), &mut name_to_id, &mut next_id);
            if c == '&' {
                node_types[id as usize] = Type::Conjunction(0);
            }
            i += 5;
            out_edges[id as usize] = read_out_edges(&mut i, &mut name_to_id, &mut next_id, &mut in_edges);
        }
        i += 1;
    }

    let node_count = next_id as usize;

    for i in 0..node_count {
        if let Type::Conjunction(n) = &mut node_types[i] {
            *n = in_edges[i] as u8;
        }
    }

    // if do_print {
    //     for i in 0..node_count {
    //         print!("{} -> ", i);
    //         for k in out_edges[i].iter() {
    //             print!("{}, ", k);
    //         }
    //         println!();
    //     }
    //     print!("Broadcast: ");
    //     for k in broadcasted.iter() {
    //         print!("{}, ", k);
    //     }
    //     println!();
    //     println!("In edges: {:?}", &in_edges[..node_count]);
    //     println!("Node types: {:?}", &node_types[..node_count]);
    // }

    const LOW_SIGNAL: bool = true;
    const HIGH_SIGNAL: bool = false;
    
    let mut last_input = vec![LOW_SIGNAL; (node_count+1)*node_count];

    // let rx = name_to_id[('r' as usize - 'a' as usize)*26 + 'x' as usize - 'a' as usize] as usize;
    // if do_print {
    //     println!("Id of rx = {}", rx);
    // }

    let mut press = |sent_low: &mut u32, sent_high: &mut u32| {
        *sent_low += 1;

        let mut q = VecDeque::new();
        for b in broadcasted.iter() {
            q.push_back((node_count, *b | 0x10000));
        }

        while let Some((from, id)) = q.pop_front() {
            let signal_type = id & 0x10000 > 0;
            let id = (id & 0x0FFFF) as usize;

            if signal_type {
                *sent_low += 1;
            } else {
                *sent_high += 1;
            }

            // if do_print {
            //     println!("Processing a {} signal between {} -> {}", if signal_type {"LOW"} else {"HIGH"}, from, id);
            // }

            let (signal, new_type) = match (signal_type, node_types[id]) {
                (LOW_SIGNAL, Type::FlipFlop(0)) => (Some(HIGH_SIGNAL), Type::FlipFlop(1)),
                (LOW_SIGNAL, Type::FlipFlop(1)) => (Some(LOW_SIGNAL), Type::FlipFlop(0)),
                (s, Type::Conjunction(mut n)) => {
                    let li = &mut last_input[id*(node_count+1) + from];
                    if *li != s {
                        if s == HIGH_SIGNAL {
                            n -= 1;
                        } else {
                            n += 1;
                        }
                    }
                    *li = s;

                    (Some(n == 0), Type::Conjunction(n))
                }
                (_, t) => (None, t),
            };

            // if do_print {
            //     print!(" -> Node became {:?}", new_type);
            //     if let Some(true) = signal {
            //         print!(" and sent LOW signal");
            //     }
            //     if let Some(false) = signal {
            //         print!(" and sent HIGH signal");
            //     }
            //     println!();
            //     println!();
            // }

            node_types[id as usize] = new_type;
            if let Some(new_signal) = signal {
                for to in out_edges[id as usize].iter() {
                    q.push_back((id, *to as u32 | if new_signal {0x10000} else {0}));
                }
            }
        }
    };

    let mut sent_high = 0;
    let mut sent_low  = 0;

    for _i in 1..1000 {
        press(&mut sent_low, &mut sent_high);
    }
    
    fn gcd(mut m: u64, mut n: u64) -> u64 {
        while m != 0 {
            let old_m = m;
            m = n % m;
            n = old_m;
        }
        n
    }

    let mut finish_time = 1u64;

    for &s in broadcasted.iter() {
        let mut period = 0;
        let mut bit = 1;
        let mut i = Some(s);
        while let Some(id) = i {
            let mut next = None;
            for &to in out_edges[id as usize].iter() {
                if matches!(node_types[to as usize], Type::Conjunction(_)) {
                    period |= bit;
                } else {
                    next = Some(to);
                }
            }
            i = next;
            bit <<= 1;
        }
        // if do_print { println!("Found: {}", period); }
        finish_time = finish_time * period / gcd(finish_time, period);
    }
    
    if do_print {
        println!("Problem 20 A: {}", sent_low * sent_high);
        println!("Problem 20 B: {}", finish_time);
    }
}

fn problem21ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/21.in").unwrap();

    let mut w = 0;
    while data[w] as char != '\n' {
        w += 1;
    }
    let w = w;

    let h = data.len() / (w + 1);

    // if do_print {
    //     println!("W = {}, H = {}", w, h);
    // }

    let mut blocked = vec![false; w*h];
    let mut start = (0,0);

    for i in 0..h {
        for j in 0..w {
            match data[i*(w+1) + j] as char {
                '#' => blocked[i*w+j] = true,
                'S' => start = (i,j),
                _ => ()
            }
        }
    }

    // if do_print {
    //     println!("S = {:?}", start);
    //     for i in 0..h {
    //         for j in 0..w {
    //             if blocked[i*w+j] {
    //                 print!("#");
    //             } else {
    //                 print!(".");
    //             }
    //         }
    //         println!()
    //     }
    //     println!();
    // }

    const INFINITE: u32 = 0xFFFFFFFF;
    let mut first_reach = vec![INFINITE; w*h];
    first_reach[start.0*w + start.1] = 0;

    let mut q = Vec::new();
    q.push(start);

    let mut t = 0;
    while !q.is_empty() {
        t += 1;
        let mut q2 = Vec::new();
        for (i,j) in q.into_iter() {
            if i > 0 && !blocked[(i-1)*w+j] && first_reach[(i-1)*w+j] == INFINITE {
                first_reach[(i-1)*w+j] = t;
                q2.push((i-1, j));
            }
            if i+1 < h && !blocked[(i+1)*w+j] && first_reach[(i+1)*w+j] == INFINITE {
                first_reach[(i+1)*w+j] = t;
                q2.push((i+1, j));
            }
            if j > 0 && !blocked[i*w+j-1] && first_reach[i*w+j-1] == INFINITE {
                first_reach[i*w+j-1] = t;
                q2.push((i, j-1));
            }
            if i+1 < h && !blocked[i*w+j+1] && first_reach[i*w+j+1] == INFINITE {
                first_reach[i*w+j+1] = t;
                q2.push((i, j+1));
            }
        }
        q = q2;
    }

    // if do_print {
    //     for i in 0..h {
    //         for j in 0..w {
    //             if blocked[i*w+j] {
    //                 print!("   _");
    //             } else if first_reach[i*w+j] == INFINITE {
    //                 print!(" inf");
    //             } else {
    //                 print!("{:>4}", first_reach[i*w+j]);
    //             }
    //         }
    //         println!()
    //     }
    //     println!();
    // }

    let mut reachable_exactly = 0;

    // if do_print {
    //     for i in 0..h {
    //         for j in 0..w {
    //             if blocked[i*w+j] {
    //                 print!("#");
    //             } else {
    //                 if first_reach[i*w+j] <= steps {
    //                     if (steps - first_reach[i*w+j]) % 2 == 0 {
    //                         print!("O");
    //                     } else {
    //                         print!("_");
    //                     }
    //                 } else {
    //                     print!(".");
    //                 }
    //             }
    //         }
    //         println!()
    //     }
    //     println!()
    // }
    
    let steps = 64;
    for i in 0..h {
        for j in 0..w {
            if !blocked[i*w+j] && first_reach[i*w+j] <= steps {
                if (steps - first_reach[i*w+j]) % 2 == 0 {
                    reachable_exactly += 1;
                }
            }
        }
    }

    if w != h {
        panic!("I assumed w=h but w={} and h={}",w,h);
    }

    let ultra_step: i64 = 26501365;

    // // Proof that this is set up extremely nicely: 
    // if do_print {
    //     for i in 0..h {
    //         for j in 0..w {
    //             if !blocked[i*w+j] && first_reach[i*w+j] < INFINITE {
    //                 let di = (start.0 as i64 - i as i64).abs();
    //                 let dj = (start.1 as i64 - j as i64).abs();
    //                 let d = di+dj;
    //                 if d % 2 == ultra_step % 2 {
    //                     if (ultra_step - d) % w as i64 == 0 {
    //                         print!("O");
    //                     } else {
    //                         if d < ultra_step % w as i64 {
    //                             print!("|");
    //                         } else {
    //                             print!(".");
    //                         }
    //                     }
    //                 } else {
    //                     print!("_");
    //                 }
    //             } else {
    //                 print!("#");
    //             }
    //         }
    //         println!()
    //     }
    // }

    //           ?      O      ?          
    //          ?#?    O#O    ?#?         
    //         ?###?  O###O  ?###?        
    //        ?#####?O#####O?#####?       
    //         ?###?O ?###? O?###?        
    //          ?#?O   ?#?   O?#?         
    //           ?O     ?     O?          
    //    ?      O             O      ?   
    //   ?#?    O#?           ?#O    ?#?  
    //  ?###?  O###?         ?###O  ?###? 
    // ?#####?O#####?       ?#####O?#####?
    //  ?###?O ?###?         ?###? O?###? 
    //   ?#?O   ?#?           ?#?   O?#?  
    //    ?O     ?             ?     O?   
    //    O                           O   
    //   O#?                         ?#O  
    //  O###?                       ?###O 
    // O#####?                     ?#####O
    //  O###?                       ?###O 
    //   O#?                         ?#O  
    //    O                           O   
    //    ?O     ?             ?     O?   
    //   ?#?O   ?#?           ?#?   O?#?  
    //  ?###?O ?###?         ?###? O?###? 
    // ?#####?O#####?       ?#####O?#####?
    //  ?###?  O###?         ?###O  ?###? 
    //   ?#?    O#?           ?#O    ?#?  
    //    ?      O             O      ?   
    //           ?O     ?     O?          
    //          ?#?O   ?#?   O?#?         
    //         ?###?O ?###? O?###?        
    //        ?#####?O#####O?#####?       
    //         ?###?  O###O  ?###?        
    //          ?#?    O#O    ?#?         
    //           ?      O      ?          

    
    // corners:
    //   CC?  
    //  CC#C? 
    // CC###C?
    // C#####?
    // ?C###??
    //  ?C#?? 
    //   ???  

    // corners: k=3
    //    cC?  
    //   cC#C? 
    //  cC#_#C?
    // cC#_#_#C?
    // C#_#_#_#?
    // ?C#_#_#??
    //  ?C#_#??
    //   ?C#?? 
    //    ???  

    // bodies: k=2
    //   ?B?  
    //  ?B#B? 
    // ?B###B?
    // B#####B
    // ?B###B?
    //  ?B#B? 
    //   ?B?  

    // w-1 + w*k <= u
    // k <= (u - w + 1) / w
    let k = (ultra_step as i64 - w as i64 + 1) / w as i64;
    let always_reachable_even = k*k; // _
    let always_reachable_odd = (k+1)*(k+1); // #
    let corners_reachable_even = 3*k+2; // C
    let corners_reachable_odd = k+1; // c
    let insides_reachable_even = (k+1)*4; // B
    let insides_reachable_odd = 0; // b

    // if do_print {
    //     println!("k={}",k);
    // }

    let mut total = 0;

    for i in 0..h {
        for j in 0..w {
            if !blocked[i*w+j] && first_reach[i*w+j] < INFINITE {
                let di = (start.0 as i64 - i as i64).abs();
                let dj = (start.1 as i64 - j as i64).abs();
                let d = di+dj;
                if d % 2 == ultra_step % 2 + k%2 - 1 {
                    total += always_reachable_even;

                    if d <= ultra_step % (w as i64) {
                        total += insides_reachable_even;
                    } else {
                        total += corners_reachable_even;
                    }
                } else {
                    total += always_reachable_odd;

                    if d <= ultra_step % (w as i64) {
                        total += insides_reachable_odd;
                    } else {
                        total += corners_reachable_odd;
                    }
                }
            }
        }
    }

    if do_print {
        println!("Problem 21 A: {}", reachable_exactly);
        println!("Problem 21 B: {}", total);
    }
}

fn problem22ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/22.in").unwrap();

    let read_number = |i: &mut usize| {
        let mut num = 0;
        while ('0'..='9').contains(&(data[*i] as char)) {
            num = num * 10 + data[*i] as u32 - '0' as u32;
            *i+=1; 
        }

        num
    };

    type Dim = u32;

    #[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
    struct Brick {
        z0: Dim,
        x0: Dim,
        y0: Dim,
        z1: Dim,
        x1: Dim,
        y1: Dim,
    }

    let mut bricks = vec![];

    let mut i = 0;
    while i < data.len() {
        let x0 = data[i] as Dim - '0' as Dim;
        let y0 = data[i+2] as Dim - '0' as Dim;
        i += 4;
        let z0 = read_number(&mut i);
        i += 1;
        let x1 = data[i] as Dim - '0' as Dim;
        let y1 = data[i+2] as Dim - '0' as Dim;
        i += 4;
        let z1 = read_number(&mut i);
        i += 1;

        bricks.push(Brick{x0,y0,z0,x1,y1,z1});

        // if x0 > x1 || y0 > y1 || z0 > z1 {panic!("Expected order, but got: {},{},{} - {},{},{}",x0,y0,z0,x1,y1,z1);}
    }

    bricks.sort();

    // if do_print {
    //     for b in bricks.iter() {
    //         println!("{:?}", b);
    //     }
    // }

    const INFINITE: u32 = 0xFFFFFFFF;

    let mut height = vec![1; 10*10];
    let mut top_brick_id = vec![INFINITE; 10*10];

    let n = bricks.len();

    let mut can_be_disintegrated = vec![true; n];
    let mut last_supported = vec![INFINITE; n];

    let mut children = vec![vec![]; n];
    let mut parents = vec![0; n];

    for i in 0..n {
        let b = &bricks[i];

        let mut maxh = 0;
        let mut supporters = 0;
        let mut one_supporter = 0;

        for x in b.x0..=b.x1 {
            for y in b.y0..=b.y1 {
                let id = (x*10+y) as usize;

                maxh = max(maxh, height[id]);
            }
        }

        for x in b.x0..=b.x1 {
            for y in b.y0..=b.y1 {
                let id = (x*10+y) as usize;

                if height[id] == maxh {
                    let top_id = top_brick_id[id];
                    if top_id != INFINITE {
                        if last_supported[top_id as usize] != i as u32 {
                            last_supported[top_id as usize] = i as u32;
                            children[top_id as usize].push(i as u32);
                            parents[i] += 1;
                            supporters += 1;
                            one_supporter = top_id;
                        }
                    }
                }
            }
        }

        for x in b.x0..=b.x1 {
            for y in b.y0..=b.y1 {
                let id = (x*10+y) as usize;

                height[id] = maxh + b.z1 - b.z0 + 1;
                top_brick_id[id] = i as u32;
            }
        }
               
        if supporters == 1 && one_supporter != INFINITE {
            can_be_disintegrated[one_supporter as usize] = false;
        }
    }

    let disint = can_be_disintegrated.iter().map(|b| *b as i32).sum::<i32>();

    enum Action {
        PushChildren,
        Evaluate
    }

    // if do_print {
    //     for i in 0..n {
    //         println!("Node {}: parents={}, children={:?}", i, parents[i], children[i]);
    //     }
    //     println!();
    // }

    let mut falls = vec![0; n];
    
    let mut rev_topo_sorted = vec![];
    let mut visited = vec![false; n];

    for i in 0..n {
        if !visited[i] {
            visited[i] = true;
            let mut q = vec![(Action::PushChildren, i as u32)];

            while let Some((a, k)) = q.pop() {
                match a {
                    Action::PushChildren => {
                        q.push((Action::Evaluate, k));
                        for c in children[k as usize].iter() {
                            if !visited[*c as usize] {
                                visited[*c as usize] = true;
                                q.push((Action::PushChildren, *c));
                            }
                        }
                    },
                    Action::Evaluate =>  {
                        rev_topo_sorted.push(k);
                    },
                }
            }
        }
    }

    if rev_topo_sorted.len() != n {
        panic!("Toposorted n = {} vs n = {}", rev_topo_sorted.len(), n);
    }

    let mut compressed = vec![0; n];

    for i in rev_topo_sorted {
        let current = i as usize;

        if children[current].len() == 0 {
            falls[current] = 1;
            continue;
        }

        if children[current].len() == 1 {
            let child = children[current][0] as usize;

            if parents[child] == 1 {
                compressed[current] = compressed[child] + 1;
                children.swap(current, child);

                falls[current] = falls[child] + 1;

                continue;
            }
        }

        let mut pillars = parents.clone();

        let mut q = Vec::new();
        q.push(i);

        let mut will_fall = 0;

        while let Some(k) = q.pop() {
            will_fall += 1 + compressed[k as usize];
            
            for child in children[k as usize].iter() {
                pillars[*child as usize] -= 1;
                if pillars[*child as usize] == 0 {
                    q.push(*child);
                }
            }
        }

        falls[i as usize] = will_fall;
    }

    let total_fall = falls.iter().map(|k| *k-1).sum::<i32>();

    // 79122
    if do_print {
        println!("Problem 22 A: {}", disint);
        println!("Problem 22 B: {}", total_fall);
    }

}

fn problem23ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/23.in").unwrap();
    
    let mut w = 0;
    while data[w] as char != '\n' {w += 1;}
    let w = w;
    let h = data.len() / (w+1);

    let mut field = vec!['K'; w*h];
    for i in 0..h {
        for j in 0..w {
            field[i*w+j] = data[i*(w+1)+j] as char;
        }
    }

    let mut visited = vec![false; w*h];
    let mut in_edges = vec![0; w*h];
    in_edges[1] = 1;
    visited[1] = true;
    
    let mut q = vec![(-1i32, 1i32, 0i32, 1i32)];

    while let Some((prev_i, prev_j, i,j)) = q.pop() {
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if i+di != prev_i || j+dj != prev_j {
                if i+di >= 0 && i+di < h as i32 && j+dj >= 0 && j+dj < w as i32 {
                    let next = (i+di) as usize * w + (j+dj) as usize;
                    let c = field[next];
    
                    if c == '.' || (c == '>' && dj == 1) || (c == 'v' && di == 1) {
                        in_edges[next] += 1;
                        if !visited[next] {
                            visited[next] = true;
                            q.push((i,j,i+di,j+dj));
                        }
                    }
                }
            }
        }
    }

    // if do_print {
    //     for i in 0..h {
    //         for j in 0..w {
    //             if in_edges[i*w+j] < 1 {
    //                 print!(" ");
    //             } else {
    //                 print!("{}", in_edges[i*w+j]);
    //             }
    //         }
    //         println!();
    //     }
    //     println!();
    // }

    let mut q = vec![(0i32,1i32)];
    in_edges[1] -= 1;

    // let mut maxd = vec![0; w*h];

    let mut longest_hike = 0;

    let mut d = 0;

    while !q.is_empty() {
        let mut q2 = vec![];

        while let Some((i,j)) = q.pop() {
            // maxd[i as usize * w + j as usize] = d;

            if i == h as i32 - 1 {
                longest_hike = d;
            }
    
            for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if i+di >= 0 && i+di < h as i32 && j+dj >= 0 && j+dj < w as i32 {
                    let next = (i+di) as usize * w + (j+dj) as usize;
                    let c = field[next];
    
                    if c == '.' || (c == '>' && dj == 1) || (c == 'v' && di == 1) {
                        in_edges[next] -= 1;
                        if in_edges[next] == 0 {
                            q2.push((i+di,j+dj));
                        }
                    }
                }
            }
        
            // if do_print {
            //     for i in 0..h {
            //         for j in 0..w {
            //             if maxd[i*w+j] < 1 {
            //                 print!(" []");
            //             } else {
            //                 print!("{:>3}", maxd[i*w+j]);
            //             }
            //         }
            //         println!();
            //     }
            //     println!();
            // }
        }

        q = q2;
        d += 1;
    }

    let mut ids = vec![-1; w*h];
    ids[1] = 0;
    let mut next_id: i32 = 1;

    for p in w..w*(h-1) {
        if field[p] as char != '#' {
            let cnt = (field[p-1] as char != '#') as i32 + 
                           (field[p-w] as char != '#') as i32 + 
                           (field[p+1] as char != '#') as i32 + 
                           (field[p+w] as char != '#') as i32;
            if cnt > 2 {
                ids[p] = next_id;
                next_id += 1;
            } else {
                ids[p] = -2;
            }
        }
    }

    ids[w*h-2] = next_id;
    next_id += 1;

    let node_count = next_id as usize;
        
    // if do_print {
    //     for i in 0..h {
    //         for j in 0..w {
    //             if ids[i*w+j] == -1 {
    //                 print!(" []");
    //             } else if ids[i*w+j] == -2 {
    //                 print!(" ..");
    //             } else {
    //                 print!("{:>3}", ids[i*w+j]);
    //             }
    //         }
    //         println!();
    //     }
    //     println!();
    // }


    
    struct Cursor {
        p: i32,
        parent: i32,
        distance: u32
    }

    let mut visited = vec![false; w*h];

    let mut q = vec![Cursor {p: 1, parent: 0, distance: 0}];

    let mut graph = vec![vec![]; node_count];

    while let Some(Cursor{p, parent, distance}) = q.pop() {
        if visited[p as usize] {continue;}
        visited[p as usize] = true;

        for d in [-(w as i32), -1, 1, w as i32] {
            if p+d >= 0 && p+d < (w*h) as i32 {
                let next = (p+d) as usize;

                if field[next] != '#' {
                    if ids[next] > -1 && ids[next] != parent {
                        graph[parent as usize].push((ids[next], distance + 1));
                        graph[ids[next] as usize].push((parent, distance + 1));
                    }
                }

                if !visited[next] && field[next] != '#' {
                    if ids[next] > -1 {
                        q.push(Cursor{
                            p: next as i32,
                            parent: ids[next],
                            distance: 0
                        });
                    } else {
                        q.push(Cursor{
                            p: next as i32,
                            parent,
                            distance: distance+1
                        });
                    }
                }
            }
        }
    }

    // if do_print {
    //     for i in 0..node_count {
    //         println!("{} -> {:?}", i, graph[i]);
    //     }
    // }

    if node_count >= 64 {
        panic!("Expected up to 63 junctions but found {}", node_count);
    }

    let mut q = Vec::new();
    q.push((0, 0, 1u64));

    let mut maximum_length = 0;

    while let Some((p, distance, visited)) = q.pop() {
        for (next, length) in graph[p as usize].iter() {
            if *next as usize == node_count-1 {
                if maximum_length < distance + length {
                    maximum_length = distance + length;
                }
            }

            if visited & (1 << next) == 0 {
                q.push((*next, distance + length, visited | (1 << next)));
            }
        }
    }

    if do_print {
        println!("Problem 23 A: {}", longest_hike);
        println!("Problem 23 B: {}", maximum_length);
    }
    
}


fn problem24ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/24.in").unwrap();

    let read_number = |i: &mut usize| {
        let mut num = 0;
        let mut neg = false;

        if data[*i] as char == '-' {
            neg = true;
            *i+=1;
        } else if data[*i] as char == ' ' {
            *i+=1;
        }
        
        while ('0'..='9').contains(&(data[*i] as char)) {
            num *= 10;
            num += data[*i] as i64;
            num -= '0' as i64;
            *i+=1; 
        }

        if neg {-num} else {num}
    };

    let mut rays = vec![];

    let mut i = 0;

    while i < data.len() {
        let x = read_number(&mut i); i += 2;
        let y = read_number(&mut i); i += 2;
        let z = read_number(&mut i); i += 3;
        let vx = read_number(&mut i) as i32; i += 2;
        let vy = read_number(&mut i) as i32; i += 2;
        let vz = read_number(&mut i) as i32; i += 1;

        // if do_print { println!("p=({},{},{}), v=({},{},{})", x,y,z, vx,vy,vz); }

        rays.push(([x,y,z], [vx,vy,vz]));
    }

    let mut intersections = 0;

    let n = rays.len();

    let min_coord = if folder.contains("small") {7i128} else {200000000000000i128};
    let max_coord = if folder.contains("small") {27i128} else {400000000000000i128};

    for i in 0..n {
        let ([x0,y0,_],[vx0,vy0,_]) = rays[i];
        for j in i+1..n {
            let ([x1,y1,_],[vx1,vy1,_]) = rays[j];
 
            // if do_print {println!("\nConsidering {} vs {}: (({},{}),({},{})) vs (({},{}),({},{}))",i,j, x0,y0,vx0,vy0, x1,y1,vx1,vy1);}
            
            // x0 + vx0 * t0 = x1 + vx1 * t1
            // y0 + vy0 * t0 = y1 + vy1 * t1

            // x0 * vy0 + vx0 * vy0 * t0 = x1 * vy0 + vx1 * vy0 * t1
            // y0 * vx0 + vy0 * vx0 * t0 = y1 * vx0 + vy1 * vx0 * t1

            // x0 * vy0 - y0 * vx0 = x1 * vy0 - y1 * vx0 + (vx1 * vy0 - vy1 * vx0) * t1
            // A = B + C * t1

            let mut a = (x0 * vy0 as i64 - y0 * vx0 as i64) - (x1 * vy0 as i64 - y1 * vx0 as i64);
            let mut b = vx1 * vy0 - vy1 * vx0;
            // t1 = a / b

            if b == 0 {
                // if do_print {println!(" -> Parallel")}
                continue;
            }

            if a.signum() as i32 * b.signum() < 0 {
                // if do_print {println!(" -> t_1 < 0")}
                continue;
            }

            if b < 0 { a *= -1; b *= -1; }

            let a = a as i128;
            let b = b as i128;
            // min <= x1 + vx1 * a / b <= max
            if min_coord * b > x1 as i128 * b + vx1 as i128 * a || 
               x1 as i128 * b + vx1 as i128 * a > max_coord * b || 
               min_coord * b > y1 as i128 * b + vy1 as i128 * a || 
               y1 as i128 * b + vy1 as i128 * a > max_coord * b {
                // if do_print {println!(" -> intersection outside of defined area");}
                continue;
            }

            // x0 + vx0 * t0 = x1 + vx1 * t1
            // y0 + vy0 * t0 = y1 + vy1 * t1

            // x0 * vy1 + vx0 * vy1 * t0 = x1 * vy1 + vx1 * vy1 * t1
            // y0 * vx1 + vy0 * vx1 * t0 = y1 * vx1 + vy1 * vx1 * t1

            // x0 * vy1 - y0 * vx1 + (vx0 * vy1 - vy0 * vx1) * t0 = x1 * vy1 - y1 * vx1

            let c = (x1 * vy1 as i64 - y1 * vx1 as i64) - (x0 * vy1 as i64 - y0 * vx1 as i64);
            let d = vx0 * vy1 - vy0 * vx1;

            if c.signum() as i32 * d.signum() < 0 {
                // if do_print {println!(" -> t_1 < 0")}
                continue;
            }

            // if do_print {println!(" -> intersection fine");}

            intersections += 1;
        }
    }
    // x + vx * t0 = x0 + vx0 * t0
    // x + vx * t1 = x1 + vx1 * t1
    // x + vx * t2 = x2 + vx2 * t2
    //
    // y + vy * t0 = y0 + vy0 * t0
    // y + vy * t1 = y1 + vy1 * t1
    // y + vy * t2 = y2 + vy2 * t2
    //
    // z + vz * t0 = z0 + vz0 * t0
    // z + vz * t1 = z1 + vz1 * t1
    // z + vz * t2 = z2 + vz2 * t2
    //
    // Unknowns: x,y,z, vx,vy,vz, t0,t1,t2
    // Equations: 9
    // -> Should be solvable
    // 
    // (x - x_i) + (vx - vx_i) * t = 0
    // 
    // f_x_i(x, vx, t_i) = (x - x_i) + (vx - vx_i) * t_i
    // 
    // E = SUM(f_x_i^2 + f_y_i^2 + f_z_i^2, i=0..K)
    // E = 0 => solution
    //  
    // d f_x_i^2 / d x = 2 * f_x_i
    // d f_x_i^2 / d vx = 2 * f_x_i * t_i
    // d f_x_i^2 / d t_i = 2 * f_x_i * (vx - vx_i)
    //  
    // d E / d r = SUM(d f_x_i^2 / dr + d f_y_i^2 / dr + d f_z_i^2 / dr, i=0..K)
    // 
    // Given t0 and t1 we can figure t2 out. Can we binary search?
    // 
    // x + vx * t0 = x0 + vx0 * t0
    // x + vx * t1 = x1 + vx1 * t1
    // A) vx = ((x1 + vx1 * t1) - (x0 + vx0 * t0)) / (t1 - t0)
    // B) x = x1 + vx1 * t1 - vx * t1
    // x + vx * t2 = x2 + vx2 * t2
    // C) t2 = (x2 - x) / (vx - vx2)
    // 
    // y + vy * t0 = y0 + vy0 * t0
    // y + vy * t1 = y1 + vy1 * t1
    // y + vy * t2 = y2 + vy2 * t2
    //
    // z + vz * t0 = z0 + vz0 * t0
    // z + vz * t1 = z1 + vz1 * t1
    // z + vz * t2 = z2 + vz2 * t2
    //
    //
    //
    // (x+y+z) + (vx+vy+vz) * t0 = (x0+y0+z0) + (vx0+vy0+vz0) * t0
    // p + v * t0 = p0 + v0 * t0
    // p + v * t1 = p1 + v1 * t1
    // 
    // 
    // vx * t0 * t1 = (x0 + vx0 * t0 - x) * t1
    // vx * t1 * t0 = (x1 + vx1 * t1 - x) * t0
    // 
    // (x1 + vx1 * t1 - x) * t0 = (x0 + vx0 * t0 - x) * t1
    // x1*t0 + vx1*t1*t0 - x*t0 = x0*t1 + vx0*t0*t1 - x*t1
    // 
    //
    // (x - x0) / (vx0 - vx) = t0
    // (y - y0) / (vy0 - vy) = t0



    // t_i = (x_i - x) / (vx - vx_i) > 0
    // x_i < x && vx < vx_i
    // x_i > x && vx > vx_i
    // hopefully there's a small window where this is true for all i

    // If we know vx,vy,vz
    //
    // x + vx * t0 = x0 + vx0 * t0
    // x + vx * t1 = x1 + vx1 * t1
    //
    // y + vy * t0 = y0 + vy0 * t0
    // y + vy * t1 = y1 + vy1 * t1
    //
    // z + vz * t0 = z0 + vz0 * t0
    // z + vz * t1 = z1 + vz1 * t1
    //
    // --
    //
    // x = x0 + (vx0 - vx) * t0
    // x = x1 + (vx1 - vx) * t1
    //
    // y = y0 + (vy0 - vy) * t0
    // y = y1 + (vy1 - vy) * t1
    //
    // z = z0 + (vz0 - vz) * t0
    // z = z1 + (vz1 - vz) * t1
    //
    // --
    //
    // (vx1 - vx) * t1 = (x0 - x1) + (vx0 - vx) * t0
    // (vy1 - vy) * t1 = (y0 - y1) + (vy0 - vy) * t0
    // (vz1 - vz) * t1 = (z0 - z1) + (vz0 - vz) * t0
    //
    // --
    //
    // [(vx1-vx) (vx-vx0)] * [t1] = [(x0 - x1)]
    // [(vy1-vy) (vy-vy0)] * [t0] = [(y0 - y1)]
    // 
    // D = (vx1-vx) * (vy-vy0) - (vx-vx0) * (vy1-vy)
    // 
    // (vy1-vy)*(vx1-vx)*t1 = (vy1-vy)*(x0-x1) + (vy1-vy)*(vx0-vx)*t0
    // (vx1-vx)*(vy1-vy)*t1 = (vx1-vx)*(y0-y1) + (vx1-vx)*(vy0-vy)*t0
    // 
    // (vy1-vy)*(x0-x1) + (vy1-vy)*(vx0-vx)*t0 = (vx1-vx)*(y0-y1) + (vx1-vx)*(vy0-vy)*t0
    // (vy1-vy)*(x0-x1) - (vx1-vx)*(y0-y1) = ((vx1-vx)*(vy0-vy) - (vy1-vy)*(vx0-vx)) * t0
    // a = b*t0
    // 
    // t1 = ((x0 - x1) + (vx0 - vx) * t0) / (vx1 - vx)


    let find_limits = |dim: usize| {
        let mut rays_sorted_x = rays.iter().map(|(p,v)| (p[dim], v[dim])).collect::<Vec<_>>();
        rays_sorted_x.sort();

        // if do_print {
        //     for k in rays_sorted_x[..10].iter() {
        //         println!("{:?}",k);
        //     }
        // }

        let mut vxs: Vec<_> = rays_sorted_x.iter().map(|(_p,v)| *v).collect();

        let mut vx_mins = Vec::with_capacity(vxs.len());
        vx_mins.push(vxs[0]);
        for i in 1..vxs.len()-1 {
            vx_mins.push(min(vx_mins[i-1], vxs[i]));
        }

        vxs.reverse();

        let mut vx_maxs = Vec::with_capacity(vxs.len());
        vx_maxs.push(vxs[0]);
        for i in 1..vxs.len()-1 {
            vx_maxs.push(max(vx_maxs[i-1], vxs[i]));
        }

        // if do_print {
        //     vxs.reverse();
        //     vx_maxs.reverse();
        //     println!("vxs: {:?}", &vxs[..17]);
        //     println!("vx_mins: {:?}", &vx_mins[..17]);
        //     println!("vx_maxs: {:?}", &vx_maxs[..17]);
        //     vx_maxs.reverse();
        //     vxs.reverse();
        // }

        let limits = vx_mins.into_iter().zip(vx_maxs.into_iter().rev())
                                      .filter(|(mn,mx)| *mn >= *mx)
                                      .collect::<Vec<_>>();

        // if do_print {
        //     println!("Limits: {:?}", limits);
        // }
        limits
    };

    let vxlims = find_limits(0);
    let vylims = find_limits(1);
    let vzlims = find_limits(2);

    #[allow(unused)]
    let mut counter = 0;

    let vx0 = rays[0].1[0];
    let vy0 = rays[0].1[1];
    let vz0 = rays[0].1[2];
    let vx1 = rays[1].1[0];
    let vy1 = rays[1].1[1];
    let vz1 = rays[1].1[2];
    let x0 = rays[0].0[0];
    let y0 = rays[0].0[1];
    let z0 = rays[0].0[2];
    let x1 = rays[1].0[0];
    let y1 = rays[1].0[1];
    let z1 = rays[1].0[2];

    let mut magic_stone = 0;
    
    'find_loop: for (xmax, xmin) in vxlims.iter() {
        for (ymax, ymin) in vylims.iter() {
            for (zmax, zmin) in vzlims.iter() {
                for vx in *xmin..=*xmax {
                    for vy in *ymin..=*ymax {
                        for vz in *zmin..=*zmax {
                            // a = b*t0
                            let a = ((vy1-vy) as i64)*(x0-x1) - ((vx1-vx) as i64)*(y0-y1);
                            let b = ((vx1-vx)*(vy0-vy) - (vy1-vy)*(vx0-vx)) as i64;

                            if b != 0 && a % b == 0 {
                                let t0 = a / b;

                                let a = (x0 - x1) + (vx0 - vx) as i64 * t0;
                                let b = (vx1 - vx) as i64;

                                if b != 0 && a % b == 0 {
                                    let t1 = a / b;

                                    if (vz1 - vz) as i64 * t1 == (z0 - z1) + (vz0 - vz) as i64 * t0 {
                                        let x = x0 + (vx0 - vx) as i64 * t0;
                                        let y = y0 + (vy0 - vy) as i64 * t0;
                                        let z = z0 + (vz0 - vz) as i64 * t0;

                                        // if do_print {
                                        //     println!("Found candidate: p={},{},{} v={},{},{} t0={}, t1={}", x,y,z, vx,vy,vz, t0, t1);
                                        // }
                                        
                                        magic_stone = x+y+z;

                                        break 'find_loop;
                                    }
                                }
                            }
                            counter += 1;
                        }
                    }
                }
            }
        }
    }

    // if do_print {
    //     println!("Had to check {} velocities", counter);
    // }



    if do_print {
        println!("Problem 24 A: {}", intersections);
        println!("Problem 24 B: {}", magic_stone);
    }
}


fn problem25ab(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/25.in").unwrap();

    let mut ids = vec![-1; 26*26*26];
    let mut next_id: i32 = 0;

    let mut read_id = |i: usize| -> usize {
        let a = data[i+0] as usize - 'a' as usize;
        let b = data[i+1] as usize - 'a' as usize;
        let c = data[i+2] as usize - 'a' as usize;
        
        let name = a * 26 * 26 + b * 26 + c;
        if ids[name] < 0 {
            ids[name] = next_id;
            next_id += 1;
        }

        ids[name] as usize
    };

    let mut graph = vec![];

    let mut register_edge = |i: usize, j: usize| {
        if max(i,j) >= graph.len() {
            graph.resize(max(i,j)+1, vec![]);
        }

        graph[i].push(j as u32);
        graph[j].push(i as u32);
    };

    let mut i = 0;
    while i < data.len() {
        let from = read_id(i); i += 4;

        while data[i] as char == ' ' {
            i += 1;
            let to = read_id(i); i += 3;

            register_edge(from, to);
        }
        i+=1;
    }

    let n = graph.len();

    // if do_print {
    //     for i in 0..n {
    //         println!("{} -> {:?}", i, graph[i]);
    //     }
    // }

    // if do_print {
    //     let degrees = graph.iter().map(Vec::len);
    //     println!("Node count: {}", n);
    //     println!("Edge count: {}", degrees.clone().sum::<usize>());
    //     println!("Maximum degree: {}", degrees.max().unwrap());
    // }

    const INFINITE: u32 = 0xFFFFFFFF;

    let max_flow = |s: u32, t: u32| -> (usize, Vec<Vec<u32>>) {
        let mut out_edges = graph.clone();

        for flow in 0.. {
            let mut parents = vec![INFINITE; n];
            let mut q = Vec::new();
            let mut q2 = vec![];
            q.push(s);
            parents[s as usize] = s;

            // if do_print { println!("Looking for flow nr {}", flow) }
        
            'bfs: while !q.is_empty() {
                // if do_print {println!("  Q = {:?}", q)}

                while let Some(p) = q.pop() {
                    for next in out_edges[p as usize].iter() {
                        if *next != INFINITE && parents[*next as usize] == INFINITE {
                            parents[*next as usize] = p;
                            if *next == t {
                                break 'bfs;
                            }
                            q2.push(*next);
                        }
                    }
                }
                swap(&mut q, &mut q2);
            }

            if parents[t as usize] == INFINITE || flow == 4 {
                // if do_print { println!("Found flows: {} between {} and {}", flow, s, t) }
                return (flow, out_edges);
            }

            let mut c = t;
            while c != s {
                let p = parents[c as usize];

                for out in out_edges[p as usize].iter_mut() {
                    if *out == c {
                        *out = INFINITE;
                    }
                }

                c = p;
            }
        
            // if do_print {
            //     let mut c = t;
            //     print!(" -> Found path: {}", t);
            //     while c != s {
            //         c = parents[c];
            //         print!(" <- {}", c);
            //     }
            //     println!();
            // }
        }

        (0, out_edges)
    };

    let step = 524287;

    let mut other_group = 0;

    let mut i = step % n as u32;
    while i != 0 {
        let (flows, out_edges) = max_flow(0, i);

        if flows <= 3 {
            let mut q = vec![0u32];

            let mut visited = vec![false; n];
            visited[0] = true;

            while !q.is_empty() {
                while let Some(p) = q.pop() {
                    for next in out_edges[p as usize].iter() {
                        if *next != INFINITE && !visited[*next as usize] {
                            visited[*next as usize] = true;
                            q.push(*next);
                        }
                    }
                }
            }

            other_group = visited.into_iter().map(|b|b as usize).sum::<usize>();

            break;
        }

        i = (i + step) % n as u32;
    }

    let group_product = other_group * (n - other_group);

    if do_print {
        println!("Problem 25 A: {}", group_product);
    }
}

fn main() {
    let problems = [
        problem1ab,
        problem2ab,
        problem3ab,
        problem4ab,
        problem5ab,
        problem6ab,
        problem7ab,
        problem8ab,
        problem9ab,
        problem10ab,
        problem11ab,
        problem12ab,
        problem13ab,
        problem14ab,
        problem15ab,
        problem16ab,
        problem17ab,
        problem18ab,
        problem19ab,
        problem20ab,
        problem21ab,
        problem22ab,
        problem23ab,
        problem24ab,
        problem25ab,
    ];
    let folder = "input";

    let number_of_runs = 100;
    println!(
        "Running solutions {} times, to collect timing",
        number_of_runs
    );

    let mut total_time = Duration::new(0, 0);
    for f in problems {
        f(true, folder);

        let execute_f = || f(false, folder);
        let min_duration = std::iter::repeat(execute_f)
            .take(number_of_runs)
            .map(time_call)
            .min()
            .unwrap();
        println!("Elapsed: {}mics", min_duration.as_micros());
        println!();

        total_time += min_duration;
    }

    println!("Total time: {}mics", total_time.as_micros());
}

fn time_call<F>(f: F) -> Duration
where
    F: Fn() -> (),
{
    use std::time::Instant;
    let start = Instant::now();

    f();

    return start.elapsed();
}
