#[derive(Debug)]
pub struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

#[derive(Debug)]
pub struct Bank {
    pub accounts: Vec<Account>,
}

impl Account {
    pub fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    pub fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: i32) {
        self.balance -= amount;
    }
}

impl Bank {
    pub fn new() -> Self {
        Bank { accounts: vec![] }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    pub fn get_all_balances(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }
}
