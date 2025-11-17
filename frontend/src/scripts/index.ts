import { parseEventData } from "../models/plugin-message";

let currentTimeoutCallbackHandler: NodeJS.Timeout | null = null;
const PLUGIN_TIMEOUT_MS = 20000;

const colors = ["red", "green", "blue", "yellow"];
let currentColorIndex = 0;

async function checkBackendStatus() {
  const statusElement = document.getElementById("backend-status");
  if (!statusElement) return;

  try {
    const response = await fetch("/api/system/health");
    statusElement.textContent = `Backend status: ${response.status} (${response.statusText})`;
  } catch (error) {
    console.error("Error checking backend status:", error);
    statusElement.textContent = "Backend status: unavailable";
  }
}

function loadPlugin() {
  const iframe = document.getElementById("main-iframe") as HTMLIFrameElement;
  if (!iframe) return;

  const pluginName = (window as any).currentPluginCards[
    (window as any).currentCardIndex
  ];
  iframe.src = `/api/plugin/${pluginName}`;
  const activePluginEl = document.getElementById("active-plugin");
  if (activePluginEl) {
    activePluginEl.textContent = `Plugin: ${pluginName} (${(window as any).currentCardIndex + 1}/${
      (window as any).currentPluginCards.length
    })`;
  }

  currentTimeoutCallbackHandler = setTimeout(() => {
    nextPlugin();
  }, PLUGIN_TIMEOUT_MS);
  console.log(`Loaded plugin: ${pluginName}`);
}

function nextPlugin() {
  if (currentTimeoutCallbackHandler !== null) {
    clearTimeout(currentTimeoutCallbackHandler);
    currentTimeoutCallbackHandler = null;
  }
  (window as any).currentCardIndex =
    ((window as any).currentCardIndex + 1) %
    (window as any).currentPluginCards.length;
  loadPlugin();
}

function changeBackgroundColor() {
  currentColorIndex = (currentColorIndex + 1) % colors.length;
  const color = colors[currentColorIndex];
  const iframe = document.getElementById("main-iframe") as HTMLIFrameElement;
  if (!iframe?.contentWindow) return;

  // Send postMessage to iframe to change background color
  iframe.contentWindow.postMessage(
    {
      type: "changeBackgroundColor",
      color: color,
    },
    window.location.origin,
  );

  console.log(`Changed background color to: ${color}`);
}

async function loadDecks() {
  const dropdown = document.getElementById(
    "decks-dropdown",
  ) as HTMLSelectElement;

  try {
    const response = await fetch("/api/decks");
    const decks = await response.json();

    dropdown.innerHTML = '<option value="">Select a deck...</option>';

    decks.forEach((deck: any) => {
      const option = document.createElement("option");
      option.value = deck.id;
      option.textContent = deck.name;
      dropdown.appendChild(option);
    });
  } catch (error) {
    console.error("Failed to load decks:", error);
    dropdown.innerHTML = '<option value="">Error loading decks</option>';
  }
}

async function loadDeckCards(deckId: string) {
  try {
    const response = await fetch(`/api/decks/${deckId}`);
    const pluginCards = await response.json();

    // Store cards in window so iframe can access them
    (window as any).currentPluginCards = pluginCards.cards.map(
      (c: any) => c.name,
    );
    (window as any).currentCardIndex = 0;

    loadPlugin();
  } catch (error) {
    console.error("Failed to load deck cards:", error);
  }
}

// Handle deck selection
const dropdown = document.getElementById("decks-dropdown") as HTMLSelectElement;
dropdown.addEventListener("change", (e) => {
  const deckId = (e.target as HTMLSelectElement).value;
  if (deckId) {
    loadDeckCards(deckId);
  }
});

// Listen for postMessage from iframe
window.addEventListener("message", function (event) {
  if (event.origin !== window.location.origin) {
    return;
  }

  const eventData = parseEventData(event.data);
  console.log(`Received message from plugin: ${JSON.stringify(eventData)}`);

  if (eventData && eventData.hasFinished()) {
    console.log("User marked card as memorized, cycling to next plugin...");
    nextPlugin();
  }
});

document.addEventListener("DOMContentLoaded", () => {
  checkBackendStatus();
  loadDecks();
  loadPlugin();

  // Add click handler for color button
  document
    .getElementById("color-btn")
    ?.addEventListener("click", changeBackgroundColor);
});
