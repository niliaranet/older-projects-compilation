CREATE DATABASE IF NOT EXISTS balalaika;
USE balalaika;

DROP TABLE IF EXISTS song;
DROP TABLE IF EXISTS album;
DROP TABLE IF EXISTS artist;

CREATE TABLE artist (
    id int NOT NULL AUTO_INCREMENT,
    name varchar(255),

    PRIMARY KEY (id)
);

CREATE TABLE album (
    id int NOT NULL AUTO_INCREMENT,
    name varchar(255),
    cover varchar(510),
    artist_id int,
    release_date date,

    PRIMARY KEY (id),
    FOREIGN KEY (artist_id) REFERENCES artist(id)
);

CREATE TABLE song (
    id int NOT NULL AUTO_INCREMENT,
    name varchar(255),
    lyrics TEXT,

    album_id int,

    PRIMARY KEY (id),
    FOREIGN KEY (album_id) REFERENCES album(id)
);

CREATE OR REPLACE TABLE user (
    id int auto_increment,
    name varchar(255) unique,
    password varchar(510),
    primary key(id)
);

ALTER TABLE song CONVERT TO CHARACTER SET utf8;
ALTER TABLE album CONVERT TO CHARACTER SET utf8;
ALTER TABLE artist CONVERT TO CHARACTER SET utf8;
ALTER TABLE user CONVERT TO CHARACTER SET utf8;

GRANT ALL PRIVILEGES ON balalaika.* TO 'balalaika_user'@'%' WITH GRANT OPTION;
FLUSH PRIVILEGES;

