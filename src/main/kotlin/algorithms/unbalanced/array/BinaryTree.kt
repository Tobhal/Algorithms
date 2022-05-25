package algorithms.unbalanced.array

import algorithms.interfaces.BinaryTree
import java.util.LinkedList
import kotlin.math.pow

class BinaryTree<T: Comparable<T>> : BinaryTree<T> {
    // TODO: Look into making elements its own data class, contain:
    //       leftChild, RightChild, parent, indexOut and nextIndexOut?
    private val elements = ArrayList<T?>(1)
    private var nodes = 1
    private var height = 0

    override fun insert(data: T) = insert(data, 0)

    override fun insert(data: List<T>) = data.forEach() {insert(it, 0)}

    private fun insert(data: List<T>, idx: Int) = data.forEach() {insert(it, idx)}

    private fun insert(data: T, idx: Int = 0) {
        if (elements.size == 0) {
            elements.add(data)
            return
        }

        var i = idx

        while (true) {
            if (i >= elements.size)
                this.increaseLevels()

            if (elements[i] == null) {
                elements[i] = data
                return
            }
            else if (elements[i]!! > data)
                i = leftChild(i)
            else if (elements[i]!! < data)
                i = rightChild(i)
        }
    }

    override fun remove(data: T) {
        var tmp = 0

        // Search for element
        while ((elements[tmp] != data && elements[tmp] != null))
            tmp = if (elements[tmp]!! > data)
                leftChild(tmp) // Move left
            else
                rightChild(tmp) // Mode right

        if (elements[tmp] != null) {
            // Case 1 - Delete leaf Node
            if (this.nextIndexOut(tmp)) {
                elements[tmp] = null
                return
            } else {
                if (elements[leftChild(tmp)] == null && elements[rightChild(tmp)] == null)
                    elements[tmp] = null
                // Case 2 - Delete node with one child
                else if (elements[leftChild(tmp)] == null || elements[rightChild(tmp)] == null)
                    if (elements[leftChild(tmp)] == null) {
                        val el = this.bfs(rightChild(tmp))
                        this.clear(tmp)
                        this.insert(el.toList(), tmp)
                    } else {
                        val el = this.bfs(leftChild(tmp))
                        this.clear(tmp)
                        this.insert(el.toList(), tmp)
                    }
                else {
                    // Case 3 - Delete Node with 2 children
                    var child = leftChild(tmp)
                    while (true) {
                        if ((rightChild(child)) > elements.size) break

                        if (elements[rightChild(child)] == null) break

                        child = rightChild(child)
                    }
                    val arr = this.bfs(leftChild(child))
                    this.clear(leftChild(child))
                    elements[tmp] = elements[child]
                    elements[child] = null
                    this.insert(arr, child)
                }
            }
        }
    }

    override fun contains(data: T): Boolean {
        var i = 0

        while (true) {
            if (elements[i] == data) return true

            if (nextIndexOut(i) || elements[i] == null) return false

            i = if (elements[i]!! > data)
               leftChild(i)
            else
                rightChild(i)
        }
    }

    private fun increaseLevels() {
        this.nodes = 2.0.pow(++this.height + 1).toInt() - 1

        for (i in elements.size until nodes)
            elements.add(i, null)
    }

    /*
        Traversal
     */
    fun bfs(idx: Int = 0): ArrayList<T> {
        if (idx > elements.size) return arrayListOf()

        val arr = ArrayList<T>()
        val queue = LinkedList<Int>()   // Next index to use

        elements[idx]?.let { arr.add(it) }

        if (nextIndexOut(idx)) return arr

        if (elements[leftChild(idx)] != null)
            queue.add(leftChild(idx))
        if (elements[rightChild(idx)] != null)
            queue.add(rightChild(idx))

        var current = idx

        while (!queue.isEmpty()) {
            current = queue.remove()
            elements[current]?.let { arr.add(it) }

            if (indexOut(current))
                continue
            if (elements[leftChild(current)] != null)
                queue.add(leftChild(current))
            if (elements[leftChild(current)] != null)
                queue.add(leftChild(current))
        }

        return arr
    }

    /*
        Util
     */
    private fun clear(idx: Int = 0) {
        if (idx > elements.size) return

        val queue = LinkedList<Int>()
        var current = idx

        elements[idx] = null

        if (nextIndexOut(idx)) return

        if (elements[leftChild(idx)] != null)
            queue.add(leftChild(idx))
        if (elements[rightChild(idx)] != null)
            queue.add(rightChild(idx))

        while (!queue.isEmpty()) {
            current = queue.remove()
            elements[current] = null

            if (indexOut(current))
                continue
            if (elements[leftChild(current)] != null)
                queue.add(leftChild(current))
            if (elements[rightChild(current)] != null)
                queue.add(rightChild(current))
        }
    }

    private fun indexOut(idx: Int, size: Int = elements.size): Boolean = idx > size || (leftChild(idx)) > size || (rightChild(idx)) > size

    private fun nextIndexOut(idx: Int, size: Int = elements.size): Boolean = (leftChild(idx)) > size || (rightChild(idx)) > size

    private fun leftChild(idx: Int): Int = leftChild(idx)

    private fun rightChild(idx: Int): Int = rightChild(idx)

    private fun parent(idx: Int): Int = (idx - 1) / 2
}