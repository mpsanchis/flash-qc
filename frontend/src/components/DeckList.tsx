import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";

interface Deck {
  id: string;
  name: string;
}

export default function DeckList() {
  const [decks, setDecks] = useState<Deck[]>([]);

  useEffect(() => {
    async function loadDecks() {
      try {
        const userDecks = await fetch("/api/decks").then((res) => res.json());
        setDecks(userDecks);
      } catch (error) {
        console.error("Failed to load decks:", error);
      }
    }
    loadDecks();
  }, []);

  async function handleDelete(deckId: string) {
    if (confirm("Are you sure you want to delete this deck?")) {
      try {
        await fetch(`/api/decks/${deckId}`, { method: "DELETE" });
        setDecks(decks.filter((deck) => deck.id !== deckId));
      } catch (error) {
        console.error("Failed to delete deck:", error);
      }
    }
  }

  function handleEdit(deckId: string, deckName: string) {
    console.log(`${deckName} --> Edit clicked`);
  }

  if (decks.length === 0) {
    return (
      <div className="text-center py-12 border rounded-lg bg-muted/50">
        <p className="text-muted-foreground">
          No decks yet. Create your first deck to get started!
        </p>
      </div>
    );
  }

  return (
    <div className="grid gap-4">
      {decks.map((deck) => (
        <div
          key={deck.id}
          className="flex items-center justify-between p-6 border rounded-lg bg-card hover:bg-accent/5 transition-colors"
        >
          <div className="flex items-center gap-4">
            <div className="h-12 w-12 rounded-md bg-primary/10 flex items-center justify-center">
              <span className="text-2xl">📚</span>
            </div>
            <div>
              <h3 className="text-lg font-semibold">{deck.name}</h3>
              <p className="text-sm text-muted-foreground">Click to study</p>
            </div>
          </div>
          <div className="flex gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={() => handleEdit(deck.id, deck.name)}
            >
              Edit
            </Button>
            <Button
              variant="destructive"
              size="sm"
              onClick={() => handleDelete(deck.id)}
            >
              Delete
            </Button>
          </div>
        </div>
      ))}
    </div>
  );
}
