CREATE TABLE games (
	id SERIAL PRIMARY KEY,
	player1_id INTEGER REFERENCES users(id) NOT NULL,
	player2_id INTEGER REFERENCES users(id),
	player1_score SMALLINT NOT NULL,
	player2_score SMALLINT NOT NULL,
	player1_extra SMALLINT[] NOT NULL,
	player2_extra SMALLINT[] NOT NULL
)
