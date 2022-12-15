package day06

import java.io.File
import java.lang.Error
import java.util.Queue

/*
Day 6: Tuning Trouble

Problem Statement: How many characters need to be processed before the first start-of-packet marker is detected?

Part 1
Sample Solution: 7 5 6 10 11

Part 2
Sample Solution: 19 23 23 29 26

 */
fun solution() {
    // Get the path to txt
    val pathToInput = Constants.getPath(6, false)

    File(pathToInput).forEachLine { line ->
        part1(line)
    }
}

/**
 * Determines the first 'start-of-packet' (SoP) marker.
 * SoP marker is the first time when the last 4 characters are all unique.
 * Uses a queue system.
 */
fun part1(dataStream: String): Int {
    var marker = mutableListOf<Char>()
    var iteration = 1

    val PART_1_SIZE = 4
    val PART_2_SIZE = 14 // Part 2 is the same as part 1 but just increases the size

    for (char in dataStream) {
        // Get the marker up to  before checks
        if (marker.size < PART_2_SIZE) {
            marker.add(char)
            print(marker)
        } else {
            marker = shiftQueue(char, marker) // Add char to the marker and remove the oldest one
            if(isStartOfPacket(marker)) {
                println(" $iteration")
                return iteration
            }
        }
        iteration++
    }

    throw Error("Start of Packet never found.")
}

/**
 * Compares the combinations of the marker for duplicate characters.
 * Only 6 comparisons:
 * 1 == 2, 1 == 3, 1 == 4
 * 2 == 3 ,2 == 4
 * 3 == 4
 *
 * Good thing this does not care about size for part 2 :)
 */
fun isStartOfPacket(marker: MutableList<Char>): Boolean {
    for (i in 0 until marker.size) {
        for (j in i+1 until marker.size){
            if (marker[i] == marker[j]) {
                return false
            }
        }
    }
    return true
}

/**
 * Dequeues the first character and enqueues the input.
 *
 * Good thing this does not care about size for part 2 :)
 */
fun shiftQueue(nextInput: Char, q: MutableList<Char>): MutableList<Char> {
    // Shift characters left
    for (i in 0 until q.size-1) {
        q[i] = q[i+1]
    }
    q[q.size-1] = nextInput

    return q
}
