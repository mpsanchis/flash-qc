// >>> Mandatory part: contract to satisfy by plugin
// Generates "cardId" and "getCardData", which are used to fetch card data
async function getCardData(cardId) {
    const response = await fetch(`/api/cards/${cardId}`);
    const { plugin_data } = await response.json();
    return plugin_data;
}

const urlParams = new URLSearchParams(window.location.search);
const cardId = urlParams.get('cardId');
// <<< Mandatory part: contract to satisfy by plugin

async function showFact() {
    const { fact } = await getCardData(cardId);
    document.getElementById('fact-text').textContent = fact;
}

// Handle "Memorized" button click
document.getElementById('memorized-btn').addEventListener('click', function() {
    // Send postMessage to parent window
    if (window.parent !== window) {
        window.parent.postMessage({
                type: 'finish',
                reason: 'success',
                statusCode: 0,
            },
            window.location.origin
        );
    }
});

// Listen for background color change from parent
window.addEventListener('message', function(event) {
    if (event.data && event.data.type === 'changeBackgroundColor') {
    document.body.style.background = event.data.color;
    console.log('Changed background to:', event.data.color);
    }
});

await showFact();