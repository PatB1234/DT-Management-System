const { invoke } = window.__TAURI__.tauri;

let amount;
let id;
let yn;
async function withdraw_item() {

    await invoke('withdraw_item', { amount: amount.value, id: id.value, deleteY: yn.value }).then((messsage) => res.innerText = messsage)
}

window.addEventListener("DOMContentLoaded", () => {
    amount = document.getElementById("amount")
    id = document.getElementById("id")
    yn = document.getElementById("yn")
    document.querySelector("#add").addEventListener("submit", (e) => {

        e.preventDefault()
        withdraw_item()

    })
})