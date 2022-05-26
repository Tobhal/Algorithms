fun main() {
    val numbers = listOf(20, 10, 30, 5, 15, 40, 35, 34, 36)

    println("Tree:")
    val root = algorithms.unbalanced.tree.BinaryTree<Int>()
    root.insert(numbers)

    root.print(root::inOrder)

    println("\nArray:")
    val arrayRoot = algorithms.unbalanced.array.BinaryTree<Int>()
    arrayRoot.insert(numbers)

    arrayRoot.print(arrayRoot::inOrder)
}
