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


