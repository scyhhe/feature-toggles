// @generated automatically by Diesel CLI.

diesel::table! {
    features (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        state -> Bool,
    }
}
