select distinct unnest(products_ids) from "orders" where "user_id" = $1;
