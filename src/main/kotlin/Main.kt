import algorithms.unbalanced.tree.BTree

fun main() {
    val root = BTree(10)
    root.add(5)
    root.add(15)
    root.add(1)


    root.print(root::dfs)
}
