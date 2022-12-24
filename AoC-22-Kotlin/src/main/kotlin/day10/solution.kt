package day10

import java.io.File
import java.text.FieldPosition
import kotlin.math.abs


/*
Day 10: Cathode-Ray Tube

20th, 60th, 100th, 140th, 180th, and 220th cycles.
What is the sum of these six signal strengths

Part 1
Sample Solution: 13140

Part 2
Sample Solution: N/A

 */
fun solution() {
    // Get the path to txt
    val pathToInput = Constants.getPath(10, false)

    var xRegisterHistory = mutableListOf(1)

    // Build the grid and store it in memory
    File(pathToInput).forEachLine { instruction ->
        val parse = instruction.split(" ")

        if (parse[0] == "addx") xRegisterHistory = addx(parse[1].toInt(), xRegisterHistory)
        else xRegisterHistory = noop(xRegisterHistory)
    }

    print(part2(xRegisterHistory))
}

fun part1(regHistory: MutableList<Int>): Int {
    return regHistory[19] * 20 +
            regHistory[59] * 60 +
            regHistory[99] * 100 +
            regHistory[139] * 140 +
            regHistory[179] * 180 +
            regHistory[219] * 220
}

fun part2(regHistory: MutableList<Int>) {
    var row = 0
    for (i in 0 until regHistory.size) {
        val delta = i - 40 * row - regHistory[i]
        if (delta < 2 && delta > -2) print("#")
        else print(".")

        if (i == 39 || i == 79 || i == 119 || i == 159 || i == 199 || i == 239) {
            row++
            println()
        }
    }
}

fun addx(value:Int, regHistory: MutableList<Int>): MutableList<Int> {
    // run first cycle
    val addHistory = noop(regHistory)

    // run second cycle
    addHistory.add(addHistory.last() + value)
    return addHistory
}

fun noop (regHistory: MutableList<Int>): MutableList<Int>{
    // Run one cycle and append the last value
    regHistory.add(regHistory.last())
    return regHistory
}