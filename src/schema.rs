table! {
    accuracies (id) {
        id -> Integer,
        accuracy -> Float,
        api -> Varchar,
    }
}

table! {
    temperatures (id) {
        id -> Integer,
        minimum -> Float,
        maximum -> Float,
        date_of_forecast -> Varchar,
        date_saved -> Varchar,
        api -> Varchar,
    }
}

table! {
    total (api) {
        api -> Varchar,
        accum_accuracy -> Float,
    }
}

allow_tables_to_appear_in_same_query!(
    accuracies,
    temperatures,
    total,
);
