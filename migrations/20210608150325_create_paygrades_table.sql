-- Add migration script here
CREATE TYPE grade_scale AS ENUM ('enlisted', 'warrant', 'commissioned');

CREATE TABLE IF NOT EXISTS paygrades (
    id SERIAL NOT NULL,
    grade VARCHAR NOT NULL,
    scale grade_scale NOT NULL,
    PRIMARY KEY (id)
);

INSERT INTO
    paygrades (grade, scale)
VALUES
    ('E-0', 'enlisted'),
    ('E-1', 'enlisted'),
    ('E-2', 'enlisted'),
    ('E-3', 'enlisted'),
    ('E-4', 'enlisted'),
    ('E-5', 'enlisted'),
    ('E-6', 'enlisted'),
    ('E-7', 'enlisted'),
    ('E-8', 'enlisted'),
    ('E-9', 'enlisted'),
    ('W-1', 'warrant'),
    ('W-2', 'warrant'),
    ('W-3', 'warrant'),
    ('W-4', 'warrant'),
    ('W-5', 'warrant'),
    ('O-1', 'commissioned'),
    ('O-2', 'commissioned'),
    ('O-3', 'commissioned'),
    ('O-4', 'commissioned'),
    ('O-5', 'commissioned'),
    ('O-6', 'commissioned'),
    ('O-7', 'commissioned'),
    ('O-8', 'commissioned'),
    ('O-9', 'commissioned'),
    ('O-10', 'commissioned');
