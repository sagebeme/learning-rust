-- Given: the schema for the course's mini-TWA. Notice how many rules the database enforces itself.

CREATE TABLE stations (
    code TEXT PRIMARY KEY CHECK (code ~ '^[A-Z]{3}$'),
    city TEXT NOT NULL
);

CREATE TABLE employees (
    staff_number INTEGER PRIMARY KEY CHECK (staff_number > 0),
    name         TEXT    NOT NULL CHECK (length(trim(name)) > 0),
    station_code TEXT    NOT NULL REFERENCES stations (code),
    department   TEXT    NOT NULL
);

CREATE TABLE holdings (
    staff_number INTEGER NOT NULL REFERENCES employees (staff_number),
    code         TEXT    NOT NULL,
    completed_on DATE    NOT NULL,
    valid_until  DATE    NOT NULL CHECK (valid_until >= completed_on),
    PRIMARY KEY (staff_number, code)
);

CREATE TABLE transfers (
    id              BIGSERIAL PRIMARY KEY,
    staff_number    INTEGER NOT NULL REFERENCES employees (staff_number),
    from_department TEXT    NOT NULL,
    to_department   TEXT    NOT NULL CHECK (length(trim(to_department)) > 0),
    transferred_on  DATE    NOT NULL
);

INSERT INTO stations (code, city) VALUES
    ('NBO', 'Nairobi'),
    ('MBA', 'Mombasa'),
    ('KIS', 'Kisumu'),
    ('EDL', 'Eldoret');
