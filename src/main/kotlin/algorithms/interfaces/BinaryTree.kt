package algorithms.interfaces

interface BinaryTree<T> {
    /*
        TODO: Add index parameter?
     */
    fun insert(data: T)
    fun insert(data: List<T>)
    fun remove(data: T)
    fun contains(data: T): Boolean
}