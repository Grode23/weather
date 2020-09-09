-- Your SQL goes here
CREATE TABLE temperatures (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  minimum FLOAT NOT NULL,
  maximum FLOAT NOT NULL,
  date_of_temp VARCHAR(20) NOT NULL,
  date_today VARCHAR(20) NOT NULL
)
