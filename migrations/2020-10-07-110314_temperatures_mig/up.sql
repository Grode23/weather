-- Your SQL goes here
CREATE TABLE temperatures (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  minimum FLOAT NOT NULL,
  maximum FLOAT NOT NULL,
  date_of_forecast VARCHAR(20) NOT NULL,
  date_saved VARCHAR(20) NOT NULL,
  api VARCHAR(20) NOT NULL
);

CREATE TABLE accuracies (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  accuracy FLOAT NOT NULL,
  date_of_forecast VARCHAR(20) NOT NULL,
  api VARCHAR(20) NOT NULL
);

CREATE TABLE total (
  api VARCHAR(20) PRIMARY KEY,
  accum_accuracy FLOAT NOT NULL
);
