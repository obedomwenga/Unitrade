use std::time::{SystemTime, UNIX_EPOCH};    //Displaying time in UNIX timestamp
use candid::{CandidType, Principal};          //serialization, deserialization for interacting with ICP
use serde::{Serialize, Deserialize};         //storing data externally in JSON format

type Timestamp = u64;
type Cycles = u64;
const TRANSACTION_FEE: Cycles = 100;   //transaction fee for each tx is hardcoded for now

//Structure of user's account
#[derive(Serialize, Deserialize, CandidType, Debug)]
struct Account {

    owner: Principal,
    balance: Cycles,        //will rep cycles, the smallest unit
    transaction_history: Vec<Transaction>,
    
}

//Structure of a transaction
#[derive(Serialize, Deserialize, CandidType, Debug)]
struct Transaction {

    transaction_id: String,
    from_account: Principal,
    to_account: Principal,
    amount: Cycles,     //cycles
    timestamp: Timestamp,  //UNIX timestamp
    transaction_type: TransactionType,
    transaction_fee: Cycles,
    transaction_status: TransactionStatus,

}

//For handling which transaction user is handling
enum TransactionType {

    Deposit,
    Withdraw,
    Transfer,

}

//For users to get notifications
enum NotificationType {


    SMS,
    Email,
    InApp,

}

//For handling all possible errors

enum ErrorType {

    InsufficientBalance, //user has insufficient balance for transaction
    AddressDoesNotExist, //using an incorrect address
    TransactionFailed,  //output an error when transaction fails
    InvalidAmount,      //transaction amount is zero or negative
    CurrencyMismatch,   //transferring diff currency types without conversion
    RateNotAvailable(Currency, Currency),   //when converting currencies with a rate not available
    
}

//For handling status of the transactions
enum TransactionStatus {

    Pending,
    Confirmed,
    Failed,

}

//The currencies currently available
enum Currency {

    KSH,  //Kenya
    TSH,  //Tanzania
    USH,  //Uganda
    GSH,  //Ghana
    ICP,  //ICP token


}



//Structure of the notification - For now it will be InApp
struct Notification {
    notification_type: NotificationType,
    message: String,
    timestamp: Timestamp,
}

//For handling conversion of currencies, eg Kshs to ICP
struct CurrencyExchange {
    base_currency: Currency,      // e.g., "Ksh"
    target_currency: Currency,    // e.g., "ICP"
    exchange_rate: f64,         // e.g., 1 Ksh = 0.005 ICP
    last_updated: Timestamp,
}


//Implement logic for User's account
impl Account {

    //Initialize a new `Account` instance
    pub fn new(owner: Principal) -> Self {

        Account {

            owner,
            balance: 0,  //new account starts at zero
            transaction_history: Vec::new(),  //initialize new empty vector to store user's tx history
        }

    }


    //Check user's balance
    pub fn check_balance(&self) -> Cycles {

        self.balance
    }


    //Adjust user's balance when a tx occurs
    pub fn adjust_balance(&mut self, amount: Cycles) -> Result<Cycles, ErrorType> {

        if self.balance + amount < 0 {

            Err(ErrorType::InsufficientBalance)

        } else {

            self.balance += amount;
            Ok(self.balance)

        }

    }

    //Create a new tx and add to user's tx history
    pub fn create_transaction(&mut self, from_account: Principal, to_account: Principal, amount: Cycles, transaction_type: TransactionType) -> Result<(), ErrorType> {

        //use transaction's timestamp as transaction_id (will create hashing later)
        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!")
            .as_millis() as Timestamp;

        let transaction = Transaction {

            transaction_id: current_timestamp.to_string(),
            from_account,
            to_account,
            amount,
            timestamp: current_timestamp,
            transaction_type,
            transaction_fee: TRANSACTION_FEE,
            transaction_status: TransactionStatus::Pending    //default status is pending

        };

        //Check if a transaction can be completed
        match transaction_type {

            TransactionType::Deposit => {

                self.adjust_balance(amount)?;            

            },

            TransactionType::Withdraw => {

                self.adjust_balance(-amount)?;

            },

            TransactionType::Transfer => {

                self.adjust_balance(-amount)?;
            },

        }

        //if all checks passed and balance was adjusted, add tx to history

        self.add_transaction(transaction);
        Ok(())

    }


    //Add transaction to transaction history
    pub fn add_transaction(&mut self, transaction: Transaction) {

        self.transaction_history.push(transaction);
    } 


    //If a user wants to retrieve their transaction history
    pub fn get_transactions(&self) {

        self.transaction_history.clone()

    }

}


fn main() {

    println!(r#"

    
                           WELCOME TO



                    
          _       __________________ _______  _______  ______   _______             _______  _        _        _______ _________
|\     /|( (    /|\__   __/\__   __/(  ____ )(  ___  )(  __  \ (  ____ \  |\     /|(  ___  )( \      ( \      (  ____ \\__   __/
| )   ( ||  \  ( |   ) (      ) (   | (    )|| (   ) || (  \  )| (    \/  | )   ( || (   ) || (      | (      | (    \/   ) (   
| |   | ||   \ | |   | |      | |   | (____)|| (___) || |   ) || (__      | | _ | || (___) || |      | |      | (__       | |   
| |   | || (\ \) |   | |      | |   |     __)|  ___  || |   | ||  __)     | |( )| ||  ___  || |      | |      |  __)      | |   
| |   | || | \   |   | |      | |   | (\ (   | (   ) || |   ) || (        | || || || (   ) || |      | |      | (         | |   
| (___) || )  \  |___) (___   | |   | ) \ \__| )   ( || (__/  )| (____/\  | () () || )   ( || (____/\| (____/\| (____/\   | |   
(_______)|/    )_)\_______/   )_(   |/   \__/|/     \|(______/ (_______/  (_______)|/     \|(_______/(_______/(_______/   )_(   
                                                                                                                                





                           THE NUMBER ONE CROSS-BORDER PAYMENT PLATFORM FOR AFRICAN MERCHANTS

                           BUILT WITH LOVE IN ICP


    "#);

    // Create a new account for a dummy principal.
    let principal = Principal::from_text("abcdef123456").unwrap();
    let mut account = Account::new(principal.clone());

    // Display initial balance.
    println!("Initial balance: {}", account.check_balance());

    // Deposit some cycles.
    account.create_transaction(
        principal.clone(),
        principal.clone(),
        5000,
        TransactionType::Deposit,
    ).unwrap();
    println!("After deposit: {}", account.check_balance());

    // Withdraw some cycles.
    account.create_transaction(
        principal.clone(),
        principal.clone(),
        1000,
        TransactionType::Withdraw,
    ).unwrap();
    println!("After withdrawal: {}", account.check_balance());

    // Display transaction history.
    for transaction in account.get_transactions() {
        println!("{:?}", transaction);
    }
}

