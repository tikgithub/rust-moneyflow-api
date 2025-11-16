-- Add migration script here
CREATE TABLE transaction (
    id SERIAL4 not null,
    user_id integer not null,
    category_id integer not null,
    type varchar(255) not null,
    amount DECIMAL(12, 2) DEFAULT 0.00,
    memo text,
    description text,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP ,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (category_id) REFERENCES categories(id),
    PRIMARY KEY (id)
);