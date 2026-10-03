use std::fs;

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
    let path = "../Cargo.toml";
    let sample = fs::read_to_string(path);
    match sample {
        Ok(contents) => {
            let stats = TextStats::analyze(&contents);
            println!("{}", stats.summary());

            let first_word = TextStats::first_word(&contents);
            match first_word {
                Some(val) => println!("First Word : {val}"),
                None => println!("No Words"),
            }
        }
        Err(e) => {
            eprintln!("Error reading the {path} : {e}");
        }
    }
}
