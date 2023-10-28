# Mathematical details
## Implementation details
Expected scores calculation given the two players' ratings
* E_A = (1) / (1 + 10^((R_B + R_A)/400))
* E_B = (1) / (1 + 10^((R_A + R_B)/400))

Formulae for updating the players' rating
* R^(')_(A) = R_A + K(S_A - E_A)
* R^(')_(B) = R_B + K(S_B - E_B)

S in the case of a win = 1, 0.5, 0

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
FIDE (chess) uses the following:
* K = 40: for a player new to the rating list until the completion of events with a total of 30 games.
* K = 20: for players who have always been rated under 2400.
* K = 10: for players with any published rating of at least 2400 and at least 30 games played in previous events. Thereafter it remains permanently at 10.
