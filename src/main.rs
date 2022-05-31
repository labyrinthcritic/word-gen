use clap::Parser;
use rand::{thread_rng as rng, Rng};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, default_value_t = 1)]
    count: usize,

    #[clap(short, long, default_value_t = 1.0)]
    variance: f64,

    #[clap(long, default_value_t = 2)]
    min_clusters: u32,

    #[clap(long, default_value_t = 3)]
    max_clusters: u32,
}

const CONSONANTS: &str = "tnsrhldcmfpgwybvkxjqz";
const VOWELS: &str = "eaoiu";

const DEFAULT_FREQUENCY_RANK: f64 = 0.6;

fn main() {
    let args = Args::parse();

    if args.min_clusters > args.max_clusters {
        panic!("min cluster count cannot be greater than max cluster count");
    }

    let consonant_distribution = rand_distr::Zipf::new(21, DEFAULT_FREQUENCY_RANK / args.variance)
        .expect("Failed to create a Zipf distribution");
    let vowel_distribution = rand_distr::Zipf::new(5, 0.6).unwrap();

    let get_consonant = || {
        CONSONANTS
            .chars()
            .nth(rng().sample(&consonant_distribution) as usize - 1)
            .unwrap()
    };

    let get_vowel = || {
        VOWELS
            .chars()
            .nth(rng().sample(&vowel_distribution) as usize - 1)
            .unwrap()
    };

    for _ in 0..args.count {
        let mut word = String::new();

        for _ in 0..rng().gen_range(args.min_clusters..=args.max_clusters) {
            word.push(get_consonant());
            word.push(get_vowel());

            if rng().gen_bool(0.25) {
                word.push(get_vowel())
            }

            if rng().gen_bool(0.5) {
                word.push(get_consonant());
            }
        }

        println!("{word}");
    }
}
