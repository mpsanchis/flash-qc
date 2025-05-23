# flash-qc
App to create flashcards for anything (cualquier cosa)

# Architectural Discussion

(Add what you think is best :D)

1. Single Monolitic system: Storage as files for flashcards, simple json (complex entities are represented in base64 or simply links to wherever they are stored.) Each flashcard itself is either Markdown or HTML. 

    Tech: React -- React Native: Why? Because we can develop a single thing and it works in the phone and as a downloadable app in the computer with electron. A small paid service for Sync could be offered. Plugins would be in lua because we can embed lua easily in the app and people are used to making plugins with it.

    Pros: Quite easish.
    Cons: Hard to scale, hard to maintain, hard to add new features. But good as a prototype.

2. Traditional Front-Back system. Back: Lambdas (we could have lots of users before even being close to hitting the free tier limits). Storing flashcards in S3, same format as section 1. 
    Tech:
    - Front: React

    - Back: Rust would be cool here, but python also works for a prototype.

    Monetization: We could give users a given amount of free flashcards per day and charge after that. Of course, free at the beginning.

    Plugins: Inyectable javascript with a defined aplication interface.





