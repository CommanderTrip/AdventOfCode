package day13

import java.io.File
import java.net.DatagramPacket
import java.text.FieldPosition
import kotlin.math.abs


/*
Day 13: Distress Signal

Part 1
Sample Solution: 13

Part 2
Sample Solution: N/A

 */
fun solution() {
    // Get the path to txt
    val pathToInput = Constants.getPath(13, true)



    // Build the grid and store it in memory
    File(pathToInput).forEachLine { packet ->
        decodePacket(packet)
    }

}

fun decodePacket(packet: String): Any? {
    val parse = packet.split("")
    println(parse)
    return null
}