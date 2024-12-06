package day08

import java.io.File


/*
Day 8: Treetop Tree House

how many trees are visible from outside the grid?

Part 1
Sample Solution: 21 (16 edges + 5 interior)

Part 2
What is the highest scenic score possible for any tree?
Sample Solution: 8

 */
fun solution() {
    // Get the path to txt
    val pathToInput = Constants.getPath(8, false)

    val grid = mutableListOf<List<Int>>()

    // Build the grid and store it in memory
    File(pathToInput).forEachLine { row ->
        val rowSplit = row.map {
            it.toString().toInt()
        }
        grid.add(rowSplit)
    }

    part2(grid)
}

fun part1(treeMap: MutableList<List<Int>>) {
    var numOfVisibleTrees = 0

    for (i in 0 until treeMap.size) {
        for (j in 0 until treeMap[i].size) {
            // If we are on the edge
            if (i == 0 || i == treeMap.size-1 ||
                j == 0 || j == treeMap[i].size-1) {
                numOfVisibleTrees++
            } else {
                // Determine if interior tree is visible
                // Only needs to be visible from one side to be considered 'Visible'
                if (checkRight(i,j, treeMap) || checkLeft(i,j, treeMap) || checkUp(i,j, treeMap) || checkDown(i,j, treeMap)) {
                    println("Interior Tree ${treeMap[i][j]} @ [$j][$i] is visible")
                    numOfVisibleTrees++
                }
            }
        }
    }

    println("The number of visible trees is $numOfVisibleTrees")
}

fun part2(treeMap: MutableList<List<Int>>) {
    var highestScenicScore = 0

    for (i in 0 until treeMap.size) { // Row
        for (j in 0 until treeMap[i].size) { // Col
            // Look in all directions for the number of visible trees
            // Start at the tree in question and iterate left, right, up, and down
            // I do not want to create more functions for part 2, so I am leaving it all here; hope nesting isn't bad lol

            // Edge trees have a scenic score of 0
            if (i == 0 || i == treeMap.size-1 ||
                j == 0 || j == treeMap[i].size-1) {
                continue
            }

            var runningScenicScore = 1

            var numOfTreesSeen = 1
            for (k in j-1 downTo 0) { // Look Left ; same row, diff col
                // We hit a tree blocking view || end of the line
                if (treeMap[i][k] >= treeMap[i][j] || k == 0) {
                    runningScenicScore *= numOfTreesSeen
                    break
                }
                numOfTreesSeen++
            }

            numOfTreesSeen = 1
            for (k in j+1 until treeMap[i].size) { // Look Right ; same row, diff col
                // We hit a tree blocking view || end of the line
                if (treeMap[i][k] >= treeMap[i][j] || k == treeMap.size-1) {
                    runningScenicScore *= numOfTreesSeen
                    break
                }
                numOfTreesSeen++
            }

            numOfTreesSeen = 1
            for (k in i-1 downTo  0) { // Look Up ; same col, diff row
                // We hit a tree blocking view || end of the line
                if (treeMap[k][j] >= treeMap[i][j] || k == 0) {
                    runningScenicScore *= numOfTreesSeen
                    break
                }
                numOfTreesSeen++
            }

            numOfTreesSeen = 1
            for (k in i+1 until treeMap.size) { // Look Down ; same col, diff row
                // We hit a tree blocking view || end of the line
                if (treeMap[k][j] >= treeMap[i][j] || k == treeMap.size-1) {
                    runningScenicScore *= numOfTreesSeen
                    break
                }
                numOfTreesSeen++
            }

            if (runningScenicScore > highestScenicScore) highestScenicScore = runningScenicScore
        }
    }

    println("The highest scenic score is $highestScenicScore")
}


/**
 * I would like to see if there is an improved way to determine this. Otherwise, for small input, I am going to iterate
 * over each layer.
 */
 fun checkRight(row: Int, col:Int, map: MutableList<List<Int>>): Boolean {
    for (i in col+1 until map[row].size) {
        if (map[row][i] >= map[row][col]) {
            return false
        }
    }
    return true
 }

fun checkLeft(row: Int, col:Int, map: MutableList<List<Int>>): Boolean {
    for (i in 0 until col) {
        if (map[row][i] >= map[row][col]) {
            return false
        }
    }
    return true
}

fun checkUp(row: Int, col:Int, map: MutableList<List<Int>>): Boolean {
    for (i in 0 until row) {
        if (map[i][col] >= map[row][col]) {
            return false
        }
    }
    return true
}

fun checkDown(row: Int, col:Int, map: MutableList<List<Int>>): Boolean {
    for (i in row+1 until map.size) {
        if (map[i][col] >= map[row][col]) {
            return false
        }
    }
    return true
}