import { parseEventData } from "../models/plugin-message";

const plugins = [
	"hello-world-js",
	"dummy",
	"da-vinci-facts",
	"drawing-canvas",
]; // List of available plugins
let currentPluginIndex = 0;

const colors = ["red", "green", "blue", "yellow"];
let currentColorIndex = 0;

async function checkBackendStatus() {
	const statusElement = document.getElementById("backend-status");
	if (!statusElement) return;

	try {
		const response = await fetch(`/api/system/health`);
		statusElement.textContent = `Backend status: ${response.status} (${response.statusText})`;
	} catch (error) {
		console.error("Error checking backend status:", error);
		statusElement.textContent = "Backend status: unavailable";
	}
}

function loadPlugin(index: number) {
	const iframe = document.getElementById("main-iframe") as HTMLIFrameElement;
	if (!iframe) return;
	const pluginName = plugins[index];
	iframe.src = `/api/plugin/${pluginName}`;
	const activePluginEl = document.getElementById("active-plugin");
	if (activePluginEl) {
		activePluginEl.textContent = `Plugin: ${pluginName} (${index + 1}/${
			plugins.length
		})`;
	}
	console.log(`Loaded plugin: ${pluginName}`);
}

function nextPlugin() {
	currentPluginIndex = (currentPluginIndex + 1) % plugins.length;
	loadPlugin(currentPluginIndex);
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

// Listen for postMessage from iframe
window.addEventListener("message", function (event) {
	if (event.origin !== window.location.origin) {
		return;
	}

	const eventData = parseEventData(event.data);

	console.log("Received message from plugin:", event.data);

	if (eventData && eventData.type === "finish") {
		console.log(
			"User marked card as memorized, cycling to next plugin...",
		);
		nextPlugin();
	}
});

document.addEventListener("DOMContentLoaded", () => {
	checkBackendStatus();
	loadPlugin(currentPluginIndex);

	// Add click handler for color button
	document
		.getElementById("color-btn")
		?.addEventListener("click", changeBackgroundColor);
});
