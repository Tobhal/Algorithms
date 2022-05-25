fun main() {
    val numbers = listOf(12, 10, 19, 5, 11, 18, 30, 16, 15, 17)

    println("Tree:")
    val root = algorithms.unbalanced.tree.BinaryTree<Int>()
    root.insert(numbers)


    println("\nArray:")
    val arrayRoot = algorithms.unbalanced.array.BinaryTree<Int>()
    arrayRoot.insert(numbers)

}
