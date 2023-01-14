-- Your SQL goes here

CREATE TABLE IF NOT EXISTS deliveries (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    webshop_id INTEGER,
    origin_address TEXT,
    delivery_address TEXT NOT NULL,
    preferred_pickup DATETIME,
    expected_pickup DATETIME,
    preferred_delivery DATETIME,
    expected_delivery DATETIME,
    status TEXT CHECK( status IN ("awaiting pickup", "in warehouse", "in transit", "delivered") ) NOT NULL DEFAULT "awaiting pickup"
)