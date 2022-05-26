fun main() {
    val numbers = listOf(20, 10, 30, 5, 15, 40, 35, 34, 36)

    println("Tree:")
    val root = algorithms.unbalanced.tree.BinaryTree<Int>()
    root.insert(numbers)

    root.print(root::bfs)
    println("Nodes       : ${root.numNodes()}")
    println("Leaves      : ${root.numLeaves()}")
    println("Two children: ${root.numTwoChildren()}")
    println("Levels      : ${root.numLevels()}")

    println("\nArray:")
    val arrayRoot = algorithms.unbalanced.array.BinaryTree<Int>()
    arrayRoot.insert(numbers)

    arrayRoot.print(arrayRoot::bfs)
    println("Nodes       : ${arrayRoot.numNodes()}")
    println("Leaves      : ${arrayRoot.numLeaves()}")
    println("Two children: ${arrayRoot.numTwoChildren()}")
    println("Levels      : ${arrayRoot.numLevels()}")
}
