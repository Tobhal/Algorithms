import algorithms.interfaces.BinaryTree

fun main() {
    run {
        val root = algorithms.unbalanced.array.BinaryTree<Int>()
        for (i in 10 until 20)
            root.insert(i)

        for (i in 9 downTo 0)
            root.insert(i)

        for (i in 0 until 10)
            root.remove(i)

        for (i in 19 downTo 10)
            root.remove(i)

        println(root.elements.size)
        root.fitSize()
        println(root.elements.size)
        root.insert(10)
        println(root.elements.size)
    }

    println("Tree: ")
    testInsertion(algorithms.unbalanced.tree.BinaryTree())
    testRemoval(algorithms.unbalanced.tree.BinaryTree())

    println("\nArray: ")
    testInsertion(algorithms.unbalanced.array.BinaryTree())
    testRemoval(algorithms.unbalanced.array.BinaryTree())
}

fun testInsertion(root: BinaryTree<Int>) {
    root.insert(listOf(8,3,1,6,4,7,10,14,13,13))

    println("Inserting:")
    println("1 3 4 6 7 8 10 13 14")
    root.println()
}

fun testRemoval(root: BinaryTree<Int>) {
    println("Removal: ")
    var checkString = ""
    root.remove(1)

    println("Basic delete functionality:")
    for (i in 10 until 20) {
        root.insert(i)
        checkString += "$i "
    }

    for (i in 9 downTo 0) {
        root.insert(i)
        checkString = "$i $checkString"
    }

    for (i in 0 until 10) {
        root.remove(i)
        checkString = checkString.replaceFirst("$i ", "")
        println(checkString)
        root.println()
    }

    for (i in 19 downTo 10) {
        root.remove(i)
        checkString = checkString.replaceFirst("$i ", "")
        println(checkString)
        root.println()
    }

    println("\nEdge cases")
    println("Deleting leaf:")

    root.insert(arrayListOf(10, 8, 9, 6, 7))
    root.remove(6)
    println("7 8 9 10")
    root.println()

    println("\nDeleting internal node:")

    root.insert(6)
    root.remove(7)
    println("6 8 9 10")
    root.println()

    println("\nDeleting an internal node and pushing new node up:")

    for (i in root.preOrder())
        root.remove(i)

    root.insert(arrayListOf(10, 8, 9, 7))
    root.remove(8)
    println("7 9 10")
    root.println()

    println("\nDeleting an internal node and progressing down left subtree to rightmost:")

    for (i in root.preOrder())
        root.remove(i)

    root.insert(arrayListOf(10, 8, 9, 6, 7))
    root.remove(8)
    println("6 7 9 10")
    root.println()
}