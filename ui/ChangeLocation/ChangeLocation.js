var result = document.getElementById("res");

function submitForm(event) {
    event.preventDefault(true);
    var id = document.getElementById("id");
    var location = document.getElementById("location");
    axios.post('/change_location', {

        id: id.value,
        location: location.value
    })
        .then(function (response) {
            res.innerText = response.data;
        })
        .catch(function (error) {
            console.error(error.response.status, error.message, error.response.data.detail)
        });

}