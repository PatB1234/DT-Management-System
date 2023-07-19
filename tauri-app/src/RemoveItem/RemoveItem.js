const { invoke } = window.__TAURI__.tauri;

let id;

async function removeItem() {

    console.log("Test")
    await invoke('remove_item', { id: id.value });
}

window.addEventListener("DOMContentLoaded", () => {

    id = document.getElementById("id")
    document.querySelector("#form_remove").addEventListener("submit", (e) => {

        e.preventDefault()
        removeItem()
    })
})