import java.io.File

/*
Day 2: Rucksack Reorganization

Each line represents 2 containers.
No character (case-sensitive) can repeat in the containers but 1 does for each.
Lower case maps to 1-26.
Upper case maps to 26 - 52.

Part 1
Problem Statement: What is the sum of the priorities of those item types?
Sample Solution: 157 (16 + 38 + 42 + 22 + 20 + 19)

Part 2
Problem Statement:
Sample Solution: 12 (4 + 1 + 7)

 */

fun main() {
    // Get the path to txt
    //val pathToInput = System.getProperty("user.dir").plus("/src/sample.txt")
    val pathToInput = System.getProperty("user.dir").plus("/src/input.txt")

    File(pathToInput).forEachLine { containers ->

    }


}