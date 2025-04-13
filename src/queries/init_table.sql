create table if not exists "orders" (
    id bigserial primary key,
    user_id integer not null,
    products_ids integer[] not null,
    is_paid boolean not null default false
);
