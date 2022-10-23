use clap::Parser;
use page::{Categories, Page};

mod page;
mod question;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Question category
    #[arg(short, long)]
    category: Option<Categories>,
    /// Ask for the answer of a category
    #[arg(short, long)]
    answer: Option<usize>,
    /// Show the question with the answer
    #[arg(short, long)]
    full: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let skin = termimad::MadSkin::default();
    match cli.category.and_then(|cat| Page::from_category(cat).ok()) {
        Some(page) => {
            let questions = page.fetch_questions().await.unwrap();
            if let Some(answer_id) = cli.answer {
                let question_answer = questions
                    .iter()
                    .find(|q_ans| q_ans.index == answer_id)
                    .unwrap();
                println!("{}", skin.term_text(&question_answer.print()));
            } else {
                let question = question::get_random_question(&questions).unwrap();
                let text = if cli.full {
                    question.print()
                } else {
                    question.print_question()
                };
                println!("{}", skin.term_text(&text));
            }
        }
        None => {
            let ran_page = Page::from_random().unwrap();
            let ran_question = ran_page.fetch_random_question().await.unwrap();
            let text = if cli.full {
                ran_question.print()
            } else {
                ran_question.print_question()
            };
            println!("{}", skin.term_text(&text));
        }
    }
}
