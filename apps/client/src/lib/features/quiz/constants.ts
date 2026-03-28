export const QUESTION_FORMAT_EXAMPLE = `{
	"questions": [
		{
			"question": "¿Cuál es la capital de Chile?",
			"options": ["Lima", "Santiago", "Bogota", "Quito"],
			"answer": 1,
			"images": ["https://example.com/chile-map.png"]
		}
	]
}`

export const DEFAULT_CERTAINTY_VALUES = {
	low: {
		correct: '1',
		incorrect: '-1'
	},
	medium: {
		correct: '2',
		incorrect: '-2'
	},
	high: {
		correct: '3',
		incorrect: '-6'
	}
} as const

export const DEFAULT_CREATE_QUIZ_INPUT = {
	title: '',
	mode: 'traditional',
	startTimeLocal: '',
	attemptDurationMinutes: '30',
	certainty: DEFAULT_CERTAINTY_VALUES
} as const
