mod errors;
mod iterators;
mod lifetimes;

use std::io::Error;

use errors::read_logs::{read_log, write_to_file};
use iterators::iterator_fn::{print_elements, shorten_string, uppercase};

const LOG_FILE: &str = "logs.txt";

fn main() -> Result<(), Error> {
    let mut log = read_log(LOG_FILE)?;

    let upper = uppercase(&log);
    print_elements(&upper);
    shorten_string(&mut log, 1);

    write_to_file("errors.txt", &log)?;

    Ok(())
}

// mod bank;
// mod game;
// mod media;

// use bank::bank::{Account, Bank};
// use game::deck::Deck;
// use media::media_container::add_media;

// fn print_account(bank: &Bank) {
//     println!("{bank:#?}");
// }

// fn main() {
//     let mut deck = Deck::new();
//     deck.shuffle();
//     let hand = deck.deal(5);
//     println!("{}", deck.cards.len());
//     println!("{hand:#?}");

//     let mut account = Account::new(1, String::from("Alice"));

//     account.deposit(100);
//     account.withdraw(50);

//     let mut bank = Bank::new();

//     bank.add_account(account);

//     print_account(&bank);

//     add_media();
// }
