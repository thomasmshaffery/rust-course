#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account { id, holder, balance: 0 }
    }
}

fn print_account(account: Account) {
    println!("{:#?}", account)
}

fn main() {
    let mut bank = Bank::new();
    let mut acc = Account::new(666, String::from("Tom"));
    bank.accounts.push(acc);

    for account in bank.accounts {
        print_account(account);
    };
}
