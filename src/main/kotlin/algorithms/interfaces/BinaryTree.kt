package algorithms.interfaces

interface BinaryTree<T> {
    /*
        TODO: Add index parameter?
        TODO: Add traversing methods here
            handle Node<T> and ArrayList<T>.
            So they can be used to convert from one type to another
     */
    fun insert(data: T)
    fun insert(data: List<T>)
    fun remove(data: T)
    fun contains(data: T): Boolean
}