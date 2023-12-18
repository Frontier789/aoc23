#![allow(dead_code)]

use core::fmt::{Debug, Display};
use std::cmp::{max, min};
use std::collections::BinaryHeap;
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

fn problem5a(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/5.in").unwrap();

    let mut i = 0;
    while data[i] as char != ':' {
        i += 1;
    }
    i += 1; // colon (:)

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

fn problem5b(do_print: bool, folder: &str) {
    let data = std::fs::read(folder.to_owned() + "/5.in").unwrap();

    let mut i = 0;
    while data[i] as char != ':' {
        i += 1;
    }
    i += 1; // colon (:)

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

    let evaluate_field = |field: &Vec<u8>| {
        let mut total_score = 0;

        for j in 0..h {
            for i in 0..w {
                if field[j*(w+1) + i] as char == 'O' {
                    total_score += h-j;
                }
            }
        }

        total_score
    };

    #[allow(unused_variables)]
    let print_field = |field: &Vec<u8>| {
        for i in 0..h {
            println!("{}", field[i*(w+1)..(i+1)*(w+1)-1].iter().map(|b|*b as char).collect::<String>());
        }
    };

    let step_north = |field: &mut Vec<u8>| {
        for i in 0..w {
            let mut dotj = 0;
            for j in 0..h {
                match field[j*(w+1)+i] as char {
                    'O' => {
                        if j > dotj {
                            field[dotj*(w+1)+i] = 'O' as u8;
                            field[j*(w+1)+i] = '.' as u8;
                        }
                        dotj += 1;
                    },
                    '#' => {dotj = j+1;},
                    _ => (),
                }
            }
        }
    };

    let step_south = |field: &mut Vec<u8>| {
        for i in 0..w {
            let mut dotj = h-1;
            for j in (0..h).rev() {
                match field[j*(w+1)+i] as char {
                    'O' => {
                        if j < dotj {
                            field[dotj*(w+1)+i] = 'O' as u8;
                            field[j*(w+1)+i] = '.' as u8;
                        }
                        dotj -= 1;
                    },
                    '#' => {dotj = j-1;},
                    _ => (),
                }
            }
        }
    };

    let step_west = |field: &mut Vec<u8>| {
        for j in 0..h {
            let mut doti = 0;
            for i in 0..w {
                match field[j*(w+1)+i] as char {
                    'O' => {
                        if i > doti {
                            field[j*(w+1)+doti] = 'O' as u8;
                            field[j*(w+1)+i] = '.' as u8;
                        }
                        doti += 1;
                    },
                    '#' => {doti = i+1;},
                    _ => (),
                }
            }
        }
    };

    let step_east = |field: &mut Vec<u8>| {
        for j in 0..h {
            let mut doti = w-1;
            for i in (0..w).rev() {
                match field[j*(w+1)+i] as char {
                    'O' => {
                        if i < doti {
                            field[j*(w+1)+doti] = 'O' as u8;
                            field[j*(w+1)+i] = '.' as u8;
                        }
                        doti -= 1;
                    },
                    '#' => {doti = i-1;},
                    _ => (),
                }
            }
        }
    };

    let step = |mut field: &mut Vec<u8>| {
        step_north(&mut field);
        step_west(&mut field);
        step_south(&mut field);
        step_east(&mut field);
    };
    
    let mut field_a = data.clone();
    step_north(&mut field_a);
    let score_a = evaluate_field(&field_a);
    // if do_print {
    //     println!("Northed field score={}", score_a);
    //     print_field(&field_a);
    // }

    let mut field = data.clone();

    let mut values = Vec::new();
    let mut i = 0;

    loop {
        let val = evaluate_field(&field);
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
        step(&mut field);
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

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    struct State {
        heat_loss: i32,
        x: i32,
        y: i32,
        next_step_hor: bool,
    }

    impl State {
        fn map_index(&self, w: usize) -> usize {
            self.y as usize * (w+1) + self.x as usize
        }
        fn visited_index(&self, w: usize) -> usize {
            self.map_index(w) * 2 + self.next_step_hor as usize
        }
    }

    let find_minimum_loss = |min_step: i32, max_step: i32| {
        let mut mlosses = vec![-1000000000; (w+1)*h*2];

        let mut pq = BinaryHeap::new();
        pq.push(State { heat_loss: 0, x: 0, y: 0, next_step_hor: true});
        pq.push(State { heat_loss: 0, x: 0, y: 0, next_step_hor: false});

        mlosses[State { heat_loss: 0, x: 0, y: 0, next_step_hor: true}.visited_index(w)] = 0;
        mlosses[State { heat_loss: 0, x: 0, y: 0, next_step_hor: false}.visited_index(w)] = 0;

        let mut minimum_loss = -1000000000;

        while let Some(s) = pq.pop() {
            if s.x as usize == w-1 && s.y as usize == h-1 {
                minimum_loss = s.heat_loss;
                break;
            }

            // if do_print {
            //     println!("Discovered minimum heat loss to {},{},{}. It is: {}", s.x, s.y, if s.next_step_hor {'H'} else {'V'}, s.heat_loss);
            // }
            // if do_print {
            //     for i in 0..h {
            //         for j in 0..w {
            //             let v = max(mlosses[(i*(w+1) + j)*2+0], mlosses[(i*(w+1) + j)*2+1]);
            //             if v == -1000000000 {print!(" [] ");} else {print!("{:>3} ", -v);}
            //         }
            //         println!();
            //     }
            //     println!();
            // }

            if s.next_step_hor {
                let mut s2 = s.clone();
                s2.next_step_hor = false;

                while s.x+max_step > s2.x && s2.x+1 < w as i32 {
                    s2.x += 1;
                    s2.heat_loss -= data[s2.map_index(w)] as i32 - '0' as i32;
                    if s2.x - s.x >= min_step {
                        if mlosses[s2.visited_index(w)] < s2.heat_loss {
                            mlosses[s2.visited_index(w)] = s2.heat_loss;
                            pq.push(s2);
                        }
                    }
                }

                s2.x = s.x;
                s2.heat_loss = s.heat_loss;

                while s.x < s2.x+max_step && s2.x > 0 {
                    s2.x -= 1;
                    s2.heat_loss -= data[s2.map_index(w)] as i32 - '0' as i32;
                    if s.x - s2.x >= min_step {
                        if mlosses[s2.visited_index(w)] < s2.heat_loss {
                            mlosses[s2.visited_index(w)] = s2.heat_loss;
                            pq.push(s2);
                        }
                    }
                }
            } else {
                let mut s2 = s.clone();
                s2.next_step_hor = true;

                while s.y+max_step > s2.y && s2.y+1 < h as i32 {
                    s2.y += 1;
                    s2.heat_loss -= data[s2.map_index(w)] as i32 - '0' as i32;
                    if s2.y - s.y >= min_step {
                        if mlosses[s2.visited_index(w)] < s2.heat_loss {
                            mlosses[s2.visited_index(w)] = s2.heat_loss;
                            pq.push(s2);
                        }
                    }
                }

                s2.y = s.y;
                s2.heat_loss = s.heat_loss;

                while s.y <= s2.y+max_step && s2.y > 0 {
                    s2.y -= 1;
                    s2.heat_loss -= data[s2.map_index(w)] as i32 - '0' as i32;
                    if s.y - s2.y >= min_step {
                        if mlosses[s2.visited_index(w)] < s2.heat_loss {
                            mlosses[s2.visited_index(w)] = s2.heat_loss;
                            pq.push(s2);
                        }
                    }
                }
            }
        }
        -minimum_loss
    };

    let answer_b = find_minimum_loss(4, 10);
    let answer_a = find_minimum_loss(1, 3);

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

fn main() {
    let problems = [
        // problem1ab,
        // problem2ab,
        // problem3ab,
        // problem4ab,
        // problem5a,
        // problem5b,
        // problem6ab,
        // problem7ab,
        // problem8ab,
        // problem9ab,
        // problem10ab,
        // problem11ab,
        // problem12ab,
        // problem13ab,
        // problem14ab,
        // problem15ab,
        // problem16ab,
        // problem17ab,
        problem18ab,
    ];
    let folder = "input";

    let number_of_runs = 1000;
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
