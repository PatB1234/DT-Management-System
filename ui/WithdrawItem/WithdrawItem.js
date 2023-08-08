var result = document.getElementById("res");

function submitForm(event) {
    event.preventDefault(true);
    var id = document.getElementById("id");
    var amount = document.getElementById("amount");
    axios.post('/withdraw_item', {

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