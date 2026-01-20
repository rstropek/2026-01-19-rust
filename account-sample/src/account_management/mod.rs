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
    #[allow(dead_code)]
    account_number: u32,
    balance: f32,
}

/// ## A collection of bank accounts.
///
/// Each account has:
/// * account number (u32) and
/// * a balance (f32).
#[derive(Debug)]
pub struct Accounts {
    accounts: HashMap<u32, Account>,
}

impl Accounts {
    /// Creates a new, empty collection of bank accounts.
    /// 
    /// # Examples
    /// ```
    /// use accounts::account_management::Accounts;
    /// let accounts = Accounts::new();
    /// ```
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    /// Opens a new account with a specified starting balance.
    ///
    /// # Arguments
    ///
    /// * `account_number` - Unique identifier for the account
    /// * `starting_balance` - Initial balance (must be non-negative)
    ///
    /// # Panics
    ///
    /// Panics if:
    /// * The starting balance is negative
    /// * An account with the same account number already exists
    ///
    /// # Examples
    ///
    /// ```
    /// use accounts::account_management::Accounts;
    ///
    /// let mut accounts = Accounts::new();
    /// accounts.open_account(1001, 500.0);
    /// ```
    pub fn open_account(&mut self, account_number: u32, starting_balance: f32) {
        if starting_balance < 0.0 {
            // TODO: Let's add better error handling later.
            panic!("Cannot open account with negative starting balance!");
        }

        if self.accounts.contains_key(&account_number) {
            // TODO: Let's add better error handling later.
            panic!("Account with number {} already exists!", account_number);
        }

        self.accounts.insert(
            account_number,
            Account {
                account_number,
                balance: starting_balance,
            },
        );
    }

    /// Opens a new account with a zero starting balance.
    ///
    /// This is a convenience method that calls [`open_account`](Self::open_account) with a balance of 0.0.
    ///
    /// # Arguments
    ///
    /// * `account_number` - Unique identifier for the account
    ///
    /// # Panics
    ///
    /// Panics if an account with the same account number already exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use accounts::account_management::Accounts;
    ///
    /// let mut accounts = Accounts::new();
    /// accounts.open_empty_account(1001);
    /// ```
    pub fn open_empty_account(&mut self, account_number: u32) {
        self.open_account(account_number, 0.0);
    }

