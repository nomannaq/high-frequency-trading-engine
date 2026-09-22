use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    StopLimit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Filled,
    PartiallyFilled,
    Cancelled,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: Decimal,
    pub filled_quantity: Decimal,
    pub price: Decimal,
    pub stop_price: Option<Decimal>,
    pub status: OrderStatus,
    pub timestamp: DateTime<Utc>,
    pub user_id: Uuid,
    pub updated_at: DateTime<Utc>,

}
impl Order {
    pub fn new(
        symbol: String,
        side: OrderSide,
        order_type: OrderType,
        quantity: Decimal,
        price: Decimal,
        stop_price: Option<Decimal>,
        user_id: Uuid,
    ) -> Self {
        let now= Utc::now();
        Self {
            id: Uuid:: new_v4(),
            symbol,
            side,
            order_type,
            quantity,
            price,
            stop_price,
            status: OrderStatus::Pending,
            timestamp: Default::default(),
            user_id,
            updated_at: Default::default(),
        }
    }
    pub fn is_fully_filled(&self) -> bool {
        self.quantity == Decimal::zero()
    }
    pub fn remaining_quantity(&self) -> Decimal {
        self.quantity.abs()
    }
    pub fn fill(&mut self, quantity: Decimal) {
        self.filled_quantity += quantity;
        self.quantity -= quantity;

        if self.is_fully_filled() {
            self.status = OrderStatus::Filled;
        }
        else {
            self.status = OrderStatus::PartiallyFilled;
        }
    }


}
