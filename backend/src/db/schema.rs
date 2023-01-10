// @generated automatically by Diesel CLI.

diesel::table! {
    deliveries (id) {
        id -> Integer,
        origin_address -> Nullable<Text>,
        delivery_address -> Text,
        preferred_pickup -> Nullable<Timestamp>,
        expected_pickup -> Nullable<Timestamp>,
        preferred_delivery -> Nullable<Timestamp>,
        expected_delivery -> Nullable<Timestamp>,
        status -> Text,
    }
}
