package algorithms.util

open class Node<T> (
    val data: T,
) {
    var count = 0
    var left: Node<T>? = null
    var right: Node<T>? = null

    fun isFull(): Boolean = (left == null) && (right == null)
}