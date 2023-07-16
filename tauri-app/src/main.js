const { invoke } = window.__TAURI__.tauri;

function generateTableHead(table, data) {
  let thead = table.createTHead();
  let row = thead.insertRow();
  for (let key of data) {
    let th = document.createElement("th");
    let text = document.createTextNode(key);
    th.appendChild(text);
    row.appendChild(th);
  }
}
function generateTable(table, data) {
  for (let element of data) {
    let row = table.insertRow();
    for (let key in element) {
      let cell = row.insertCell();
      let text = document.createTextNode(element[key]);
      cell.appendChild(text);
    }
  }
}
async function run() {
  let rowData = []
  // each entry here represents one column
  let contents = JSON.stringify(await invoke("list_items"));
  contents = JSON.parse(contents)
  console.log(typeof (contents))
  for (let key in contents) {
    if (contents.hasOwnProperty(key)) {
      const value = contents[key];
      console.log(`Key: ${key}, Value: ${value['name']}`);
      rowData.push({ id: value['id'], name: value['name'], amount: value['amount'], location: value['location'] });
    }
  }

  let table = document.querySelector("table");
  generateTableHead(table, Object.keys(rowData[0]));
  generateTable(table, rowData)
}
document.addEventListener('DOMContentLoaded', function () {
  run()
});
