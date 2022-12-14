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

    val containerYard = mutableListOf<Stack<Char>>();

//    val stack1 = Stack<Char>()
//    stack1.push('Z')
//    stack1.push('N')
//
//    val stack2 = Stack<Char>()
//    stack2.push('M')
//    stack2.push('C')
//    stack2.push('D')
//
//    val stack3 = Stack<Char>()
//    stack3.push('P')
//
//    containerYard.add(stack1)
//    containerYard.add(stack2)
//    containerYard.add(stack3)

    File(pathToInput).forEachLine { instruction ->

       if (instruction.isNotBlank()) {
           // We are on a 'move' instruction
           // e.g. move 1 from 2 to 3
          if (instruction[0] == 'm') {
              val directions = instruction.split(" ")
              var quantity = directions[1].toInt()
              val fromStack = directions[3].toInt()
              val toStack = directions[5].toInt()

              // Get the cargo stacks
              val fromCargoStack = containerYard[fromStack - 1]
              val toCargoStack = containerYard[toStack - 1]

              // Loop for the quantity
              while(quantity > 0) {
                  val cargo = fromCargoStack.pop()
                  toCargoStack.push(cargo)
                  quantity--
              }

              // Save the changes to the stacks
              containerYard[fromStack - 1] = fromCargoStack
              containerYard[toStack - 1] = toCargoStack
          }
       } else {
           // This is diagram of the stacks
//           containerYard = buildContainerStacks(instruction)
       }
    }

    printStackTops(containerYard)

}

/**
 * Dynamically builds the stacks of containers
 */
fun buildContainerStacks(instruction: String, currentYard: MutableList<Char>): MutableList<Char> {

}

/**
 * Print the tops of each stack
 */
fun printStackTops(yard: List<Stack<Char>>) {
    for (stack: Stack<Char> in yard) {
        print("${stack.peek()} ")
    }
}