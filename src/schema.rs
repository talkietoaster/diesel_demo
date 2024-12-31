// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct CustomerOrderStatusEnum;
}

diesel::table! {
    customer (id) {
        id -> Integer,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 20]
        phone_number -> Nullable<Varchar>,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        #[max_length = 100]
        city -> Nullable<Varchar>,
        #[max_length = 100]
        state -> Nullable<Varchar>,
        #[max_length = 100]
        country -> Nullable<Varchar>,
        #[max_length = 20]
        postal_code -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 50]
        created_by -> Nullable<Varchar>,
        #[max_length = 50]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CustomerOrderStatusEnum;

    customer_order (id) {
        id -> Integer,
        customer_id -> Integer,
        instrument_id -> Integer,
        order_date -> Date,
        quantity -> Integer,
        total_amount -> Decimal,
        #[max_length = 9]
        status -> CustomerOrderStatusEnum,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 50]
        created_by -> Nullable<Varchar>,
        #[max_length = 50]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    instrument (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        make -> Nullable<Varchar>,
        #[max_length = 255]
        model -> Nullable<Varchar>,
        #[sql_name = "type"]
        #[max_length = 125]
        type_ -> Nullable<Varchar>,
        #[max_length = 255]
        country_of_manufacture -> Nullable<Varchar>,
        #[max_length = 1024]
        serial_number -> Nullable<Varchar>,
        #[max_length = 255]
        sku -> Nullable<Varchar>,
        new -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 50]
        created_by -> Nullable<Varchar>,
        #[max_length = 50]
        updated_by -> Nullable<Varchar>,
        model_id -> Nullable<Integer>,
        #[max_length = 255]
        line -> Nullable<Varchar>,
        #[max_length = 255]
        picture -> Nullable<Varchar>,
    }
}

diesel::table! {
    make (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 50]
        created_by -> Nullable<Varchar>,
        #[max_length = 50]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    model (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        make_id -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 50]
        created_by -> Nullable<Varchar>,
        #[max_length = 50]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::joinable!(customer_order -> customer (customer_id));
diesel::joinable!(customer_order -> instrument (instrument_id));
diesel::joinable!(instrument -> model (model_id));
diesel::joinable!(model -> make (make_id));

diesel::allow_tables_to_appear_in_same_query!(
    customer,
    customer_order,
    instrument,
    make,
    model,
    posts,
);
