//! Data set generator for the examples.

use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

use poetry_contest::find_workspace_root;
use rand::Rng;
use serde::Serialize;

fn main() {
    // Replaces contents of `data/` in the workspace root with a fresh data set, deleting existing
    // contents of `data/` first and creating the directory if it does not already exist.
    //
    // The workspace root is defined as the directory containing `Cargo.toml`, searching upwards
    // from the current directory.
    //
    // The manifest is serialized as `manifest.json` in the `data/` directory, and all entry files
    // are created in the same directory, named `<unique index>.txt`.

    let workspace_root = find_workspace_root();
    let data_dir = workspace_root.join("data");

    // Delete and recreate the data directory
    if data_dir.exists() {
        println!("Removing existing data directory: {}", data_dir.display());
        fs::remove_dir_all(&data_dir).expect("Failed to remove existing data directory");
    }
    fs::create_dir_all(&data_dir).expect("Failed to create data directory");

    println!("Generating data set in: {}", data_dir.display());

    // Generate categories
    let mut categories = Vec::with_capacity(CATEGORY_COUNT);
    for _ in 0..CATEGORY_COUNT {
        categories.push(generate_category());
    }
    println!("Generated {} categories", categories.len());

    // Generate rounds with entries
    let mut rounds = Vec::with_capacity(ROUND_COUNT);
    let mut entry_index = 0;
    for round_idx in 0..ROUND_COUNT {
        let (round, new_index) = generate_round(&data_dir, entry_index);
        entry_index = new_index;
        rounds.push(round);
        if (round_idx + 1) % 10 == 0 || round_idx == ROUND_COUNT - 1 {
            println!(
                "Generated {}/{} rounds ({} total entries)",
                round_idx + 1,
                ROUND_COUNT,
                entry_index
            );
        }
    }

    // Create manifest
    let manifest = Manifest { categories, rounds };

    // Serialize manifest to JSON
    let manifest_path = data_dir.join("manifest.json");
    let manifest_json =
        serde_json::to_string_pretty(&manifest).expect("Failed to serialize manifest");
    fs::write(&manifest_path, manifest_json).expect("Failed to write manifest file");

    println!("Data generation complete!");
    println!("Manifest: {}", manifest_path.display());
    println!("Total entries: {}", entry_index);
}

const ROUND_COUNT: usize = 200;
const CATEGORY_COUNT: usize = 20;
const AUTHOR_COUNT: usize = 250;
const MIN_ENTRIES_PER_ROUND: usize = 500;
const MAX_ENTRIES_PER_ROUND: usize = 2000;
const MIN_ENTRY_WORDS: usize = 20;
const MAX_ENTRY_WORDS: usize = 250;
const MIN_CATEGORY_KEYWORDS: usize = 1;
const MAX_CATEGORY_KEYWORDS: usize = 100;
const LINE_LENGTH_WORDS: usize = 10;

/// How many words there are in the vocabulary we use.
///
/// For simplify, we do not use real words, we just use integers as words. The first word is "1",
/// then "2", and so on up to `VOCABULARY_SIZE`.
const VOCABULARY_SIZE: usize = 128_000;

/// The manifest is the root object of the data set.
///
/// It defines all the metadata and references all the other files that make up the data set.
///
/// This is the manifest of one poetry contest, whereby entries from different rounds are evaluated
/// against different categories, with the authors gaining points based on the scores they receive
/// in each round of the contest.
#[derive(Serialize)]
struct Manifest {
    categories: Vec<Category>,
    rounds: Vec<Round>,
}

/// One category that entries are evaluated against.
///
/// A category is simply a set of keywords that are used to identify entries
/// that belong in that category.
#[derive(Serialize)]
struct Category {
    /// Keywords that define the category.
    ///
    /// Between `MIN_CATEGORY_KEYWORDS` and `MAX_CATEGORY_KEYWORDS` keywords in each category,
    /// randomly chosen from the vocabulary.
    keywords: Vec<&'static str>,
}

/// One round of the contest.
#[derive(Serialize)]
struct Round {
    /// All the entries that compete in the round.
    ///
    /// Between `MIN_ENTRIES_PER_ROUND` and `MAX_ENTRIES_PER_ROUND` entries in each round, random.
    entries: Vec<Entry>,
}

#[derive(Serialize)]
struct Entry {
    /// Name of the author - the person that any scoring is attributed to.
    author: String,

    /// Relative path from the directory of the manifest file
    /// to the file containing the entry contents.
    ///
    /// This will be a `<unique index>.txt` file in the same directory as the manifest.
    /// The index is unique across all entries in all rounds, we start from 0 and just increment.
    path: PathBuf,
}

static VOCABULARY: LazyLock<Vec<String>> = LazyLock::new(|| {
    let mut words = Vec::with_capacity(VOCABULARY_SIZE);

    for i in 1..=VOCABULARY_SIZE {
        words.push(i.to_string());
    }

    words
});

fn word() -> &'static str {
    /// There is more diversity in word length at the low end of the range (99 and 100 differ more
    /// significantly than 2099 and 2100), so we bias the distribution toward the low end.
    const LOW_BIAS_POWER: usize = 3;

    let selector = rand::rng().random::<f64>().powf(LOW_BIAS_POWER as f64);
    let word_index = (selector * (VOCABULARY_SIZE as f64)) as usize + 1;
    VOCABULARY[word_index - 1].as_str()
}

fn generate_category() -> Category {
    let keyword_count = rand::rng().random_range(MIN_CATEGORY_KEYWORDS..=MAX_CATEGORY_KEYWORDS);
    let keywords = (0..keyword_count).map(|_| word()).collect();
    Category { keywords }
}

fn generate_round(data_dir: &PathBuf, mut entry_index: usize) -> (Round, usize) {
    let entry_count = rand::rng().random_range(MIN_ENTRIES_PER_ROUND..=MAX_ENTRIES_PER_ROUND);
    let mut entries = Vec::with_capacity(entry_count);

    for _ in 0..entry_count {
        let entry = generate_entry(data_dir, entry_index);
        entries.push(entry);
        entry_index += 1;
    }

    (Round { entries }, entry_index)
}

thread_local!(static CONTENTS_BUFFER: RefCell<String> = RefCell::new(String::new()));

fn generate_entry(data_dir: &PathBuf, index: usize) -> Entry {
    let mut rng = rand::rng();

    // Generate author (random from 0 to AUTHOR_COUNT-1)
    let author_index = rng.random_range(0..AUTHOR_COUNT).to_string();

    // Generate entry file path
    let filename = format!("{}.txt", index);
    let file_path = data_dir.join(&filename);

    // Generate entry content
    let word_count = rng.random_range(MIN_ENTRY_WORDS..=MAX_ENTRY_WORDS);

    CONTENTS_BUFFER.with_borrow_mut(|buffer| {
        for i in 0..word_count {
            if i > 0 {
                // Check if we should insert a line break
                if i % LINE_LENGTH_WORDS == 0 {
                    buffer.push('\n');
                } else {
                    buffer.push(' ');
                }
            }

            buffer.push_str(word());
        }

        // Write the entry content to file
        fs::write(&file_path, &buffer).expect(&format!(
            "Failed to write entry file: {}",
            file_path.display()
        ));

        buffer.clear();
    });

    Entry {
        author: author_index,
        path: PathBuf::from(filename),
    }
}
