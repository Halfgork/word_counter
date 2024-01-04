use std::io;

// Define a struct named WordCounter
struct WordCounter {
    text: String,
}

// Implement the WordCounter struct
impl WordCounter {
    // Implement the new function
    fn new(text: &str) -> WordCounter {
        WordCounter {
            text: String::from(text),
        }
    }

    // Implement the count_words function
    fn count_words(&self) -> usize {
        // Split the text by whitespace characters and count the number of elements
        self.text.split_whitespace().count()
    }
}

fn main() {
    // Prompt the user for text input
    println!("Enter a text:");

    // Read user input from the console
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    // Create a WordCounter instance using the input text
    let word_counter = WordCounter::new(&user_input);

    // Call the count_words function and print the word count to the screen
    let word_count = word_counter.count_words();
    println!("Word count: {}", word_count);
}