    /// Applies a transaction to the accounts.
    ///
    /// # Arguments
    ///
    /// * `transaction` - The transaction to apply (either CashDeposit or Transfer)
    ///
    /// # Panics
    ///
    /// Panics if:
    /// * The transaction amount is negative
    /// * The target account does not exist (for CashDeposit)
    /// * The source or target account does not exist (for Transfer)
    /// * Attempting to transfer to the same account
    /// * Insufficient funds in the source account (for Transfer)
    ///
    /// # Examples
    ///
    /// ```
    /// use accounts::account_management::{Accounts, transactions::Transaction};
    ///
    /// let mut accounts = Accounts::new();
    /// accounts.open_account(1001, 100.0);
    /// accounts.open_account(1002, 50.0);
    ///
    /// // Cash deposit
    /// accounts.apply_transaction(Transaction::CashDeposit {
    ///     target_account: 1001,
    ///     amount: 200.0,
    /// });
    ///
    /// // Transfer between accounts
    /// accounts.apply_transaction(Transaction::Transfer {
    ///     source_account: 1001,
    ///     target_account: 1002,
    ///     amount: 50.0,
    /// });
    /// ```
    pub fn apply_transaction(&mut self, transaction: transactions::Transaction) {
        match transaction {
            transactions::Transaction::CashDeposit {
                target_account,
                amount,
            } => {
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
                    }
                    None => {
                        panic!("Target account {} does not exist!", target_account);
                    }
                }
            }
            transactions::Transaction::Transfer {
                source_account,
                target_account,
                amount,
            } => {
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

pub mod transactions;

#[cfg(test)]
mod tests {
    use super::*;
    use transactions::Transaction;

    #[test]
    fn test_new_accounts() {
        let accounts = Accounts::new();
        assert_eq!(accounts.accounts.len(), 0);
    }

    #[test]
    fn test_open_account_success() {
        let mut accounts = Accounts::new();
        accounts.open_account(1001, 500.0);
        assert_eq!(accounts.accounts.len(), 1);
        assert_eq!(accounts.accounts.get(&1001).unwrap().balance, 500.0);
    }

    #[test]
    #[should_panic(expected = "Cannot open account with negative starting balance!")]
    fn test_open_account_negative_balance() {
        let mut accounts = Accounts::new();
        accounts.open_account(1001, -100.0);
    }

    #[test]
    #[should_panic(expected = "Account with number 1001 already exists!")]
    fn test_open_account_duplicate() {
        let mut accounts = Accounts::new();
        accounts.open_account(1001, 500.0);
        accounts.open_account(1001, 300.0);
    }

    #[test]
    fn test_open_empty_account() {
        let mut accounts = Accounts::new();
        accounts.open_empty_account(1001);
        assert_eq!(accounts.accounts.len(), 1);
        assert_eq!(accounts.accounts.get(&1001).unwrap().balance, 0.0);
    }

    #[test]
    fn test_cash_deposit_success() {
        let mut accounts = Accounts::new();
        accounts.open_account(1001, 100.0);
        accounts.apply_transaction(Transaction::CashDeposit {
            target_account: 1001,
            amount: 200.0,
        });
        assert_eq!(accounts.accounts.get(&1001).unwrap().balance, 300.0);
    }

    #[test]
    #[should_panic(expected = "Cannot deposit negative amount!")]
    fn test_cash_deposit_negative_amount() {
        let mut accounts = Accounts::new();
        accounts.open_account(1001, 100.0);
        accounts.apply_transaction(Transaction::CashDeposit {
            target_account: 1001,
            amount: -50.0,
        });
    }

    #[test]
    #[should_panic(expected = "Target account 9999 does not exist!")]
    fn test_cash_deposit_nonexistent_account() {
        let mut accounts = Accounts::new();
        accounts.apply_transaction(Transaction::CashDeposit {
            target_account: 9999,
            amount: 100.0,
        });
    }

    #[test]
    fn test_transfer_success() {
        let mut accounts = Accounts::new();
        accounts.open_account(1001, 500.0);
        accounts.open_account(1002, 100.0);
        accounts.apply_transaction(Transaction::Transfer {
            source_account: 1001,
            target_account: 1002,
            amount: 200.0,
        });
        assert_eq!(accounts.accounts.get(&1001).unwrap().balance, 300.0);
        assert_eq!(accounts.accounts.get(&1002).unwrap().balance, 300.0);
    }

    #[test]
    #[should_panic(expected = "Cannot transfer negative amount!")]
    fn test_transfer_negative_amount() {
        let mut accounts = Accounts::new();
        accounts.open_account(1001, 500.0);
        accounts.open_account(1002, 100.0);
        accounts.apply_transaction(Transaction::Transfer {
            source_account: 1001,
            target_account: 1002,
            amount: -50.0,
        });
    }

    #[test]
    #[should_panic(expected = "Cannot transfer to the same account!")]
    fn test_transfer_same_account() {
        let mut accounts = Accounts::new();
        accounts.open_account(1001, 500.0);
        accounts.apply_transaction(Transaction::Transfer {
            source_account: 1001,
            target_account: 1001,
            amount: 100.0,
        });
    }

    #[test]
    #[should_panic(expected = "Source account 9999 does not exist!")]
    fn test_transfer_nonexistent_source() {
        let mut accounts = Accounts::new();
        accounts.open_account(1002, 100.0);
        accounts.apply_transaction(Transaction::Transfer {
            source_account: 9999,
            target_account: 1002,
            amount: 50.0,
        });
    }

    #[test]
    #[should_panic(expected = "Target account 9999 does not exist!")]
    fn test_transfer_nonexistent_target() {
        let mut accounts = Accounts::new();
        accounts.open_account(1001, 500.0);
        accounts.apply_transaction(Transaction::Transfer {
            source_account: 1001,
            target_account: 9999,
            amount: 50.0,
        });
    }

    #[test]
    #[should_panic(expected = "Insufficient funds in account 1001!")]
    fn test_transfer_insufficient_funds() {
        let mut accounts = Accounts::new();
        accounts.open_account(1001, 100.0);
        accounts.open_account(1002, 50.0);
        accounts.apply_transaction(Transaction::Transfer {
            source_account: 1001,
            target_account: 1002,
            amount: 200.0,
        });
    }

    #[test]
    fn test_multiple_transactions() {
        let mut accounts = Accounts::new();
        accounts.open_account(1001, 1000.0);
        accounts.open_account(1002, 500.0);
        accounts.open_account(1003, 0.0);

        // Deposit to account 1001
        accounts.apply_transaction(Transaction::CashDeposit {
            target_account: 1001,
            amount: 500.0,
        });
        assert_eq!(accounts.accounts.get(&1001).unwrap().balance, 1500.0);

        // Transfer from 1001 to 1002
        accounts.apply_transaction(Transaction::Transfer {
            source_account: 1001,
            target_account: 1002,
            amount: 300.0,
        });
        assert_eq!(accounts.accounts.get(&1001).unwrap().balance, 1200.0);
        assert_eq!(accounts.accounts.get(&1002).unwrap().balance, 800.0);

        // Transfer from 1002 to 1003
        accounts.apply_transaction(Transaction::Transfer {
            source_account: 1002,
            target_account: 1003,
            amount: 400.0,
        });
        assert_eq!(accounts.accounts.get(&1002).unwrap().balance, 400.0);
        assert_eq!(accounts.accounts.get(&1003).unwrap().balance, 400.0);

        // Deposit to empty account 1003
        accounts.apply_transaction(Transaction::CashDeposit {
            target_account: 1003,
            amount: 100.0,
        });
        assert_eq!(accounts.accounts.get(&1003).unwrap().balance, 500.0);
    }
}
