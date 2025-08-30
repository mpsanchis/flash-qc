# flash-qc

App to create flashcards for anything (cualquier cosa)

## Projects

- **docker-compose-poc/**: Docker Compose multi-service architecture demonstration with Nginx, Astro frontend, Rust backend, and PostgreSQL
- **diesel-poc/**: Diesel ORM exploration (Rust)

## Architectural Discussion

Traditional Front-Back system. Back: Lambdas (we could have lots of users before even being close to hitting the free tier limits). Storing flashcards in S3, same format as section 1.

Tech:
    - Front: Astro
    - Back: Rust (Rocket, Diesel)

## Monetization

- We could give users a given amount of free flashcards per day and charge after that. Of course, free at the beginning.

## Plugins

- Inyectable javascript with a defined aplication interface.

## Rust tooling

- Diesel as ORM:
    Tested. It does require keeping the migrations manually made with SQL, but otherwise seems to work very well.
- Rocket for HTTP engine:
    Tested.  

## Interesting ideas

- Suggest changes to card author, using git as a backend for this, and make an ui to hide it.
- Gamify: Interesting, it can be done at the card level, which allows people to modify their decks without losing the competition posibilities.
  - Deck "Leaderboards" ala Duolingo would be nice-to-have
- Automatic definition for language learning use case (by fetching from dictionary and/or LLM)
- Import decks from Anki

## Weak Anki points

- Python as plugin language is not great for web
- Interface is too highly convoluted and complex.
- Phone apps dont have plugins
  - The experience in android and iOS is very different
  - Offer different features

## Setup

### First time ever

#### Running the pre-commit hook setup

Make the setup script executable and run it (this will install the project's pre-commit hooks):

```bash
chmod +x ./setup.sh && ./setup.sh
```

#### Assuming you have rust

```sh

sudo apt install libpq-dev
sudo apt install libsqlite3-dev
cargo build
cargo bininstall diesel_cli
diesel setup
```

### On every start

`diesel migration run`

### Adding new Migrations

`diesel migration generate <migration_name>`

This will create a new folder in `migrations/` with up.sql and down.sql files: You have to fill those yourself.
For now there will be NO alter commands, only create and drop.
