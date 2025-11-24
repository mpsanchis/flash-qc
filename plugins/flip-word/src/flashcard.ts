// >>> Mandatory part: contract to satisfy by plugin
// Generates "cardId" and "getCardData", which are used to fetch card data
const urlParams = new URLSearchParams(window.location.search);
const cardId = urlParams.get('cardId');

async function getCardData() {
    const response = await fetch(`/api/cards/${cardId}`);
    const { plugin_data } = await response.json();
    return plugin_data;
}
// <<< Mandatory part: contract to satisfy by plugin

/**
 * Flashcard class that handles the flip animation and state management
 */
class Flashcard {
  private card: HTMLElement;
  private isFlipped: boolean = false;

  constructor(cardElement: HTMLElement, frontContent: string, backContent: string) {
    this.card = cardElement;
    this.init(frontContent, backContent);
  }

  /**
   * Initialize the flashcard with event listeners
   */
  private init(frontContent: string, backContent: string): void {
    const frontContentEl = this.card.querySelector('.flashcard-front .flashcard-content');
    const backContentEl = this.card.querySelector('.flashcard-back .flashcard-content');

    if (frontContentEl) frontContentEl.textContent = frontContent;
    if (backContentEl) backContentEl.textContent = backContent;

    this.card.addEventListener("click", () => this.flip());
    this.card.addEventListener("keydown", (e: KeyboardEvent) => {
      if (e.key === "Enter" || e.key === " ") {
        e.preventDefault();
        this.flip();
      }
    });
  }

  /**
   * Flip the card between front and back
   */
  public flip(): void {
    this.isFlipped = !this.isFlipped;

    if (this.isFlipped) {
      this.card.classList.add("flipped");
    } else {
      this.card.classList.remove("flipped");
    }
  }

  /**
   * Reset the card to front side
   */
  public reset(): void {
    this.isFlipped = false;
    this.card.classList.remove("flipped");
  }

  /**
   * Get the current state of the card
   */
  public getIsFlipped(): boolean {
    return this.isFlipped;
  }
}

/**
 * Initialize the flashcard when DOM is ready
 */
document.addEventListener("DOMContentLoaded", async () => {
  const cardElement = document.querySelector(".flashcard") as HTMLElement;

  if (cardElement) {
    const {
        frontContent,
        backContent
    } = await getCardData();
    const flashcard = new Flashcard(cardElement, frontContent, backContent);

    // Expose to window for debugging/testing if needed
    (window as any).flashcard = flashcard;
  } else {
    console.error("Flashcard element not found");
  }
});
