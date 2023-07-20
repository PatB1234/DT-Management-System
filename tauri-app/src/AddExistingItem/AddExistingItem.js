const { invoke } = window.__TAURI__.tauri;

let id;
let amount;
let res;
async function AddItem() {

    await invoke('add_existing_item', { id: id.value, amount: amount.value }).then((message) => res.innerText = message)
}

window.addEventListener("DOMContentLoaded", () => {

    amount = document.getElementById("amount")
    id = document.getElementById("id")
    res = document.getElementById("res")
    document.querySelector("#add").addEventListener("submit", (e) => {

        e.preventDefault()
        AddItem()
    })
})