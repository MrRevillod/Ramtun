CREATE TABLE question_banks (
	id UUID PRIMARY KEY,
	course_id UUID NOT NULL REFERENCES courses(id),
	name TEXT NOT NULL,
	created_at TIMESTAMPTZ NOT NULL,
	deleted_at TIMESTAMPTZ
);

CREATE TABLE questions (
	id UUID PRIMARY KEY,
	bank_id UUID NOT NULL REFERENCES question_banks(id),
	position SMALLINT NOT NULL,
	prompt TEXT NOT NULL,
	options TEXT[] NOT NULL,
	answer_index SMALLINT NOT NULL,
	created_at TIMESTAMPTZ NOT NULL,
	deleted_at TIMESTAMPTZ
);

CREATE INDEX idx_question_banks_course_id ON question_banks(course_id);
CREATE INDEX idx_question_banks_created_at ON question_banks(created_at);
CREATE INDEX idx_question_banks_deleted_at ON question_banks(deleted_at);

CREATE INDEX idx_questions_bank_position ON questions(bank_id, position);
CREATE INDEX idx_questions_deleted_at ON questions(deleted_at);
