CREATE TABLE Users (
    username text NOT NULL,
    password text NOT NULL,
    id uuid NOT NULL DEFAULT uuid_generate_v1(),
    CONSTRAINT user_pkey PRIMARY KEY (id)
);