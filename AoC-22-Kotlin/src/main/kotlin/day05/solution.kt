package day05

import java.io.File
import java.util.Stack

/*
Day 5: Supply Stacks

Seems like a simple Stack data structure.

Problem Statement: After the rearrangement procedure completes, what crate ends up on top of each stack?

Part 1
Sample Solution: CMZ

Part 2
Sample Solution:

 */
fun solution() {
    // Get the path to txt
    val pathToInput = Constants.getPath(5, true)

    // I am not making a dynamic stack determiner. I am hard coding it for the solution
    val containerYard = null;


    File(pathToInput).forEachLine { instruction ->
       if (instruction.isNotBlank() && instruction[1] == '1') println(instruction[instruction.length - 1])
    }

}

/**
 * Dynamically builds the stacks of containers
 * 1. Determine how many stacks we need
 * 2. fill each stack
 */
fun buildContainerStacks(filePath: String): List<Stack<Char>> {
    val containerYard = mutableListOf<Stack<Char>>()
    var numOfContainers = 0

    File(filePath).forEachLine { instruction ->
        if (instruction[1] == '1') {
            numOfContainers = instruction[instruction.length - 1].digitToInt()
            return@forEachLine
        }
    }
}