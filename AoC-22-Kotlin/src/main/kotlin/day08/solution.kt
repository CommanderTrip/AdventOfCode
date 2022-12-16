package day08

import java.io.File


/*
Day 8: Treetop Tree House

how many trees are visible from outside the grid?

Part 1
Sample Solution: 21 (16 edges + 5 interior)

Part 2
Sample Solution:

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

    print(part1(grid))
}

fun part1(treeMap: MutableList<List<Int>>): Int {
    var numOfVisibleTrees = 0

    for (i in 0 until treeMap.size) {
        for (j in 0 until treeMap[i].size) {
            // If we are on the edge
            if (i == 0 || i == treeMap.size-1 ||
                j == 0 || j == treeMap[i].size-1) {
                numOfVisibleTrees++
            } else {
                // Determine if interior tree is visible
                if (checkRight(i,j) && checkLeft(i,j) && checkUp(i,j) && checkDown(i,j)) numOfVisibleTrees++
            }
        }
    }

    return 0
}
 fun checkRight(x: Int, y:Int): Boolean {

 }

fun checkLeft(x: Int, y:Int): Boolean {

}

fun checkUp(x: Int, y:Int): Boolean {

}

fun checkDown(x: Int, y:Int): Boolean {

}