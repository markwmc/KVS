mod store;
use crate::store::KVStore;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kv_store")]
#[command(about = "A simple key-value store in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Set { key: String, value: String},

    Get { key: String},

    Delete { key: String},

    List,
}

fn emoji(symbol: &str) -> &str {
    symbol
}

fn main() {
    let cli = Cli::parse();
    let mut store = KVStore::new();

    match cli.command {
        Commands::Set {key, value} => {
        store.set(key, value);
        println!( "{}Key stored successfully", emoji("✅"));
        }
        Commands::Get { key } => {
            match store.get(&key) {
                Some(value) => println!("{} {}:{}", key, value, emoji("🔍")),
                None => println!("❌ Key not found."),
            }
        }
        Commands::Delete { key } => {
            store.delete(&key);
            println!("🗑️  Key deleted.");
        }
        Commands::List => {
            println!("📋  Stored keys:");
            store.list();
        }
    }
}
