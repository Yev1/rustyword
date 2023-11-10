# rustyword
A word-guessing game. A project to try out Rust as a programming language.

## User interface
The idea is that a user should interact with a landing HTML page: so far served locally at 127.0.0.1:3030/index.html.
A user enters a guess for a 5-letter word and the correct letters are displayed. A new target word is generated on page refresh.

## Repository structure
| Src | Source folder containing the main Rust code. |
| Data | Stores the list of words for the game. |
| Frontend | Webpage interface. |

## How to use
Official tutorial for installing Rust: rust-lang.org/tools/install.
Navigate to the source folder and run the project:
{
    cargo run
}

# Frontend
It's a simple HTML page that has javascript powering the user interaction like inputting the guess.

# Src
It is responsible for powering the server and listening for specific requests.
Word guess processing is done here. It can generate a new word.