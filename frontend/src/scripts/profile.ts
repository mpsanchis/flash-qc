const addButton = (parent: HTMLElement, text: string) => {
  const btn = parent.appendChild(document.createElement("button"));
  btn.textContent = text;
  btn.addEventListener("click", () => {
    console.log(`${parent.textContent} --> ${text} clicked`);
  });

  return btn;
};

async function populateUserDecks() {
  const userDecks = await fetch("/api/decks").then((res) => res.json());
  const deckList = document.getElementById("deck-list");
  if (!deckList) {
    console.error("Deck list element not found");
    return;
  }

  userDecks.forEach((deck: { id: string; name: string }) => {
    const deckItem = document.createElement("li");
    deckItem.textContent = deck.name;

    addButton(deckItem, "Edit");

    let deleteButton = addButton(deckItem, "Delete");
    deleteButton.addEventListener("click", async () => {
      if (confirm("Are you sure you want to delete this deck?")) {
        await fetch(`/api/decks/${deck.id}`, { method: "DELETE" });
        deckList.removeChild(deckItem);
      }
    });

    deckList.appendChild(deckItem);
  });
}

document.addEventListener("DOMContentLoaded", async () => {
  await populateUserDecks();
});
