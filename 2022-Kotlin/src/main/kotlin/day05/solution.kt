package day05

import java.io.File

/*
Day 5: Supply Stacks

Seems like a simple Stack data structure.

Problem Statement: After the rearrangement procedure completes, what crate ends up on top of each stack?

Part 1
Sample Solution: CMZ

Part 2
Sample Solution: MCD

 */
fun solution() {
    // Get the path to txt
    val pathToInput = Constants.getPath(5, false)

    val containerYard = buildContainerStacks(pathToInput)

    File(pathToInput).forEachLine { instruction ->

       if (instruction.isNotEmpty() && instruction[0] == 'm') {
           // We are on a 'move' instruction
           // e.g. move 1 from 2 to 3
           moveOperation(instruction, containerYard)
       }
    }

    printStackTops(containerYard)
}

/**
 * Dynamically builds the stacks of containers
 */
fun buildContainerStacks(path: String): MutableList<MutableList<Char>> {
    val containerYard = mutableListOf<MutableList<Char>>()

    File(path).forEachLine { line ->
        // Ignore Empty, Move, and the Numbers lines
        if (line.isNotEmpty() && line[0] != 'm' && line[1] != '1') {
            val numOfStacks = line.length / 4 + 1

            // expand the yard as needed
            while (containerYard.size < numOfStacks) {
                containerYard.add(mutableListOf())
            }

            // Add cargo to the stacks
            for (i in 0 until numOfStacks) {
                val cargo = line[4 * i + 1] // Each cargo is 4 long and offset 1 with zero index
                if (cargo != ' ') {
                    containerYard[i].add(0, cargo) // Always add to the front since we read top to bottom
                }
            }
        }
    }

    return containerYard
}

/**
 * Print the tops of each stack
 */
fun printStackTops(yard: MutableList<MutableList<Char>>) {
    for (stack: MutableList<Char> in yard) {
        print("${stack[stack.lastIndex]}")
    }
}

fun moveOperation(instruction: String, containerYard: MutableList<MutableList<Char>>) {
    val directions = instruction.split(" ")
    val quantity = directions[1].toInt()
    val fromStack = directions[3].toInt()
    val toStack = directions[5].toInt()

    // Get the cargo stacks
    val fromCargoStack = containerYard[fromStack - 1]
    val toCargoStack = containerYard[toStack - 1]

    //val (newFromCargoStack, newToCargoStack) = part1(quantity, fromCargoStack, toCargoStack)
    val (newFromCargoStack, newToCargoStack) = part2(quantity, fromCargoStack, toCargoStack)

    // Save the changes to the stacks
    containerYard[fromStack - 1] = newFromCargoStack
    containerYard[toStack - 1] = newToCargoStack
}

/**
 * The crane picks one crate up at a time
 */
fun part1(quantity: Int, from: MutableList<Char>, to: MutableList<Char>): Pair<MutableList<Char>,MutableList<Char>> {
    var i = quantity

    // Loop for the quantity
    while(i > 0) {
        val cargo = from.removeLast()
        to.add(to.size, cargo)
        i--
    }

    return Pair(to, from)
}

/**
 * The crane can move many crates from one stack at once
 */
fun part2(quantity: Int, from: MutableList<Char>, to: MutableList<Char>): Pair<MutableList<Char>,MutableList<Char>>{
    var i = quantity

    // Loop for the quantity
    while(i > 0) {
        val cargo = from.removeAt(from.size - quantity)
        to.add(to.size, cargo)
        i--
    }

    return Pair(to, from)
}