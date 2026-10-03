#[derive(Debug)]
struct TextStats {
    char_count: usize,
    word_count: usize,
    line_count: usize,
}

impl TextStats {
    /// Analyze the given text and compute its statistics.
    fn analyze(text: &str) -> Self {
        let char_count = text.chars().count();
        let word_count = text.split_whitespace().count();
        let line_count = text.lines().count();
        TextStats {
            char_count,
            word_count,
            line_count,
        }
    }

    /// Return the first word of the text, if there is one.
    fn first_word(text: &str) -> Option<&str> {
        text.split_whitespace().next()
    }

    /// Build a human-readable summary line.
    fn summary(&self) -> String {
        format!(
            "Word Count : {}\nChar Count : {}\nLine Count : {}\n",
            self.word_count, self.char_count, self.line_count,
        )
    }
}

fn main() {
    let samples = [
        "the quick brown fox\njumps over the lazy dog",
        "",
        "my foot",
        "my \n\n foot \n is \n on \n my \n way",
    ];

    for sample in samples {
        let stats = TextStats::analyze(sample);

        println!("\nSample : {}", sample);
        println!("****************************************************************\n");
        println!("{}", stats.summary());

        let first_word = TextStats::first_word(sample);
        match first_word {
            Some(val) => println!("First Word : {val}"),
            None => println!("No Words"),
        }

        println!("\n-------------------");
    }
}
