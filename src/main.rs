#![allow(dead_code)]

use core::fmt::{Debug, Display};
use std::cmp::{max, min};
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
        
        if card <= '9' && card >= '2' { return card as u8 - '2' as u8 + 1; }
        
        match card as char {
            'T' => 9,
            'J' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => 0
        }
    };
    
    let mut all_hands_a: [Vec<_>; 7] = Default::default();
    let mut all_hands_b: [Vec<_>; 7] = Default::default();
    
    let mut i = 0;
    while i < data.len() {
        let power_a;
        let power_b;
        {
            let cv0 = card_value(data[i+0]) as usize;
            let cv1 = card_value(data[i+1]) as usize;
            let cv2 = card_value(data[i+2]) as usize;
            let cv3 = card_value(data[i+3]) as usize;
            let cv4 = card_value(data[i+4]) as usize;
            
            power_a = ((cv0 as u32)<<16) + 
                    ((cv1 as u32)<<12) + 
                    ((cv2 as u32)<<8) + 
                    ((cv3 as u32)<<4) + 
                    ((cv4 as u32)<<0);
            
            let cv0 = if cv0 == 10 {0} else {cv0};
            let cv1 = if cv1 == 10 {0} else {cv1};
            let cv2 = if cv2 == 10 {0} else {cv2};
            let cv3 = if cv3 == 10 {0} else {cv3};
            let cv4 = if cv4 == 10 {0} else {cv4};
            
            power_b = ((cv0 as u32)<<16) + 
                    ((cv1 as u32)<<12) + 
                    ((cv2 as u32)<<8) + 
                    ((cv3 as u32)<<4) + 
                    ((cv4 as u32)<<0);
        }
        
        let o0 = 1
        + (data[i+0] == data[i+1]) as u32
        + (data[i+0] == data[i+2]) as u32
        + (data[i+0] == data[i+3]) as u32
        + (data[i+0] == data[i+4]) as u32;
        
        let o1 = 1
        + (data[i+1] == data[i+0]) as u32
        + (data[i+1] == data[i+2]) as u32
        + (data[i+1] == data[i+3]) as u32
        + (data[i+1] == data[i+4]) as u32;
        
        let o2 = 1
        + (data[i+2] == data[i+0]) as u32
        + (data[i+2] == data[i+1]) as u32
        + (data[i+2] == data[i+3]) as u32
        + (data[i+2] == data[i+4]) as u32;
        
        let o3 = 1
        + (data[i+3] == data[i+0]) as u32
        + (data[i+3] == data[i+2]) as u32
        + (data[i+3] == data[i+1]) as u32
        + (data[i+3] == data[i+4]) as u32;
        
        let o4 = 1
        + (data[i+4] == data[i+0]) as u32
        + (data[i+4] == data[i+2]) as u32
        + (data[i+4] == data[i+3]) as u32
        + (data[i+4] == data[i+1]) as u32;
        
        let js = 
          (data[i+0] == 'J' as u8) as u32
        + (data[i+1] == 'J' as u8) as u32
        + (data[i+2] == 'J' as u8) as u32
        + (data[i+3] == 'J' as u8) as u32
        + (data[i+4] == 'J' as u8) as u32;
        
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
        
        let strength_a = 
        if freqs[5] == 5 {FIVE_OF_A_KIND}
        else if freqs[4] == 4 {FOUR_OF_A_KIND}
        else if freqs[3] == 3 && freqs[2] == 2 {FULL_HOUSE}
        else if freqs[3] == 3 {THREE_OF_A_KIND}
        else if freqs[2] == 4 {TWO_PAIR}
        else if freqs[2] == 2 {ONE_PAIR}
        else {HIGH_CARD};
        
        let strength_b = if js == 0 {
            strength_a
        } else if strength_a >= FULL_HOUSE {FIVE_OF_A_KIND} 
          else if strength_a == THREE_OF_A_KIND {FOUR_OF_A_KIND}
          else if strength_a == ONE_PAIR {THREE_OF_A_KIND}
          else if strength_a == HIGH_CARD {ONE_PAIR}
          else {
            if js == 2 {FOUR_OF_A_KIND}
            else {FULL_HOUSE}
          };

        i+=6;
        
        let mut bet = 0;
        
        while data[i] as char != '\n' {
            bet = bet * 10 + data[i] as u32 - '0' as u32;
            i+=1;
        }
        i+=1; // linefeed \n
        
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
    
    struct BitVec
    {
        data: [u32; 10],
        len: usize,
    }
    
    impl BitVec {
        fn new() -> BitVec {BitVec{data:[0u32; 10], len:0}}
        
        fn add(&mut self, bit: bool)
        {
            if self.len >= self.data.len()*32 {
                panic!("Len too large: {}", self.len);
            }
            
            if bit {
                let items = self.len / 32;
                let offset = self.len - items*32;
                self.data[items] |= 1u32 << offset;
            }
            
            self.len += 1;
        }
        
        fn get(&self, index: usize) -> bool {
            let index = index;
            let items = index / 32;
            let offset = index - items*32;
            
            (self.data[items] & 1u32 << offset) > 0
        }
    }
    
    let mut bitvec = BitVec::new();
    
    let mut i = 0;
    while data[i] as char != '\n' {
        bitvec.add(data[i] as char == 'L');
        i+=1;
    }
    i+=2;
    
    macro_rules! read_id {
        ($arr:expr,$i:expr) => {
            ($arr[$i+0] as usize - 'A' as usize)*26*26 + 
            ($arr[$i+1] as usize - 'A' as usize)*26 +
            ($arr[$i+2] as usize - 'A' as usize)
        };
    }
    
    #[allow(unused_macros)]
    macro_rules! val_into_str {
        ($value:expr) => {
            [($value/26/26 + 'A' as usize) as u8 as char, (($value/26)%26 + 'A' as usize) as u8 as char, (($value)%26 + 'A' as usize) as u8 as char].into_iter().collect::<String>()
        };
    }
    
    let mut ids = [Default::default(); 26*26*26];
    let node_count;
    
    {
        let mut j = i;
        let mut next_id: u32 = 0;
        
        while j+16 < data.len() {
            let a = read_id!(data, j);
            j+=17;
            
            ids[a] = next_id;
            // if do_print { println!("{} got id {}", val_into_str!(a), next_id); }
            
            next_id += 1;
        }
        
        node_count = next_id as usize;
    }
    
    struct Node(u32);
    impl Node {
        fn make(left: u32, right: u32) -> Node {Node( ((left << 16) + right) as u32 )}
        fn left(&self) -> u32 { self.0 >> 16 }
        fn right(&self) -> u32 { self.0 & 0x0000FFFF }
    }
    
    impl Debug for Node {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("Node").field(&(self.0 >> 16)).field(&(self.0 & 0x0000FFFF)).finish()
        }
    }
    
    #[derive(Debug, Eq, PartialEq)]
    enum NodeType {
        Start,
        End,
        Normal
    }
    
    let mut children = Vec::with_capacity(node_count);
    let mut node_types = Vec::with_capacity(node_count);
    
    {
        let mut j = i;
        
        while j+16 < data.len() {
            let a = read_id!(data, j);
            let b = read_id!(data, j+7);
            let c = read_id!(data, j+12);
            j+=17;
            
            let ntype = match a%26 {
                0 => NodeType::Start,
                25 => NodeType::End,
                _ => NodeType::Normal
            };
            
            // if do_print {
            //     println!("A={}, type={:?}", val_into_str!(a), ntype);
            // }
            
            let b = ids[b];
            let c = ids[c];
            children.push(Node::make(b, c));
            node_types.push(ntype);
        }
    }
    
    const INFINITY: u32 = 0xFFFFFFFF;
    
    let mut dist_to_next_end = vec![INFINITY; node_count * bitvec.len];
    
    #[derive(Debug, Copy, Clone)]
    struct State(u32, u32);
    impl State {
        fn make(node_id: u32, decision_id: u32) -> State {State( node_id, decision_id )}
        fn node_id(&self) -> u32 { self.0 }
        fn decision_id(&self) -> u32 { self.1 }
        fn flat_id(&self, node_count: usize) -> usize {self.decision_id() as usize * node_count + self.node_id() as usize}
    }
    
    let mut stack = Vec::new();
    for j in 0..node_count {
        if node_types[j] == NodeType::Start {
            stack.push(State::make(j as u32, 0));
        }
    }
    
    let mut intermediate_nodes = 0;
    
    while let Some(current_state) = stack.last().copied() {
        let current_id = current_state.node_id() as usize;
        let decision_id = current_state.decision_id();
        let left = bitvec.get(decision_id as usize);
        
        let kids = &children[current_id];
        let next_id = if left {kids.left()} else {kids.right()};
        let next_state = State::make(next_id, (decision_id+1) % bitvec.len as u32);
        
        // if do_print {
        //     println!("At node {}, decision {}, kids {:?}, next is {}", 
        //               current_id, decision_id,      kids, next_id,
        //             );
        // }
        
        let has_next_been_processed = dist_to_next_end[next_state.flat_id(node_count)] != INFINITY;
        
        
        if node_types[next_id as usize] == NodeType::End {
            let mut distance = 1;
            while intermediate_nodes > 0 {
                if let Some(back_state) = stack.pop() {
                    dist_to_next_end[back_state.flat_id(node_count)] = distance;
                }
                
                intermediate_nodes -= 1;
                distance += 1;
            }
        } else {
            if has_next_been_processed {
                stack.pop();
                
                dist_to_next_end[current_state.flat_id(node_count)] = dist_to_next_end[next_state.flat_id(node_count)]+1;
                intermediate_nodes = 0;
            } else {
                stack.push(next_state);
                intermediate_nodes += 1;
            }
        }
    }
    
    let state_aaa = State::make(ids[0], 0);
    let answer_a = dist_to_next_end[state_aaa.flat_id(node_count)];
    
    let mut least_total_dist = 1;
    
    fn gcd(mut m: u64, mut n: u64) -> u64 {
        while m != 0 {
            let old_m = m;
            m = n % m;
            n = old_m;
        }
        n
    }
    
    // assume each ghost moves on a circle, with one start and one end
    for i in 0..node_count {
        if node_types[i] == NodeType::Start {
            let dist = dist_to_next_end[State::make(i as u32, 0).flat_id(node_count)] as u64;
            least_total_dist = (least_total_dist * dist) / gcd(least_total_dist, dist);
            
            // if do_print {
            //     println!("{} -> {}", i, dist);
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
                nums.push(num * if neg {-1} else {1});
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
        
        nums.push(num * if neg {-1} else {1});
        
        // if do_print {
        //     println!("Line contains numbers: {:?}", nums);
        // }
        
        let mut nums_b = nums.clone();
        
        let len = nums.len();
        let mut n = len - 1;
        while n >= 1 {
            for j in 0..n {
                nums[j] = nums[j+1] - nums[j];
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
            nums_b[n] = nums_b[n] - nums_b[n+1];
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

fn main() {
    let problems = [
        // problem1ab, problem2ab, problem3ab, problem4ab, problem5a, problem5b, problem6ab, problem7ab, problem8ab,
        problem9ab,
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
