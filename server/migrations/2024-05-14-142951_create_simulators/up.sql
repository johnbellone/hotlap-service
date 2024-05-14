CREATE EXTENSION citext;
CREATE TABLE IF NOT EXISTS simulators(
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    name CITEXT NOT NULL,
    version VARCHAR NOT NULL,
    description TEXT,
    create_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY(name, version)
);
