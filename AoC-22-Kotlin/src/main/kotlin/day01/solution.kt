package day01

import Constants
import java.io.File

/*
Day 1: Calorie Counting

Part 1
Problem Statement: Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?

Sample.txt Solution: 24000 (fourth elf)

Initial Approach
    Read in lines from input file
    Sum values in the file until blank like
    Determine if this is the most we've seen, if so save it. Otherwise, move on.

Part 2
Problem Statement: Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?

Sample.txt Solution: 45000 (fourth, third, fifth elf)

Initial Approach
    Track the greatest calories in an array that goes from smallest to largest (to reduce unneeded comparisons)
    Shift the array as needed

*/

fun solution() {
    // Get the path to txt
    val path = Constants.getPath(1, true)

    val topCalories = mutableListOf(0,0,0) // Ascending order to allow short circuit
    var runningSum = 0

    File(path).forEachLine { line ->
        if (line.isNotBlank()) {
            runningSum += line.toInt()
        } else {
            checkForNewTopCalorie(runningSum, topCalories)
            runningSum = 0
        }
    }

    var finalSum = 0
    for (cal in topCalories) {
        finalSum += cal
    }

    println("$topCalories = $finalSum")
}

/**
 * Check to see if the current count is among the top calories. Shift the List as needed.
 * Simple enough to hard code the possibilities.
 * @param currentCount The current calorie count in question
 * @param topCalories The list of the highest calories
 */
fun checkForNewTopCalorie(currentCount: Int, topCalories: MutableList<Int>) {
    // Check to see if the current count even belongs here
    if (topCalories[0] > currentCount) return

    // Check to see if it is the 3rd largest
    if (topCalories[1] > currentCount) {
        topCalories[0] = currentCount

        // Check for second largest (shift down)
    } else if (topCalories[2] > currentCount) {
        topCalories[0] = topCalories[1]
        topCalories[1] = currentCount

        // The current count is the biggest (shift down)
    } else {
        topCalories[0] = topCalories[1]
        topCalories[1] = topCalories[2]
        topCalories[2] = currentCount
    }
}