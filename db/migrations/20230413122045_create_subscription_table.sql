-- migrate:up
CREATE TABLE subscriptions(
id uuid not null primary key,
email text not null UNIQUE,
name text not null,
subscribed_at timestamptz not null
);

-- migrate:down
--DROP table subscriptionsddd;
