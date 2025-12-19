# Guessing Game

A simple command-line number guessing game written in Rust.

## How to Play

1. The program generates a random number between 0 and 100
2. Enter your guess when prompted
3. Receive feedback:
   - **Too high** - your guess is above the secret number
   - **Too low** - your guess is below the secret number
   - **Correct** - you win!
4. Keep guessing until you find the number

## Installation

```bash
git clone https://github.com/thestackdev/guessing-game.git
cd guessing_game
cargo build --release
```

## Usage

```bash
cargo run
```

## Dependencies

- [rand](https://crates.io/crates/rand) - Random number generation
- [colored](https://crates.io/crates/colored) - Colorful terminal output
