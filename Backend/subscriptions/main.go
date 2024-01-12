// This is done in go and not django as speed is critical for payment resolution
package main

import (
	"fmt"
	// "log"
    // "subscriptions/paypal"
)

func main() {
	fmt.Println("test");
	// log.Fatal(":8000");
    tester();

    fmt.Println("ayo");
    // paypal.GetAccessToken()
}

func tester() {
    fmt.Println("test this");
}
