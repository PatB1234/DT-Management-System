var result = document.getElementById("res");

function submitForm() {
    var id = document.getElementById("id");
    var amount = document.getElementById("amount");
    axios.post('/add_existing_item', {

        id: id.value,
        amount: amount.value
    })
        .then(function (response) {
            res.innerText = response.data;
        })
        .catch(function (error) {
            console.error(error.response.status, error.message, error.response.data.detail)
        });

}