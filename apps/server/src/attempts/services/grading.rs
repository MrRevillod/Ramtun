use crate::{
    attempts::*,
    banks::{Question, QuestionId},
    quizzes::*,
    shared::AppResult,
};

use std::collections::HashMap;
use sword::prelude::*;

type PointMappingFn<'a> = Box<dyn Fn(bool, CertaintyLevel, Option<&'a CertaintyTable>) -> i16 + 'a>;

pub struct QuestionGradingOutput {
    pub question_id: QuestionId,
    pub is_correct: bool,
    pub awarded_points: i16,
}

pub struct GradingOutput {
    pub score_points: i16,
    pub grade: f64,
    pub question_results: Vec<QuestionGradingOutput>,
}

#[injectable]
pub struct GradingService;

impl GradingService {
    pub async fn grade_attempt(
        &self,
        quiz: &Quiz,
        questions: &HashMap<QuestionId, Question>,
        answers: &HashMap<QuestionId, AttemptAnswer>,
    ) -> AppResult<GradingOutput> {
        let score_points_max = quiz.max_score;
        let table = quiz.certainty_table.as_ref();

        let mut score_points = 0_i16;
        let mut question_results = Vec::new();

        let calc_points_fn = self.calc_points_fn(quiz.kind);

        for (question_id, question) in questions.iter() {
            let Some(submitted_answer) = answers.get(question_id) else {
                continue;
            };

            let is_correct = question.answer_index == submitted_answer.answer_index;

            // Uses `unwrap_or_default` even if the quiz isn't certainty based
            // The closure ignores the certainty_level if the quiz it's traditional.

            let certainty_level = submitted_answer.certainty_level.unwrap_or_default();
            let awarded_points = calc_points_fn(is_correct, certainty_level, table);

            score_points += awarded_points;

            question_results.push(QuestionGradingOutput {
                question_id: *question_id,
                is_correct,
                awarded_points,
            });
        }

        let mut grade = self.calculate_grade(score_points, score_points_max);

        if score_points <= 0 {
            grade = 1_f64
        }

        Ok(GradingOutput {
            score_points,
            grade,
            question_results,
        })
    }

    fn calculate_grade(&self, p: i16, p_max: i16) -> f64 {
        ((p as f64 / p_max as f64) * 6_f64) + 1_f64
    }

    fn calc_points_fn(&self, quiz_kind: QuizKind) -> PointMappingFn<'_> {
        match quiz_kind {
            QuizKind::Traditional => {
                Box::new(|is_correct, _, _| self.calc_traditional_points(is_correct))
            }
            QuizKind::Certainty => Box::new(|is_correct, level, table| {
                self.calc_certainty_points(is_correct, level, table)
            }),
        }
    }

    fn calc_traditional_points(&self, is_correct: bool) -> i16 {
        if is_correct { 1 } else { 0 }
    }

    fn calc_certainty_points(
        &self,
        is_correct: bool,
        level: CertaintyLevel,
        table: Option<&CertaintyTable>,
    ) -> i16 {
        let Some(table) = table else {
            return 0;
        };

        match (level, is_correct) {
            (CertaintyLevel::Low, true) => table.low.correct,
            (CertaintyLevel::Low, false) => table.low.incorrect,

            (CertaintyLevel::Medium, true) => table.medium.correct,
            (CertaintyLevel::Medium, false) => table.medium.incorrect,

            (CertaintyLevel::High, true) => table.high.correct,
            (CertaintyLevel::High, false) => table.high.incorrect,
        }
    }
}
