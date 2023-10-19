package webhooks

import (
  "fmt"
  "https://github.com/gofiber/fiber"
)

struct subscriber {
  
}

func PaymentMethod (method String) uint8 {
    var value uint8
    if method == "Stripe" {
        value = 1;
    } else if method == "Paypal" {
        value = 2;
    } else if method == "Coinbase" {
        value = 3;
    }
  
    return value
}

