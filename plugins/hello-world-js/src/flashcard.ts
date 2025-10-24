/**
 * Flashcard class that handles the flip animation and state management
 */
class Flashcard {
  private card: HTMLElement;
  private isFlipped: boolean = false;

  constructor(cardElement: HTMLElement) {
    this.card = cardElement;
    this.init();
  }

  /**
   * Initialize the flashcard with event listeners
   */
  private init(): void {
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
document.addEventListener("DOMContentLoaded", () => {
  const cardElement = document.querySelector(".flashcard") as HTMLElement;

  if (cardElement) {
    const flashcard = new Flashcard(cardElement);

    // Expose to window for debugging/testing if needed
    (window as any).flashcard = flashcard;
  } else {
    console.error("Flashcard element not found");
  }
});
