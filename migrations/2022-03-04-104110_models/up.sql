CREATE TABLE decks (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);
CREATE UNIQUE INDEX name_idx ON decks (name);

CREATE TABLE cards (
  id SERIAL PRIMARY KEY,
  card_type VARCHAR NOT NULL,
  suit VARCHAR NOT NULL,
  number integer NOT NULL
);

CREATE TABLE card_decks (
  card_id int REFERENCES cards (id) ON UPDATE CASCADE ON DELETE CASCADE,
  deck_id int REFERENCES decks (id) ON UPDATE CASCADE ON DELETE CASCADE,
  "order" INTEGER NOT NULL,
  CONSTRAINT card_decks_pkey PRIMARY KEY (card_id, deck_id)
);