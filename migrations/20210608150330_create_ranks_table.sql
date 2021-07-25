-- Add migration script here
CREATE TYPE rank_modifier AS ENUM ('leadership', 'technical', 'clan');
CREATE TYPE branch AS ENUM ('army', 'navy', 'marine corps', 'air force');

CREATE TABLE IF NOT EXISTS ranks (
    id SERIAL NOT NULL,
    grade_id INT NOT NULL,
    modifier rank_modifier DEFAULT NULL,
    name VARCHAR NOT NULL,
    abbreviation VARCHAR NOT NULL,
    image_uri VARCHAR NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT fk_grade_id FOREIGN KEY (grade_id) REFERENCES paygrades(id)
);

INSERT INTO
    ranks (grade_id, modifier, name, abbreviation, image_uri)
VALUES
    (1, DEFAULT, 'Recruit', 'RT', '/images/e0-rct.png'),
    (2, DEFAULT, 'Private', 'PVT', '/images/e1-pvt.png'),
    (3, DEFAULT, 'Private Second Class', 'PV2', '/images/e2-pv2.png'),
    (4, DEFAULT, 'Private First Class', 'PFC', '/images/e3-pfc.png'),
    (5, DEFAULT, 'Specialist', 'SPC', '/images/e4-spc.png'),
    (5, 'leadership', 'Corporal', 'CPL', '/images/e4-cpl.png'),
    (6, 'leadership', 'Sergeant', 'SGT', '/images/e5-sgt.png'),
    (7, 'leadership', 'Staff Sergeant', 'SSG', '/images/e6-ssg.png'),
    (8, 'leadership', 'Sergeant First Class', 'SFC', '/images/e7-sfc.png'),
    (9, 'technical', 'Master Sergeant', 'MSG', '/images/e8-msg.png'),
    (9, 'leadership', 'First Sergeant', '1SG', '/images/e8-1sg.png'),
    (10, 'technical', 'Sergeant Major', 'SGM', '/images/e9-sgm.png'),
    (10, 'leadership', 'Command Sergeant Major', 'CSM', '/images/e9-csm.png'),
    (10, 'clan', 'Sergeant Major of the Army', 'SMA', '/images/e9-sma.png'),
    (11, DEFAULT, 'Warrant Officer 1', 'WO1', '/images/w1-wo1.png'),
    (12, DEFAULT, 'Chief Warrant Officer 2', 'CW2', '/images/w2-cw2.png'),
    (13, DEFAULT, 'Chief Warrant Officer 3', 'CW3', '/images/w3-cw3.png'),
    (14, DEFAULT, 'Chief Warrant Officer 4', 'CW4', '/images/w4-cw4.png'),
    (15, DEFAULT, 'Chief Warrant Officer 5', 'CW5', '/images/w5-cw5.png'),
    (16, DEFAULT, '2nd Lieutenant', '2LT', '/images/o1-2lt.png'),
    (17, DEFAULT, '1st Lieutenant', '1LT', '/images/o2-1lt.png'),
    (18, DEFAULT, 'Captain', 'CPT', '/images/o3-cpt.png'),
    (19, DEFAULT, 'Major', 'MAJ', '/images/o4-maj.png'),
    (20, DEFAULT, 'Lieutenant Colonel', 'LTC', '/images/o5-ltc.png'),
    (21, DEFAULT, 'Colonel', 'COL', '/images/o6-col.png'),
    (22, DEFAULT, 'Brigadier General', 'BG', '/images/o7-bg.png'),
    (23, DEFAULT, 'Major General', 'MG', '/images/o8-mg.png'),
    (24, DEFAULT, 'Lieutenant General', 'LTG', '/images/o9-ltg.png'),
    (25, DEFAULT, 'General', 'GEN', '/images/o10-gen.png');
