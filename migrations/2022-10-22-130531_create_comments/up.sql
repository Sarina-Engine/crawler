CREATE TABLE comments(
	vid INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	id INTEGER NOT NULL,
	product_id INTEGER NOT NULL,
	title TEXT NOT NULL,
	body TEXT NOT NULL,
	rating INTEGER NOT NULL
);

CREATE TABLE products(
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	title_fa TEXT NOT NULL
);