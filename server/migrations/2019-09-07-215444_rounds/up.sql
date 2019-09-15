CREATE TABLE rounds (
	id SERIAL PRIMARY KEY,
	game_id SERIAL REFERENCES games(id) NOT NULL,
	round_count SMALLINT NOT NULL,
	player1_throws SMALLINT[3] NOT NULL,
	player2_throws SMALLINT[3] NOT NULL
)
