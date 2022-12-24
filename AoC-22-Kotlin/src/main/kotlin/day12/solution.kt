package day12

import java.io.File
import java.util.LinkedList
import java.util.Queue


/*
Day 12: Hill Climbing Algorithm

What is the fewest steps required to move from your current position to the location that should get the best signal?

Part 1
Sample Solution: 31

Part 2
Sample Solution: N/A

 */
fun solution() {
    // Get the path to txt
    val pathToInput = Constants.getPath(12, false)

    val map = mutableListOf<List<Int>>()
    val startPair = mutableListOf<Int>() // row , col
    val endPair = mutableListOf(-1,-1) // row , col



    // Build the grid and store it in memory
    var rowIndex = 0
    File(pathToInput).forEachLine { level ->
        var colIndex = -1
        val row = level.map {
            colIndex++
            when (it) {
                'S' -> {
                    startPair.add(rowIndex)
                    startPair.add(colIndex) 
                    'a'.code
                }
                'E' -> {
                    endPair[0] = rowIndex
                    endPair[1] = colIndex
                    'z'.code
                }
                else -> {
                    if (it.code == 'a'.code) {
                        startPair.add(rowIndex)
                        startPair.add(colIndex)
                    }
                    it.code
                }
            }
        }
        map.add(row)
        rowIndex++
    }

    val adjList = buildAdjacencyList(map)
    val startIndices = mutableListOf<Int>()
    for (i in 0 .. startPair.size-2 step 2) {
        startIndices.add(map[0].size * startPair[i] + startPair[i+1])
    }
    val endIndex = map[0].size * endPair[0] + endPair[1]

    // Part 2
    val distances = mutableListOf<Int>()
    for (startIndex in startIndices) {
        distances.add(breadthFirstSearch(adjList, startIndex, endIndex))
    }
    distances.sort()
    println(distances[0])

}

fun breadthFirstSearch(adjacencyList: MutableList<LinkedList<Int>>, source: Int, end: Int): Int {
    val vertexColor = mutableListOf<String>()   // Used to track the progress of vertex status (WHITE, GRAY, BLACK)
    val vertexDistanceSoFar = mutableListOf<Int>()  // Distance from Source
    val vertexPredecessor = mutableListOf<Int>()    // Previous vertex

    // init vertex data tracking
    for (vertex in 0 until adjacencyList.size){
        vertexColor.add("WHITE")
        vertexDistanceSoFar.add(Int.MAX_VALUE) // acting as infinity
        vertexPredecessor.add(-1) // Invalid predecessor
    }

    // Init the source vertex
    vertexColor[source] = "GRAY"
    vertexDistanceSoFar[source] = 0
    vertexPredecessor[source] = -1

    // Enqueue the source vertex
    val queue = mutableListOf(source)

    // BFS
    while (queue.isNotEmpty()) {
        val vertex = queue.removeFirst()
        for (adjacentVertexIndex in 0 until adjacencyList[vertex].size){
            val adjacentVertex = adjacencyList[vertex][adjacentVertexIndex]
            if (vertexColor[adjacentVertex] == "WHITE") {
                vertexColor[adjacentVertex] = "GRAY"
                vertexDistanceSoFar[adjacentVertex] = vertexDistanceSoFar[vertex] + 1
                vertexPredecessor[adjacentVertex] = vertex
                queue.add(adjacentVertex)
            }
        }
        vertexColor[vertex] = "BLACK"
    }

    return vertexDistanceSoFar[end] // Part 1
}

fun buildAdjacencyList(map: MutableList<List<Int>>): MutableList<LinkedList<Int>> {
    val adjacencyList = mutableListOf<LinkedList<Int>>()

    for (row in 0 until map.size) {
        for (col in 0 until map[row].size){
            val nodeConnections = LinkedList<Int>() // Build the adjacency linked list for this node
            val currentElevation = map[row][col]
            val currentIndex = map[row].size * row + col

            // Add the node we are investigating as the first in the list
            //nodeConnections.add(currentIndex)

            // Check for up, down, left, right connections
            if (row != 0 && currentElevation >= map[row-1][col]-1) nodeConnections.add(currentIndex - map[row].size)
            if (row != map.size-1 && currentElevation >= map[row+1][col]-1) nodeConnections.add(currentIndex + map[row].size)
            if (col != 0 && currentElevation >= map[row][col-1]-1) nodeConnections.add(currentIndex - 1)
            if (col != map[row].size-1 && currentElevation >= map[row][col+1]-1) nodeConnections.add(currentIndex + 1)

            // Add this node to the list
            adjacencyList.add(nodeConnections)
        }
    }
    return adjacencyList
}

