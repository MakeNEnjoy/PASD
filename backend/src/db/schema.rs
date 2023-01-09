// @generated automatically by Diesel CLI.

diesel::table! {
    deliveries (id) {
        id -> Integer,
        origin_address -> Text,
        delivery_address -> Text,
        preferred_pickup -> Text,
        expected_pickup -> Text,
        preferred_delivery -> Text,
        expected_delivery -> Text,
        status -> Text,
    }
}
