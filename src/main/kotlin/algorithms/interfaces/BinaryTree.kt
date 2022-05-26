package algorithms.interfaces

interface BinaryTree<T> {
    fun numNodes(): Int
    fun numLeaves(): Int
    fun numTwoChildren(): Int
    fun numLevels(): Int

    fun insert(data: T)
    fun insert(data: List<T>)
    fun remove(data: T)
    fun contains(data: T): Boolean

    fun preOrder(): ArrayList<T>
    fun inOrder(): ArrayList<T>
    fun postOrder(): ArrayList<T>
    fun bfs(): ArrayList<T>
    fun dfs(): ArrayList<T>
}