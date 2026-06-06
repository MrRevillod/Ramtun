-- Recreate warning_type enum with finer-grained categories.
-- Old values are kept so existing records still deserialize.
-- The old 'focus_loss' / 'navigation' / 'clipboard' types are no longer
-- emitted by the anti-cheat but remain in the DB for historical data.

CREATE TYPE warning_type_new AS ENUM (
	'focus_loss',
	'clipboard',
	'screenshot',
	'navigation',
	'devtools',
	'window_blur',
	'tab_hide',
	'alt_tab',
	'meta_key',
	'context_menu',
	'copy_attempt',
	'search_attempt'
);

ALTER TABLE attempt_warnings
	ALTER COLUMN warning_type TYPE warning_type_new
	USING (warning_type::text::warning_type_new);

DROP TYPE warning_type;

ALTER TYPE warning_type_new RENAME TO warning_type;
