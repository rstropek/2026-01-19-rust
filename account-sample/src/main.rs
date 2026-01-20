/*
We maintain bank accounts.

A struct should maintain a list of bank accounts. Each account has an account number (u32)
and a balance (f32). The account number uniquely identifies an account.

We have transactions (enum):
- Cash deposit (target account, amount)
- Transfer (source account, target account, amount) between accounts of our own bank

The struct for the bank accounts should have a method to apply a transaction to the accounts.
We can add helper methods (e.g. constructors, etc.).
 */

use core::panic;
use std::collections::HashMap;

#[derive(Debug)]
struct Account {
    account_number: u32,
    balance: f32
}

/// ## A collection of bank accounts.
/// 
/// Each account has:
/// * account number (u32) and 
/// * a balance (f32).
#[derive(Debug)]
struct Accounts {
    accounts: HashMap<u32, Account>
}

impl Accounts {
    /// Creates a new, empty collection of bank accounts.
    fn new() -> Self {
        Self {
            accounts: HashMap::new()
        }
    }

    fn open_account(&mut self, account_number: u32, starting_balance: f32) {
        if starting_balance < 0.0 {
            // TODO: Let's add better error handling later.
            panic!("Cannot open account with negative starting balance!");
        }

        if self.accounts.contains_key(&account_number) {
            // TODO: Let's add better error handling later.
            panic!("Account with number {} already exists!", account_number);
        }

        self.accounts.insert(account_number, Account {
            account_number,
            balance: starting_balance
        });
    }

    fn open_empty_account(&mut self, account_number: u32) {
        self.open_account(account_number, 0.0);
    }

    fn apply_transaction(&mut self, transaction: Transaction) {
        match transaction {
            Transaction::CashDeposit { target_account, amount } => {
                if amount < 0.0 {
                    panic!("Cannot deposit negative amount!");
                }

                /*
                if let Some(account) = self.accounts.get_mut(&target_account) {
                    account.balance += amount;
                } else {
                    panic!("Target account {} does not exist!", target_account);
                }
                */
                match self.accounts.get_mut(&target_account) {
                    Some(account) => {
                        account.balance += amount;
                    },
                    None => {
                        panic!("Target account {} does not exist!", target_account);
                    }
                }
            },
            Transaction::Transfer { source_account, target_account, amount } => {
                if amount < 0.0 {
                    panic!("Cannot transfer negative amount!");
                }

                if source_account == target_account {
                    panic!("Cannot transfer to the same account!");
                }

                // Check if both accounts exist
                if !self.accounts.contains_key(&source_account) {
                    panic!("Source account {} does not exist!", source_account);
                }
                if !self.accounts.contains_key(&target_account) {
                    panic!("Target account {} does not exist!", target_account);
                }

                // Check sufficient funds
                let source_balance = self.accounts.get(&source_account).unwrap().balance;
                if source_balance < amount {
                    panic!("Insufficient funds in account {}!", source_account);
                }

                // Perform the transfer
                self.accounts.get_mut(&source_account).unwrap().balance -= amount;
                self.accounts.get_mut(&target_account).unwrap().balance += amount;
            }
        }
    }
}

enum Transaction {
    CashDeposit { target_account: u32, amount: f32 },
    Transfer { source_account: u32, target_account: u32, amount: f32 },
}

fn main() {
    let mut our_bank_accounts = Accounts::new();

    our_bank_accounts.open_empty_account(1001);
    our_bank_accounts.open_account(1002, 500.0);

    println!("Accounts: {:?}", our_bank_accounts);

    let deposit = Transaction::CashDeposit { target_account: 1001, amount: 150.0 };
    our_bank_accounts.apply_transaction(deposit);
    println!("Accounts: {:?}", our_bank_accounts);
}
