extern crate termion;

use itertools::Itertools;
use termion::{clear, cursor};
use rand::{Rng, thread_rng};
use std::io::{stdout, Write};
use std::time::Duration;
use std::thread::sleep;

const STAR1: &str = r"★≡≡≡≡≡≡";
const STAR2: &str = r"☆≡≡≡≡≡≡";
const STAR_COUNT: usize = 30;
const STAR_PERCENT: f32 = 0.05;
const DURATION: u64 = 50;

#[derive(Clone, Copy)]
struct StarAppearance {
    col: isize,
    row: isize,
    star: &'static str,
}

fn initialize(rows: usize) -> Vec<StarAppearance> { 
    let mut star_remain = STAR_COUNT;
    let mut star_list: Vec<StarAppearance> = Vec::new();
    let mut rng = thread_rng();

    'initialize: for curr_col in 0.. {
        for curr_row in 0..rows {
            if rng.gen::<f32>() < STAR_PERCENT {
                let star = match rng.gen::<usize>() % 2 {
                    0 => STAR1,
                    1 => STAR2,
                    _ => panic!("Should not reach here..."),
                };

                let appearance = StarAppearance {
                    col: curr_col as isize,
                    row: curr_row as isize,
                    star
                };
                star_list.push(appearance);
                star_remain -= 1;
            }

            if star_remain <= 0 {
                break 'initialize;
            } 
        } 
    }

    star_list
}

fn generate_frame(appears: &Vec<StarAppearance>, term_col: isize, term_row: isize, head_col: isize) -> Vec<String> {
    let shifted = appears
        .clone()
        .into_iter()
        .map(|mut ap| {
            ap.col += head_col;
            ap
        })
        .filter(|ap| ap.col < term_col)
        .into_group_map_by(|ap| ap.row);
        
    (0..term_row).map(|idx| shifted.get(&idx)).map(|row| {
        match row {
            None => (0..term_col).map(|_| ' ').collect::<String>(),
            Some(stars) => {
                let mut chars: Vec<char> = (0..term_col).map(|_| ' ').collect();
                for star in stars {
                    let start = isize::max(0, star.col) as usize;
                    let end = isize::min(term_col - 1, star.col + star.star.chars().count() as isize) as usize;
                    let star_start = if star.col < 0 { -star.col } else { 0 } as usize;
                    let star_len = end - start;
                    let star_slice = &star.star.chars().collect::<Vec<char>>()[star_start..(star_start + star_len)];
                    chars[start..end].copy_from_slice(star_slice);
                }

                chars.iter().collect::<String>()
            }, 
        }
    })   
    .collect::<Vec<String>>()
}

fn is_end(appears: &Vec<StarAppearance>, head_col: isize) -> bool {
    let stars = [STAR1, STAR2];
    let mut max_len = 0;
    for s in stars.map(|s| s.chars().count()) {
        max_len = usize::max(s, max_len);
    }

    appears.iter()
        .map(|a| a.col)
        .map(|c| c + head_col + max_len as isize)
        .all(|c| c < 0)
}

fn main() {
    // let hide_cursor = cursor::HideCursor::from(stdout());
    // print!("{}", cursor::Hide);

    if let Ok((cols, rows)) = termion::terminal_size() {
        let appears = initialize(rows as usize);
        let mut head_col = cols as isize - 1; 
        while !is_end(&appears, head_col) {
            print!("{}{}", clear::All, cursor::Goto(1, 1));

            generate_frame(&appears, cols as isize, rows as isize, head_col)
                .iter()
                .for_each(|line| println!("{}", line)); 
        
            head_col -= 1;

            stdout().flush().unwrap(); 

            sleep(Duration::from_millis(DURATION));
        }
    } else {
        println!("terminal does not respond col and row");
    }
}