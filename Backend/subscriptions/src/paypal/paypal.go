package paypal

import (
    "bytes"
    "encoding/json"
    "fmt"
    "os"
    "net/http"
)

type GetAccessTokenStruct struct {
    user   string `json:"user"`
    header string `json:"header"`
    data   string `json:"data"`
}

func get_access_token() *http.Response {
    var url string = "https://api-m.sandbox.paypal.com/v1/oauth2/token";
    var client_id string = os.Getenv("PAYPAL_CLIENT_ID");
    var secret_key string = os.Getenv("PAYPAL_SECRET_KEY");

    var user string = client_id + ":" + secret_key;
    // var header string = "Content-Type: application/x-www-form-urlencoded";
    var data string = "grant_type=client_credentials";

    body := []byte(`{
        "user": ` + user + `,
        "data": ` + data + `,
    }`)

    r, err := http.NewRequest("POST", url, bytes.NewBuffer(body))
    if err != nil {
        panic(err)
    }

    r.Header.Add("Content-Type", "application/x-www-form-urlencoded")

    client := &http.Client{}
    res, err := client.Do(r)
    if err != nil {
        panic(err)
    }

    fmt.Println(res)

    return res
}

func show_sub_details() {

}

func cancel_sub() {

}

func suspend_sub() {

}

func activate_sub() {

}



