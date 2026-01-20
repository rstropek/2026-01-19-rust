use crate::account_management::{Accounts, transactions::Transaction};

mod account_management;

fn main() {
    let mut our_bank_accounts = Accounts::new();

    our_bank_accounts.open_empty_account(1001);
    our_bank_accounts.open_account(1002, 500.0);

    println!("Accounts: {:?}", our_bank_accounts);

    let deposit = Transaction::CashDeposit {
        target_account: 1001,
        amount: 150.0,
    };
    our_bank_accounts.apply_transaction(deposit);
    println!("Accounts: {:?}", our_bank_accounts);
}
