struct BankAccount {
    account_holder: String,
    balance: f64,
    account_number: String,
}
fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
impl BankAccount {
    fn new(account_holder: String, balance: f64, account_number: String) -> Self {
        Self {
            account_holder,
            balance,
            account_number,
        }
    }

    fn deposit_money(&mut self, amount: f64) -> f64 {
        if amount < 0.0 {
            println!("amount must be greater than 0");
            return self.balance;
        }
        self.balance += amount;
        println!("account updated,{}", self.balance);
        self.balance
    }
    fn withdrawal(&mut self, amount: f64) -> f64 {
        if amount > self.balance {
            println!("amount more than balance");
            return self.balance;
        } else if amount < 0.0 {
            println!("amount must be greater than 0");
            return self.balance;
        }
        self.balance -= amount;
        println!("amount withdrawn:{}", amount);
        return self.balance;
    }
    fn check_balance(&self) {
        println!(
            "Account holder:{}\nBalance:{}",
            self.account_holder, self.balance
        )
    }
    fn display_information(&self) {
        println!("\n--- Account Information ---");
        println!("Account Holder: {}", self.account_holder);
        println!("Account Number: {}", self.account_number);
        println!("Current Balance: ${:.2}", self.balance);
        println!("---------------------------");
    }
}


fn user_query()
