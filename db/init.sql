CREATE TABLE datasets (
    id          SERIAL PRIMARY KEY,
    name        TEXT NOT NULL,
    description TEXT
);

CREATE TABLE locations (
    id   SERIAL PRIMARY KEY,
    name TEXT,
    lat  DOUBLE PRECISION NOT NULL,
    lon  DOUBLE PRECISION NOT NULL
);

CREATE TABLE records (
    id          SERIAL PRIMARY KEY,
    dataset_id  INTEGER NOT NULL REFERENCES datasets(id),
    location_id INTEGER REFERENCES locations(id),
    timestamp   TIMESTAMPTZ NOT NULL DEFAULT now(),
    data        JSONB
);
