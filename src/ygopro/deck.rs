//! Deck reader and struct

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Default)]
pub struct Deck {
    pub main: Vec<i32>,
    pub extra: Vec<i32>,
    pub side: Vec<i32>,
}

impl Deck {
    pub fn from_path(p: impl AsRef<Path>) -> anyhow::Result<Self> {
        let mut deck = Deck::default();

        let f = File::open(p)?;
        let mut reader = BufReader::new(f);

        let mut line = String::new();
        let mut flag = -1;

        while reader.read_line(&mut line)? > 0 {
            match line.as_str() {
                "#main" => {
                    flag = 1;
                }
                "#extra" => {
                    flag = 2;
                }
                "!side" => {
                    flag = 3;
                }
                _ => {
                    if let Ok(code) = line.parse::<i32>() {
                        if code > 100 {
                            match flag {
                                1 => deck.main.push(code),
                                2 => deck.extra.push(code),
                                3 => deck.side.push(code),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        Ok(deck)
    }
}
