# flash-qc

App to create flashcards for anything (cualquier cosa)

## Projects

- **docker-compose-poc/**: Docker Compose multi-service architecture demonstration with Nginx, Astro frontend, Rust backend, and PostgreSQL
- **diesel-poc/**: Diesel ORM exploration (Rust)

## Architectural Discussion

(Add what you think is best :D)

~~1. Single Monolitic system: Storage as files for flashcards, simple json (complex entities are represented in base64 or simply links to wherever they are stored.) Each flashcard itself is either Markdown or HTML.~~

   ~~Tech: React -- React Native: Why? Because we can develop a single thing and it works in the phone and as a downloadable app in the computer with electron. A small paid service for Sync could be offered. Plugins would be in lua because we can embed lua easily in the app and people are used to making plugins with it.~~

   ~~Pros: Quite easish.~~
   ~~Cons: Hard to scale, hard to maintain, hard to add new features. But good as a prototype.~~

1. Traditional Front-Back system. Back: Lambdas (we could have lots of users before even being close to hitting the free tier limits). Storing flashcards in S3, same format as section 1.
    Tech:
    - Front: React

    - Back: Rust would be cool here, but python also works for a prototype.

    Monetization: We could give users a given amount of free flashcards per day and charge after that. Of course, free at the beginning.

    Plugins: Inyectable javascript with a defined aplication interface.

## Rust tooling

- Diesel as ORM:
    Tested. It does require keeping the migrations manually made with SQL, but otherwise seems to work very well.
- Rocket for HTTP engine:
    Not tested in the repo yet, but tested by AN at work. The maintenance on this repo seems to be low though, it can be that it just has become stable. AN is quite happy with it :)

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

# Setup

- Install Diesel CLI

```sh

sudo apt install libpq-dev
sudo apt install libsqlite3-dev
cargo build
cargo install diesel_cli
diesel setup
diesel migration run
```

## Migration add

`diesel migration generate <migration_name>`
