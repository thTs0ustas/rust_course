mod errors;
use errors::read_logs::{read_log, write_to_file};

const LOG_FILE: &str = "logs.txt";

fn main() {
    let log = match read_log(LOG_FILE) {
        Ok(log) => log,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            return ();
        }
    };

    match write_to_file("errors.txt", &log) {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {:?}", err),
    }
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
