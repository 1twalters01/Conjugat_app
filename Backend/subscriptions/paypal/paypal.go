package paypal

import (
    "bytes"
    // "encoding/json"
    "fmt"
    "github.com/joho/godotenv"
    "net/http"
)

// type GetAccessTokenStruct struct {
//     user   string `json:"user"`
//     header string `json:"header"`
//     data   string `json:"data"`
// }

// func GetAccessToken() {
func GetAccessToken() *http.Response {
    envFile, _ := godotenv.Read(".env")

    var url string = "https://api-m.sandbox.paypal.com/v1/oauth2/token";
    var client_id string = envFile["PAYPAL_CLIENT_ID"];
    var secret_key string = envFile["PAYPAL_SECRET_KEY"];

    var user string = client_id + ":" + secret_key;
    fmt.Println(user);

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
    fmt.Println(r)

    client := &http.Client{}
    res, err := client.Do(r)
    if err != nil {
        panic(err)
    }

    fmt.Println(res)

    return res
}

func ShowSubDetails() {

}

func CancelSub() {

}

func SuspendSub() {

}

func ActivateSub() {

}



