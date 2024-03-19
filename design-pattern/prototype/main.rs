#[derive(Clone)]
struct Question {
    answer: String,
}

impl Question {
    fn new(answer: &str) -> Self {
        Self {
            answer: answer.to_string(),
        }
    }

    fn get_answer(&self) -> &str {
        self.answer.as_str()
    }

    fn set_answer(&mut self, answer: String) {
        self.answer = answer
    }
}

#[derive(Clone)]
struct ExaminationPaper {
    choice_questions: Vec<Question>,
    short_answer_questions: Vec<Question>,
    name: String,
}

impl ExaminationPaper {
    fn new(
        name: &str,
        choice_questions: Vec<Question>,
        short_answer_questions: Vec<Question>,
    ) -> Self {
        Self {
            name: name.to_string(),
            choice_questions,
            short_answer_questions,
        }
    }

    fn print(&self) {
        println!("{} paper:", self.name);
        println!(
            "{}",
            self.choice_questions
                .iter()
                .map(|q| q.get_answer())
                .collect::<Vec<_>>()
                .join(" ")
        );
        println!(
            "{}",
            self.short_answer_questions
                .iter()
                .map(|q| q.get_answer())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}

fn main() {
    let xiaohong_paper = &ExaminationPaper::new(
        "xiaohong",
        vec![Question::new("A"), Question::new("B")],
        vec![Question::new("answer1."), Question::new("answer2.")],
    );
    xiaohong_paper.print();

    let xiaoming_paper = &mut xiaohong_paper.clone();
    xiaoming_paper.name = "xiaoming".to_string();
    for q in xiaoming_paper.short_answer_questions.iter_mut() {
        q.set_answer(format!("{} {}", q.get_answer(), "That's all. Thanks!"));
    }
    xiaoming_paper.print();
}
