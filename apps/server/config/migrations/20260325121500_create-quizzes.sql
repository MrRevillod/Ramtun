CREATE TYPE quiz_kind AS ENUM ('Traditional', 'Certainly');

CREATE TYPE certainly_level AS (
    correct SMALLINT,
    incorrect SMALLINT
);

CREATE TYPE certainly_table AS (
    low certainly_level,
    medium certainly_level,
    high certainly_level
);

CREATE TYPE question AS (
    id UUID,
    question TEXT,
    options TEXT[],
    answer SMALLINT,
    images TEXT[]
);

CREATE TYPE attempt_status AS ENUM ('in_progress', 'submitted');

CREATE TYPE attempt_certainty_level AS ENUM ('low', 'medium', 'high');

CREATE TABLE quizzes (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES users(id),
    title TEXT NOT NULL,
    kind quiz_kind NOT NULL,
    join_code TEXT UNIQUE NOT NULL,
    questions question[] NOT NULL,
    certainly_table certainly_table NULL,
    start_time TIMESTAMPTZ NOT NULL,
    attempt_duration_minutes INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CHECK (attempt_duration_minutes > 0)
);

CREATE TABLE quiz_collaborators (
    quiz_id UUID NOT NULL REFERENCES quizzes(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (quiz_id, user_id)
);

CREATE TABLE quiz_attempts (
    id UUID PRIMARY KEY,
    quiz_id UUID NOT NULL REFERENCES quizzes(id) ON DELETE CASCADE,
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    started_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    submitted_at TIMESTAMPTZ NULL,
    status attempt_status NOT NULL,
    question_order UUID[] NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CHECK (expires_at > started_at),
    CHECK ((status = 'in_progress' AND submitted_at IS NULL) OR (status = 'submitted' AND submitted_at IS NOT NULL))
);

CREATE TABLE quiz_answers (
    attempt_id UUID NOT NULL REFERENCES quiz_attempts(id) ON DELETE CASCADE,
    question_id UUID NOT NULL,
    answer_index SMALLINT NOT NULL,
    certainty_level attempt_certainty_level NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (attempt_id, question_id)
);

CREATE INDEX idx_quizzes_owner_id ON quizzes(owner_id);
CREATE INDEX idx_quizzes_created_at ON quizzes(created_at);
CREATE INDEX idx_quizzes_join_code ON quizzes(join_code);

CREATE INDEX idx_quiz_collaborators_user_id ON quiz_collaborators(user_id);

CREATE INDEX idx_quiz_attempts_student_id ON quiz_attempts(student_id);
CREATE INDEX idx_quiz_attempts_quiz_id ON quiz_attempts(quiz_id);
CREATE UNIQUE INDEX idx_quiz_attempts_one_per_student
    ON quiz_attempts(quiz_id, student_id);

CREATE INDEX idx_quiz_answers_question_id ON quiz_answers(question_id);
