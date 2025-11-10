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

Prerequisites:

- have `mise` installed
- have a postgresql DB installed and running
- have system libraries prerequisites of diesel
  - have `libpq` (typically installed with postgresql) available in your `LIBRARY_PATH`
    - for MacOS: comes with `libpq` (or `postgres@17`) if installed with homebrew, but is not in the default brew library path (`/opt/homebrew/lib`) unless `brew link --force libpq` is run. Even in that case, extending LIBRARY_PATH with `export LIBRARY_PATH="/opt/homebrew/lib:$LIBRARY_PATH"` is needed.
- install `wasm-pack` with `cargo install wasm-pack` (for plugin development)

Steps:

- `mise install` (might fail if you don't have system libs for postgres, but you can also decide to install it with another tool such as [postgresapp](https://postgresapp.com))
- `mise bootstrap`
- `mise setup-hooks` (configures pre-commit hooks using uvx if available, or Python otherwise)

### Daily development

Steps:

- `mise build`
- `mise serve` (TODO)
- `diesel migration run` (TODO: define when, and potentiall move to fqc)

### Code Quality

Format all files:

- `mise fmt` - Runs all formatters (rustfmt, prettier, markdownlint, taplo, sqlfluff, yamlfix)

Lint all files:

- `mise lint` - Runs all linters (clippy, prettier check, markdownlint, taplo, sqlfluff)

### Diesel commands

TODO: might be hidden by `fqc`, if we only need to

- clear the db, to drop all "business" tables (e.g.: `fqc db clear`)
- re-create the db tables (e.g.: `fqc db generate` -> maybe delegate to `diesel migration run`?)
- fill the db with dummy data from somewhere (e.g.: `fqc db fill`, or `fqc db populate`)

`diesel migration generate <migration_name>`

This will create a new folder in `migrations/` with up.sql and down.sql files: You have to fill those yourself.
For now there will be NO alter commands, only create and drop.

## Plugin concept

The following is a possible architecture for the plugin system.

The plugins can be public or private. Public plugins are usable for anyone, private plugins have an user or a set of users asociated with them. The plugin is uploaded to a git repository by its creators. From our perspective, the plugin is stored as a table row with an id and a link for its repository.

Each card template can be rendered with one or more plugins. That can be configured at the card type, which defines which fields does the card possess.

Each plugin has a yaml file or something similar which describes its functionality (think of Github/lab Actions repositories). For it to be a plugin, it has to be HTML/CSS/JS or WebAssembly. If it is build with any other tool, the result must be compiled to HTML/CSS/JS or WebAssembly (e.g. NextJs static compilation).

The plugin renders inside an iFrame and it obtains cards through a postMessage API which communicates the iFrame and the surrounding flash-qc website. This protocol is fixed and all plugins must use it. The plugins cannot access third party HTTP APIs through the postMessage API.

### The responsabilities of a plugin are the following

- The plugin renders a card.
- The plugin request a new card to flash-qc.
- The plugin informs of success-failure, or % of success to flash-qc.
- The plugin has a mechanism to determine success-failure with a card. This can be as simple as a set of buttons Anki style or automatic evaluation
- The plugin renders the forms or closest equivalent to add/edit/remove cards.

### The responsabilities of the flash-qc front are the following

- Generate some sort of container where the plugin is rendered. This could be the WASM VM, or an HTML iFrame.
- Disponibilize a postMessage API to communicate with the iFrame and behave as an HTTP client for the flash-qc backend server.
- Visual interface for user creation, edits and deletion of user data.
- Inspecting other user profiles
- Rendering Progress Leaderboards for public decks.
- Showing lists of public decks which can be added to your UserLibrary.
- Allow to open cards templates in json edit mode.
- Allows to configure decks:
  - Deck Recall Algorithm Parameters
  - Deck Parent tracking (autodownload updated cards from source)
- See and insert comments for public decks.
- Suggest changes on public decks.

### The responsabilities of flash-qc backend are as follows

- On request, extract the next ideal card for the user from a given deck, or from a filtered deck, based on a recall algorithm.
- Store the success-failure on cards, given by flash-qc front
- Disponiblizes a HTTP API for communication with flash-qc.
- Generate statistics for users.
- In the event of a new training session, the backend informs how many cards of each deck have to be evaluated by the user.
- Manages all CRUD operations in general (Users, decks, cards, card templates, etc)
