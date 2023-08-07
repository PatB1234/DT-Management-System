
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
var rowData = [];

axios.get("/items")
  .then(function (response) {
    rowData = response.data.map(function (student) { return { id: student.id, name: student.name, amount: student.amount, location: student.location } });
    let table = document.querySelector("table");
    generateTableHead(table, Object.keys(rowData[0]));
    generateTable(table, rowData)
  });