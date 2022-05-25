fun main() {
    println("Tree:")
    val root = algorithms.unbalanced.tree.BinaryTree<Int>()
    root.insert(listOf(12, 10, 20, 5, 11, 15, 30, 14))

    root.print(order = root::bfs)
    root.remove(5)
    root.print(order = root::bfs)
    root.remove(10)
    root.print(order = root::bfs)
    root.remove(20)
    root.print(order = root::bfs)


    println("Array:")
    val arrayRoot = algorithms.unbalanced.array.BinaryTree<Int>()
    arrayRoot.insert(listOf(12, 10, 19, 5, 11, 18, 30, 16, 15, 17))

    println(arrayRoot.elements)
    arrayRoot.remove(5)
    println(arrayRoot.elements)
    arrayRoot.remove(10)
    println(arrayRoot.elements)
    arrayRoot.remove(19)
    println(arrayRoot.elements)
}
