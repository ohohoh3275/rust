struct Item {
    name:String,
    price:i64,
    transaction:RefCell<Weak<Transaction>>
}

struct Transaction {
    item: RefCell<Vec<Item>>,
    amount: i64
}


fn main() {

}