// @generated automatically by Diesel CLI.

diesel::table! {
    deliveries (id) {
        id -> Nullable<Integer>,
        origin_address -> Nullable<Text>,
        delivery_address -> Text,
        preferred_pickup -> Nullable<Text>,
        expected_pickup -> Nullable<Text>,
        preferred_delivery -> Nullable<Text>,
        expected_delivery -> Nullable<Text>,
        status -> Text,
    }
}
