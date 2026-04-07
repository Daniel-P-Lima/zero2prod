-- Add migration script here
CREATE TABLE emails_log(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    email_sent timestamptz NOT NULL,
    CONSTRAINT fk_email 
        FOREIGN KEY (email) 
        REFERENCES subscriptions(email) 
        ON DELETE CASCADE
)