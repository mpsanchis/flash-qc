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

// >>> Optional part: APIs that the plugin could offer
// OPT1: Offer a "Memorized" button click
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

// OPT2: Listen for background color change from parent
window.addEventListener('message', function(event) {
    if (event.data && event.data.type === 'changeBackgroundColor') {
    document.body.style.background = event.data.color;
    console.log('Changed background to:', event.data.color);
    }
});
// <<< Optional part: APIs that the plugin could offer

// >>> Core plugin functionality
async function changeDummyWord() {
    const { word } = await getCardData(cardId);
    document.getElementById('word').textContent = word;
}

await changeDummyWord();
// <<< Core plugin functionality
