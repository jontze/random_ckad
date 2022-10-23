use crate::page;
use rand::Rng;

#[derive(Debug, Clone)]
pub(crate) struct Question {
    pub index: usize,
    pub question: String,
    pub answer: String,
    pub category: super::Categories,
}

impl Question {
    pub fn print_question(&self) -> String {
        format!(
            "\n# {:?}\n\n{}. **{}**\n",
            self.category, self.index, self.question
        )
    }

    pub fn print_answer(&self) -> String {
        self.answer.to_string()
    }

    pub fn print(&self) -> String {
        format!("{}\n\n{}", self.print_question(), self.print_answer())
    }
}

pub(crate) fn parse_questions(md_string: String, category: page::Categories) -> Vec<Question> {
    // Get one entry for each h3 heading
    let mut questions_string: Vec<&str> = md_string.split("\n###").collect();
    // Remove heading
    questions_string.remove(0);
    let ret: Vec<Question> = questions_string
        .iter()
        .enumerate()
        .map(|(index, question_str)| {
            // The question ends and the answer begins
            let question_md = question_str.split_once("<p>").unwrap();
            // Clear the question from unwanted strings
            let question = question_md
                .0
                .replace("<details><summary>show</summary>", "")
                .trim()
                .to_string();
            // Clear the answer from unwanted strings
            let answer = question_md
                .1
                .replace("</details>", "")
                .replace("</p>", "")
                .trim()
                .to_string();

            Question {
                index: index + 1,
                question,
                answer,
                category,
            }
        })
        .collect();
    ret
}

pub(crate) fn get_random_question(questions: &[Question]) -> Option<&Question> {
    let mut rng = rand::thread_rng();
    questions.get(rng.gen_range(0..questions.len()))
}
