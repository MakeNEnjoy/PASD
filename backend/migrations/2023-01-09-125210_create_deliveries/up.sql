-- Your SQL goes here


CREATE TABLE IF NOT EXISTS deliveries (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    origin_address TEXT NOT NULL DEFAULT "",
    delivery_address TEXT NOT NULL,
    preferred_pickup TEXT NOT NULL DEFAULT "",
    expected_pickup TEXT NOT NULL DEFAULT "",
    preferred_delivery TEXT NOT NULL DEFAULT "",
    expected_delivery TEXT NOT NULL DEFAULT "",
    status TEXT CHECK( status IN ("awaiting pickup", "in warehouse", "in transit", "delivered") ) NOT NULL DEFAULT "awaiting pickup"
)