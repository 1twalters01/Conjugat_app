// This is done in go and not django as speed is critical for payment resolution
package main

import (
	"fmt"
	"log"
    "paypal/paypal"
)

func main() {
	fmt.Println("test")
	log.Fatal(":8000")

    get_access_token()
}
