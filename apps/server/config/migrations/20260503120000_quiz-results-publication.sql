ALTER TABLE quizzes
ADD COLUMN results_published_at TIMESTAMPTZ;

CREATE INDEX idx_quizzes_results_published_at ON quizzes(results_published_at);
