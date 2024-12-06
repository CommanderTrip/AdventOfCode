package day03

import java.io.File

/*
Day 3: Rucksack Reorganization

Each line represents 2 containers of equal size.
No character (case-sensitive) can repeat in the containers but 1 does for each.
Lower case maps to 1 - 26.
Upper case maps to 27 - 52.

Part 1
Problem Statement: What is the sum of the priorities of those item types?
Sample Solution: 157 (16 + 38 + 42 + 22 + 20 + 19)

Part 2
Problem Statement: What is the sum of the priorities of those item types?
Sample Solution: 70 (18 + 52)

 */

fun solution() {
    // Get the path to txt
    val pathToInput = Constants.getPath(3, false)

    var prioritySum = 0

    // Part 1
//    File(pathToInput).forEachLine { containers ->
//        prioritySum += part1(containers)
//    }

    // Part 2
    var groupNumber = 0
    var groupContainers = mutableListOf<String>()

    File(pathToInput).forEachLine { container ->
        if (groupNumber <= 2) {
            groupContainers.add(container)
            groupNumber++
        } else {
            prioritySum += part2(groupContainers)
            groupContainers = mutableListOf(container)
            groupNumber = 1
        }
    }


    println(prioritySum)
}

/**
 * 1. Find the common letter in each container. (Don't use ".contains"😒)
 * 2. Convert the letter to its priority.
 * 3. Return priority.
 */
fun part1(containers: String): Int {
    val container1 = containers.subSequence(0, containers.length / 2)
    val container2 = containers.subSequence(containers.length / 2, containers.length)

    for (item1: Char in container1) {
        for (item2: Char in container2) {
            if (item1 == item2) {
                return getPriority(item1)
            }
        }
    }
    throw Error("There was no common item")
}

/**
 * Takes a character, converts it to UTF-16 number, then gets the value where:
 * Lower case maps to 1 - 26.
 * Upper case maps to 27 - 52.
 */
fun getPriority(character: Char): Int {
    var priority = character.code

    // UTF-16 A = 65 (offset by 27)
    // UTF-16 a = 97 (offset by 1)
    priority -= if (character.isUpperCase()) 38
    else 96

    return priority
}

/**
 * 1. Find all common characters in the first two elves
 * 2. Find the common character in the third from the list from the first
 * 3. Convert and return priority
 */
fun part2(elfGroup: List<String>): Int {
    val commonItems = mutableListOf<Char>()

    // I'm not happy with all these loops :/
    for (item1: Char in elfGroup[0]) {
        for (item2: Char in elfGroup[1]) {
            if (item1 == item2) {
                commonItems.add(item1)
            }
        }
    }

    for (commonItem: Char in commonItems) {
        for (item: Char in elfGroup[2]) {
            if (commonItem == item) {
                return getPriority(commonItem)
            }
        }
    }

    throw Error("There was no common item")
}