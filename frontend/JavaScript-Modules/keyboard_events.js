let searching = false;

// Checks what the user clicks and acts on it
async function onEnter(event) {

    if (event.key !== "Enter" || searching) {
        return;
    }

    searching = true;

    try {
        const input = document.getElementById("search-box-input");
        await displayProducts(input, true);
    } finally {
        searching = false;
    }
}

async function onSlash(event) {
    if (
        event.key !== "/" ||
        event.ctrlKey ||
        event.altKey ||
        event.metaKey
    ) {
        return;
    }

    if (
        event.target instanceof HTMLInputElement ||
        event.target instanceof HTMLTextAreaElement ||
        event.target.isContentEditable
    ) {
        return;
    }

    event.preventDefault();
    document.querySelector(".search-box").focus();
}

document.addEventListener("keydown", onSlash);

const searchBox = document.getElementById("search-box-input");

searchBox.addEventListener("keydown", onEnter);