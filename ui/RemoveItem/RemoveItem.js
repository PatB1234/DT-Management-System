var res = document.getElementById("res");
function submitForm(event) {

    event.preventDefault();
    var id = document.getElementById("id");
    axios.post('/remove_item', {

        id: id.value
    })
        .then(function (response) {
            res.innerText = response.data;
        })
        .catch(function (error) {
            console.error(error.response.status, error.message, error.response.data.detail)
        });

}