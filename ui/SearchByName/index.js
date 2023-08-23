var textarea = document.getElementById("res");

function submitForm(event) {
    event.preventDefault(true);
    var name = document.getElementById("name");
    axios.post('/search_by_name', {

        name: name.value
    })
        .then(function (response) {

            let arr = response.data;
            for (let i = 0; i < arr.length; i++) {


                let text = `Name: ${arr[i][0]}, Amount: ${arr[i][1]}, Location: ${arr[i][2]}, ID: ${arr[i][3]}\n`
                textarea.value += text;

            }

        })
        .catch(function (error) {
            console.error(error.response.status, error.message, error.response.data.detail)
        });

}