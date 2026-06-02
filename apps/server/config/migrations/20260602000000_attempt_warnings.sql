CREATE TYPE warning_type AS ENUM (
	'focus_loss',
	'clipboard',
	'screenshot',
	'navigation',
	'devtools'
);

CREATE TABLE attempt_warnings (
	id UUID PRIMARY KEY,
	attempt_id UUID NOT NULL REFERENCES attempts(id),
	warning_type warning_type NOT NULL,
	details VARCHAR(255) NOT NULL,
	sequence_number SMALLINT NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_attempt_warnings_attempt_id ON attempt_warnings(attempt_id);
CREATE INDEX idx_attempt_warnings_created_at ON attempt_warnings(created_at);
