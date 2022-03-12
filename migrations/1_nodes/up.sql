CREATE TABLE nodes (
	device_id BIGINT NOT NULL UNIQUE PRIMARY KEY,
	hits INT NOT NULL,
	last_hit_at_epoch BIGINT NOT NULL,
	connected BOOLEAN NOT NULL
);
