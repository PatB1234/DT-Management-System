var result = document.getElementById("res");

function submitForm(event) {
    event.preventDefault(true);
    var username = document.getElementById("username");
    var password = document.getElementById("password");
    axios.post('/check_login', {

        username: username.value,
        password: password.value
    })
        .then(function (response) {
            res.innerText = response.data;
            if (res.innerText == "Correct") {

                window.location.replace("/index");

            }
        })
        .catch(function (error) {
            console.error(error.response.status, error.message, error.response.data.detail)
        });

}