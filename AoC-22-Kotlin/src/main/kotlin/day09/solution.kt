package day09

import java.io.File
import java.text.FieldPosition
import kotlin.math.abs


/*
Day 9: Rope Bridge

How many positions does the tail of the rope visit at least once?

Part 1
Sample Solution: 13

Part 2
Sample Solution:

 */
fun solution() {
    // Get the path to txt
    val pathToInput = Constants.getPath(9, false)

    // X, Y position pair
    var headPos = Pair(0,0)
    var tailPos = Pair(0,0)

    val tailHistory = mutableListOf(tailPos)

    // Build the grid and store it in memory
    File(pathToInput).forEachLine { headMove ->
        val instruction = headMove.split(" ")
        val direction = instruction[0]
        val moveAmount = instruction[1].toInt()

        for (i in 1..moveAmount) {
            headPos = moveHead(direction, headPos)
            tailPos = moveTail(headPos, tailPos)

            if (!tailHistory.contains(tailPos)) tailHistory.add(tailPos)
        }
    }


    // Part 1
    println(tailHistory.size)
}

fun moveHead(direction: String, currentPosition: Pair<Int, Int>): Pair<Int, Int> {
    val end = "Head"
    when (direction) {
        "U" -> {
            printMove(end, "Up")
            return currentPosition.copy(second = currentPosition.second + 1)
        }
        "D" -> {
            printMove(end, "Down")
            return currentPosition.copy(second = currentPosition.second - 1)
        }
        "L" -> {
            printMove(end, "Left")
            return currentPosition.copy(first = currentPosition.first - 1)
        }
        "R" -> {
            printMove(end, "Right")
            return currentPosition.copy(first = currentPosition.first + 1)
        }
    }
    throw Error("Head move was unrecognized.")
}

fun moveTail(headPosition: Pair<Int, Int>, tailPosition: Pair<Int, Int>): Pair<Int, Int> {
    // Check if head is too far away
    val verticalDelta = headPosition.second - tailPosition.second
    val horizontalDelta = headPosition.first - tailPosition.first
    val end = "Tail"

    println("Deltas - V:$verticalDelta, H:$horizontalDelta")

    // Move UP right behind head
    if (verticalDelta > 1)  {
        printMove(end, "Up")
        return tailPosition.copy(headPosition.first, headPosition.second-1)
    }

    // Move DOWN to Head
    if (verticalDelta < -1) {
        printMove(end, "Down")
        return tailPosition.copy(headPosition.first, headPosition.second+1)
    }

    // Move LEFT right behind head
    if (horizontalDelta < -1)  {
        printMove(end, "Left")
        return tailPosition.copy(headPosition.first+1, headPosition.second)
    }

    // Move RIGHT to Head
    if (horizontalDelta > 1) {
        printMove(end, "Right")
        return tailPosition.copy(headPosition.first-1, headPosition.second)
    }

    return tailPosition
}

fun printMove(ropeEnd: String, direction: String) {
    println("$ropeEnd moved $direction")
}