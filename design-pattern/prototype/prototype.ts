interface Prototype {
    clone(): Prototype
}

class Question implements Prototype {
    private answer: string

    constructor(answer: string) {
        this.answer = answer
    }

    setAnswer(answer: string) {
        this.answer = answer
    }

    getAnswer(): string {
        return this.answer
    }

    clone(): Prototype {
        return new Question(this.answer)
    }
}

class ExaminationPaper implements Prototype {
    choiceQuestions: Question[]
    shortAnswerQuestions: Question[]
    name: string

    constructor(
        name: string,
        choiceQuestions: Question[],
        shortAnswerQuestions: Question[]
    ) {
        this.name = name
        this.choiceQuestions = choiceQuestions
        this.shortAnswerQuestions = shortAnswerQuestions
    }

    clone(): Prototype {
        return new ExaminationPaper(
            this.name,
            this.choiceQuestions.map((q) => q.clone() as Question),
            this.shortAnswerQuestions.map((q) => q.clone() as Question)
        )
    }

    print() {
        console.log(this.name, 'paper:')
        console.log(this.choiceQuestions.map((q) => q.getAnswer()))
        console.log(this.shortAnswerQuestions.map((q) => q.getAnswer()))
    }
}

const xiaohongPaper = new ExaminationPaper(
    'xiaohong',
    [new Question('A'), new Question('B')],
    [new Question('answer1.'), new Question('answer2.')]
)
xiaohongPaper.print()

// Copy xiaohong's paper
const xiaomingPager = xiaohongPaper.clone() as ExaminationPaper
// Modify name
xiaomingPager.name = 'xiaoming'
// For short answer questions, add a closing word to the end
xiaomingPager.shortAnswerQuestions.forEach((q) =>
    q.setAnswer(q.getAnswer() + `That's all, thanks!`)
)
xiaomingPager.print()
