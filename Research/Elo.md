# Mathematical details
## Implementation details
Expected scores calculation given the two players' ratings
* E_A = (1) / (1 + 10^((R_B + R_A)/400))
* E_B = (1) / (1 + 10^((R_A + R_B)/400))

Formulae for updating the players' rating
* R^(')_(A) = R_A + K(S_A - E_A)
* R^(')_(B) = R_B + K(S_B - E_B)

## Example
* Player A Elo: 2600
* Player B Elo: 2300

Let's say for this example that K = 16

* Player A expected score: 1 / (1 + 10^((2300-2600)/400)) = 0.849
* Player B expected score: 1 / (1 + 10^((2600-2300)/400)) = 0.151

If player A wins:
* Player A new Elo: 2600 + 16 (1 – 0.849) = 2602
* Player B new Elo: 2300 + 16 (1 – 0.151) = 2298

If player B wins:
* Player A new Elo: 2600 + 16 (1 – 0.849) = 2586
* Player B new Elo: 2300 + 16 (1 – 0.151) = 2314

# K factor

