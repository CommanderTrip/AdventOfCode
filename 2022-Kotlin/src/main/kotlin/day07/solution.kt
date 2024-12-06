package day07

import day04.part1
import java.io.File
import java.lang.Error

/*
Day 7: No Space Left On Device

Took a very object-oriented approach

Find all the directories with a total size of at most 100,000
Problem Statement: What is the sum of the total sizes of those directories?

Part 1
Sample Solution: 95437 (94853 + 584)

Part 2
Sample Solution: 24933642 (d)

 */
fun solution() {
    // Get the path to txt
    val pathToInput = Constants.getPath(7, false)

    // Set up the root and keep a reference of it
    val root = Dir("Root")
    root.addChildDir(Dir("/"))
    var cwd = root

    File(pathToInput).forEachLine { outputLine ->
        cwd = buildDirStructure(cwd, outputLine)
    }

    // Print the full file structure
    //root.print("")

    val allDirs = mutableListOf<Int>()
    //println(part1(getAllDirSizes(root, allDirs)))
    println(part2(getAllDirSizes(root, allDirs)))
}

/**
 * Find all the directories with a total size of at most 100,000 and add them together
 */
fun part1(dirs: MutableList<Int>): Int {
    var sum = 0
    for (dir in dirs) {
        if (dir <= 100000) sum += dir
    }
    return sum
}

/**
 * Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update.
 * What is the total size of that directory?
 */
fun part2(dirs: MutableList<Int>): Int {
    val MAX_SPACE = 70000000
    val NEEDED_SPACE = 30000000
    val currentSpace = dirs.max()
    val unusedSpace = MAX_SPACE - currentSpace
    val spaceToDelete = NEEDED_SPACE - unusedSpace
    dirs.sort()

    for (dir in dirs) {
        if (dir > spaceToDelete) return dir
    }
    throw Error("None where big enough")
}

/**
 * Get list of all directories
 */
fun getAllDirSizes(cwd: Dir, allDirs: MutableList<Int>): MutableList<Int> {
    allDirs.add(cwd.getSize())
    for (childDir in cwd.childrenDirs) {
        getAllDirSizes(childDir, allDirs)
    }
    return allDirs
}

fun buildDirStructure(cwd: Dir, outputLine: String): Dir {
    // Parse commands
    if (outputLine[0] == '$') return parseMoveCmd(cwd, outputLine)

    val contents = outputLine.split(" ")
    if (contents[0] == "dir") {
        cwd.addChildDir(Dir(contents[1]))
    } else {
        cwd.addChildFiles(Pair(contents[0].toInt(), contents[1]))
    }
    return cwd
}

fun parseMoveCmd(cwd: Dir, cmd: String): Dir {
    val args = cmd.split(" ")
    if (args[1] == "ls") return cwd

    // Change directory command
    return if (args[2] == "..") {
        cwd.parent // move up
    } else {
        cwd.getChild(args[2]) // move to child
    }
}

class Dir(val name: String) {
    val childrenDirs = mutableListOf<Dir>()
    private val childrenFiles = mutableListOf<Pair<Int, String>>()
    lateinit var parent: Dir
    private var size = 0

    /**
     * Get the size of the dir by getting the size of it's files and each sub dir
     */
    fun getSize(): Int {
        var sum = 0
        for(file in childrenFiles) {
            sum += file.first
        }
        for (dir in childrenDirs) {
            sum += dir.getSize()
        }
        return sum
    }

    /**
     * Verify that we don't already have this child then add it
     */
    fun addChildDir(child: Dir) {
        if (childrenDirs.contains(child)) return
        child.parent = this
        childrenDirs.add(child)
    }

    /**
     * Verify that we don't already have this child then add it
     */
    fun addChildFiles(file: Pair<Int, String>) {
        if (childrenFiles.contains(file)) return
        this.size += file.first
        childrenFiles.add(file)
    }

    /**
     * Search our child Dirs for the right name and return it
     */
    fun getChild(name: String): Dir {
        for (child in childrenDirs) {
            if (child.name == name) return child
        }
        throw Error("Could not find child $name in ${this.name}")
    }

    /**
     * Print the contents of the Dir
     */
    fun print(padding: String) {
        for (dir in childrenDirs) {
            println("\t$padding/${dir.name} - ${dir.getSize()}")
            dir.print(padding.plus("  "))
        }
        for (pair in childrenFiles) {
            println("\t$padding${pair.second} ${pair.first}")
        }
    }
}
