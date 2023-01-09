-- Your SQL goes here


CREATE TABLE IF NOT EXISTS deliveries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    origin_address TEXT,
    delivery_address TEXT NOT NULL,
    preferred_pickup TEXT,
    expected_pickup TEXT,
    preferred_delivery TEXT,
    expected_delivery TEXT,
    status TEXT CHECK( status IN ("awaiting pickup", "in warehouse", "in transit", "delivered") ) NOT NULL DEFAULT "awaiting pickup"
)