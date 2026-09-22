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
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            symbol,
            side,
            order_type,
            quantity,
            filled_quantity: Decimal::ZERO,
            price,
            stop_price,
            status: OrderStatus::Pending,
            timestamp: now,
            user_id,
            updated_at: now,
        }
    }

    pub fn is_fully_filled(&self) -> bool {
        self.filled_quantity >= self.quantity
    }

    pub fn remaining_quantity(&self) -> Decimal {
        self.quantity - self.filled_quantity
    }

    pub fn fill(&mut self, quantity: Decimal) {
        if quantity <= Decimal::ZERO {
            return;
        }

        let remaining = self.remaining_quantity();
        let actual_fill = if quantity > remaining {
            remaining
        } else {
            quantity
        };

        self.filled_quantity += actual_fill;
        self.updated_at = Utc::now();

        if self.is_fully_filled() {
            self.status = OrderStatus::Filled;
        } else {
            self.status = OrderStatus::PartiallyFilled;
        }
    }

    pub fn cancel(&mut self) {
        self.status = OrderStatus::Cancelled;
        self.updated_at = Utc::now();
    }

    pub fn reject(&mut self) {
        self.status = OrderStatus::Rejected;
        self.updated_at = Utc::now();
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.quantity <= Decimal::ZERO {
            return Err("Quantity must be positive".to_string());
        }

        match self.order_type {
            OrderType::Limit | OrderType::StopLimit => {
                if self.price <= Decimal::ZERO {
                    return Err("Limit orders must have a positive price".to_string());
                }
            }
            _ => {}
        }

        match self.order_type {
            OrderType::StopLoss | OrderType::StopLimit => {
                if self.stop_price.is_none() || self.stop_price.unwrap() <= Decimal::ZERO {
                    return Err("Stop orders must have a positive stop price".to_string());
                }
            }
            _ => {}
        }

        Ok(())
    }
}