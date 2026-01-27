pub struct User {
    pub id: u32,
    pub balances: Vec<Balance>
}

#[derive(Debug)]
pub struct Balance {
    pub asset: (String, i32)
}

#[derive(Debug)]
pub struct Order {
    pub user_id: u32,
    pub price: i32,
    pub quantity: u32,
    pub order_type: OrderType
}

#[derive(Debug)]
pub enum OrderType {
    BID,
    ASK
}

impl User {
    pub fn new(id: u32, balances: Vec<Balance>) -> User {
        User {
            id,
            balances: balances
        }
    }
}

impl Order {
    pub fn new(user_id: u32, price: i32, quantity: u32, order_type: OrderType) -> Order {
        Order {
            user_id,
            price,
            quantity,
            order_type
        }
    }
}
