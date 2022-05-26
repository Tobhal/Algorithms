package algorithms.util

data class Node<T> (
    val data: T,
) {
    var count = 0
    var left: Node<T>? = null
    var right: Node<T>? = null

}