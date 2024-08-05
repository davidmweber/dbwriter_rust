
CREATE TABLE samples (
                         id BIGINT PRIMARY KEY,
                         name VARCHAR NOT NULL,
                         timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
                         v0 REAL,
                         v1 REAL );
