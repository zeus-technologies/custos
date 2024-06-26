// @generated automatically by Diesel CLI.

diesel::table! {
    files (filepath) {
        filepath -> Text,
        hash -> Text,
    }
}
