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
    val pathToInput = Constants.getPath(8, true)

    val grid = mutableListOf<List<Int>>()

    // Build the grid and store it in memory
    File(pathToInput).forEachLine { row ->
        val rowSplit = row.map {
            it.toString().toInt()
        }
        grid.add(rowSplit)
    }

    part1(grid)
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

/**
 * I would like to see if there is an improved way to determine this. Otherwise, for small input, I am going to iterate
 * over each layer.
 */
 fun checkRight(row: Int, col:Int, map: MutableList<List<Int>>): Boolean {
    for (i in col+1 until map[row].size) {
        if (map[row][i] >= map[row][col]) {
            //println("A tree to the right is bigger.")
            return false
        }
    }
    return true
 }

fun checkLeft(row: Int, col:Int, map: MutableList<List<Int>>): Boolean {
    for (i in 0 until col) {
        if (map[row][i] >= map[row][col]) {
            //println("A tree to the left is bigger.")
            return false
        }
    }
    return true
}

fun checkUp(row: Int, col:Int, map: MutableList<List<Int>>): Boolean {
    for (i in 0 until row) {
        if (map[i][col] >= map[row][col]) {
            //println("A tree above is bigger.")
            return false
        }
    }
    return true
}

fun checkDown(row: Int, col:Int, map: MutableList<List<Int>>): Boolean {
    for (i in row+1 until map.size) {
        if (map[i][col] >= map[row][col]) {
            //println("A tree below is bigger.")
            return false
        }
    }
    return true
}