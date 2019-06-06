CREATE TABLE todos (
    done BOOLEAN NOT NULL DEFAULT 'f',
    name VARCHAR NOT NULL,
    id uuid PRIMARY KEY
)