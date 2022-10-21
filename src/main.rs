use input::{Input, StdInReader};
use output::{Output, StdOutWriter};
use word_counter::WordCounter;

mod input;
mod word_counter;
mod output;

fn main() {
    let writer = StdOutWriter;
    writer.write("Enter text: ");
    let reader = StdInReader;
    let input = reader.read();
    let word_counter = WordCounter::default();
    let count = word_counter.count(&input);
    writer.write(&format!("Number of words: {count}"));
}

