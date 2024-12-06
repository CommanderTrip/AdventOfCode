package day02

import Constants
import java.io.File

/*
Day 2: Rock Paper Scissors

Rock    - A X (1 point)
Paper   - B Y (2 points)
Scissors- C Z (3 points)

Win 6 points
Draw 3 points
Loss 0 points

Part 1
Problem Statement: What would your total score be if everything goes exactly according to your strategy guide?
Sample Solution: 15 (8 + 1 + 6)

Part 2
Problem Statement: what would your total score be if everything goes exactly according to your strategy guide?
Sample Solution: 12 (4 + 1 + 7)

 */

/**
 * Determine the outcome of each 'round' (W, D, L) and what you threw (R, P, S).
 * Sum the score for the round and add it to a running score.
 */
fun solution() {
    // Get the path to txt
    val pathToInput = Constants.getPath(2, false)

    var totalScore = 0

    // each line is a string with 3 characters: opponent, space, you
    File(pathToInput).forEachLine { round ->
        totalScore += resolveRoundPart2(round[0], round[2])
    }

    println(totalScore)
}

/**
 * This will resolve the round if the input is formatted as "{opponent} {you}" e.g. "A Z"
 */
fun resolveRoundPart1(opponentChoice: Char, yourChoice: Char): Int {
    var score = 0

    // Could I make this into a 2D matrix and map results?
    when(yourChoice) {
        'X' -> { // Rock
            score += 1 // Add the value of the choice
            when (opponentChoice) { // Add the value of the conflict resolution
                'A' -> score += 3 // Rock = Draw
                'B' -> score += 0 // Paper = Loss
                'C' -> score += 6 // Scissors = Win
            }
        }

        'Y' -> { // Paper
            score += 2 // Add the value of the choice
            when (opponentChoice) { // Add the value of the conflict resolution
                'A' -> score += 6 // Rock = Win
                'B' -> score += 3 // Paper = Draw
                'C' -> score += 0 // Scissors = Loss
            }
        }

        'Z' -> { // Scissors
            score += 3 // Add the value of the choice
            when (opponentChoice) { // Add the value of the conflict resolution
                'A' -> score += 0 // Rock = Loss
                'B' -> score += 6 // Paper = Win
                'C' -> score += 3 // Scissors = Draw
            }
        }
    }
    return score
}

/**
 * This will resolve the round if the input is formatted as "{opponent} {round result}" e.g. "A Z"
 */
fun resolveRoundPart2(opponentChoice: Char, roundResult: Char): Int {
    var score = 0

    when(roundResult) {
        'X' -> { // Lose
            score += 0 // Add the value of the conflict resolution
            when (opponentChoice) { // Add the value of the choice
                'A' -> score += 3 // Lose && Rock = Scissors
                'B' -> score += 1 // Lose && Paper = Rock
                'C' -> score += 2 // Lose && Scissors = Paper
            }
        }

        'Y' -> { // Draw
            score += 3 // Add the value of the conflict resolution
            when (opponentChoice) { // Add the value of the choice
                'A' -> score += 1 // Draw && Rock = Rock
                'B' -> score += 2 // Draw && Paper = Paper
                'C' -> score += 3 // Draw && Scissors = Scissors
            }
        }

        'Z' -> { // Win
            score += 6 // Add the value of the conflict resolution
            when (opponentChoice) { // Add the value of the choice
                'A' -> score += 2 // Win && Rock = Paper
                'B' -> score += 3 // Win && Paper = Scissors
                'C' -> score += 1 // Win && Scissors = Rock
            }
        }
    }
    return score
}
