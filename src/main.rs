#[allow(dead_code)]
#[derive(Debug)]
struct BankAccount {
    account_holder: String,
    balance: f64,
    account_number: String,
}
fn main() {
    // loop{
        // println!("\nWhat would you like to do?");
        // println!("1. Check Balance");
        // println!("2. Make Deposit");
        // println!("3. Make Withdrawal");
        // println!("4. View Account Info");
        // println!("5. Exit");
        // println!("Enter your choice (1-5):");
        // let user_entry=user_query().expect("cannot take user entry");
        // // let user_entry:i32=user_entry.trim().parse().unwrap_or_default();
        // match user_entry{
        //     x=> x.check.balance(),

        // }
    // }
    let mut person_1 = BankAccount::new(
        "benjamin".to_string(),
        1_000_000.0,
        "234-356-999".to_string(),
    );
    person_1.display_information();
    person_1.check_balance();

    // let user_query=user_query("enter amount");
    let user_amount = atof64("enter amount").expect("cannot convert to float64 from string");
    let withdrawn_amount = person_1.withdrawal(user_amount);
    println!("amount remaining:{}", withdrawn_amount);

    println!("Enter name");
    let account_holder = user_query().unwrap_or_default();
    let balance =
        atof64("enter balance amount:").expect("cannot convert user entry into balance f64");
    println!("enter account number::");
    let user_account_number = user_query().unwrap_or_default();
    let account_number = user_account_number;
    let person_2 = BankAccount::new(account_holder, balance, account_number);
    println!("person 2:{:?}", person_2);

    println!()
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
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

fn user_query() -> Result<String, String> {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("cannot collect user input");
    if user_input.trim().is_empty() {
        return Err("user input cannot be empty".to_string());
    }

    Ok(user_input)
}

fn atof64(prompt: &str) -> Result<f64, String> {
    println!("{}", prompt);
    let buffer_entry = user_query()?;
    let buffer_entry: f64 = buffer_entry
        .trim()
        .parse()
        .expect("cannot convert string to f64");
    Ok(buffer_entry)
}
