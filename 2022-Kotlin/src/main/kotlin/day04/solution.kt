package day04

import java.io.File

/*
Day 4: Camp Cleanup

Part 1
Problem Statement: In how many assignment pairs does one range fully contain the other?
Sample Solution: 2 (group 4 and 5)

Part 2
Problem Statement: In how many assignment pairs does one range fully contain the other?
Sample Solution:

 */
fun solution() {
    // Get the path to txt
    val pathToInput = Constants.getPath(4, false)

    var sumOfOverlaps = 0


    File(pathToInput).forEachLine { elfPair ->
        //sumOfOverlaps += part1(elfPair)
        sumOfOverlaps += part2(elfPair)
    }

    println(sumOfOverlaps)
}

/**
 * Get the start and ends of each elf
 */
fun getStartAndEnds(rawPair: String): List<Int> {
    val pair = rawPair.split(",")

    // Elf 1
    val elf1Start = pair[0].split("-")[0].toInt()
    val elf1End = pair[0].split("-")[1].toInt()

    // Elf 2
    val elf2Start = pair[1].split("-")[0].toInt()
    val elf2End = pair[1].split("-")[1].toInt()

    return listOf(elf1Start, elf1End, elf2Start, elf2End)
}

/**
 * 1. Get the Starting and End index for each elf
 * 2. Determine if either elf fully contain the other
 * 3. return 1 for yes, 0 for no
 */
fun part1(rawPair: String): Int {
    val (elf1Start, elf1End, elf2Start, elf2End) = getStartAndEnds(rawPair)

    // Guaranteed to fully overlap if the starts or ends are the same
    if (elf1Start == elf2Start || elf1End == elf2End) return 1

    // find if elf 1 fully overlaps
    if (elf1Start < elf2Start && elf1End > elf2End) return 1

    // find if elf 2 fully overlaps
    if (elf2Start < elf1Start && elf2End > elf1End) return 1

    // No full overlap
    return 0
}

/**
 * Determine if there is any overlap at all.
 */
fun part2(rawPair: String): Int {
    val (elf1Start, elf1End, elf2Start, elf2End) = getStartAndEnds(rawPair)

    // Only guaranteed no overlap in two conditions
    if (elf1End < elf2Start ||
        elf2End < elf1Start) return 0

    return 1
}