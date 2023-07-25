const { invoke } = window.__TAURI__.tauri;

let names;
let amount;
let id;
let locations;
let res;
async function AddItem() {

    await invoke('add_item', { name: names.value, amount: amount.value, id: id.value, location: locations.value })
    res.innerText = "Successful"
}

window.addEventListener("DOMContentLoaded", () => {

    names = document.getElementById("name")
    amount = document.getElementById("amount")
    id = document.getElementById("id")
    locations = document.getElementById("location")
    res = document.getElementById("res")
    document.querySelector("#add").addEventListener("submit", (e) => {

        e.preventDefault()
        AddItem()
    })
})