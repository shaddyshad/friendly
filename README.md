## Command to run   
`systemfd --no-pid -s http::3000 -- cargo watch -x run` it shall run the http server on port 8088.

## APIs
1. `POST - ::1/upload ` to upload an xml document, make a post request with the multipart data
2. `GET - ::1/{text} ` - append the text query to the get request to resolve a node