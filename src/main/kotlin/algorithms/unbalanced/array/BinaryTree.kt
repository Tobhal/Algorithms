package algorithms.unbalanced.array

import algorithms.interfaces.BinaryTree
import java.util.LinkedList
import kotlin.math.pow
import kotlin.reflect.jvm.internal.impl.name.StandardClassIds

class BinaryTree<T: Comparable<T>> : BinaryTree<T>{
    val elements = ArrayList<T?>(1)
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
                i = 2 * i + 1
            else if (elements[i]!! < data)
                i = 2 * i + 2
        }
    }

    override fun remove(data: T) {
        var tmp = 0

        // Search for element
        while ((elements[tmp] != data && elements[tmp] != null)) {
            if (elements[tmp]!! > data) {
                tmp = 2 * tmp + 1 // Move left
            } else {
                tmp = 2 * tmp + 2
            }
        }

        if (elements[tmp] != null) {
            // Case 1 - Delete leaf Node
            if ((2 * tmp + 1) > elements.size || (2 * tmp + 2) > elements.size) {
                elements[tmp] = null
                return
            } else {
                if (elements[2 * tmp + 1] == null && elements[2 * tmp + 2] == null)
                    elements[tmp] = null
                // Case 2 - Delete node with one child
                else if (elements[2 * tmp + 1] == null || elements[2 * tmp + 2] == null)
                    if (elements[2 * tmp + 1] == null) {
                        val el = this.bfs(2 * tmp + 2)
                        this.clear(tmp)
                        this.insert(el.toList(), tmp)
                    } else {
                        val el = this.bfs(2 * tmp + 1)
                        this.clear(tmp)
                        this.insert(el.toList(), tmp)
                    }
                else {
                    // Case 3 - Delete Node with 2 children
                    var child = 2 * tmp + 1
                    while (true) {
                        if ((2 * child + 2) > elements.size) break

                        if (elements[2 * child + 2] == null) break

                        child = 2 * child + 2
                    }
                    val arr = this.bfs(2 * child + 1)
                    this.clear(2 * child + 1)
                    elements[tmp] = elements[child]
                    elements[child] = null
                    this.insert(arr, child)
                }
            }
        }
    }

    override fun contains(data: T): Boolean {
        TODO("Not yet implemented")
    }

    private fun increaseLevels() {
        this.height++
        this.nodes = 2.0.pow(height + 1).toInt() - 1

        for (i in elements.size until nodes)
            elements.add(i, null)
    }

    /*
        Util
     */
    fun clear(idx: Int = 0) {
        if (idx > elements.size) return

        var i = idx
        val queue = LinkedList<Int>()
        var current = i

        elements[i] = null

        if ((2 * i + 1) > elements.size || (2 * i + 2) > elements.size) return

        if (elements[2 * i + 1] != null)
            queue.add(2 * i + 1)
        if (elements[2 * i + 2] != null)
            queue.add(2 * i + 2)

        while (!queue.isEmpty()) {
            current = queue.remove()
            elements[current] = null

            if (current > elements.size || (2 * current + 1) > elements.size || (2 * current + 2) > elements.size)
                continue
            if (elements[2 * current + 1] != null)
                queue.add(2 * current + 1)
            if (elements[2 * current + 2] != null)
                queue.add(2 * current + 2)
        }
    }

    /*
        Traversal
     */
    fun bfs(idx: Int = 0): ArrayList<T> {
        if (idx > elements.size) return arrayListOf()

        var i = idx
        val arr = ArrayList<T>()
        val queue = LinkedList<Int>()   // Next index to use

        elements[i]?.let { arr.add(it) }

        if ((2 * i + 1) > elements.size || (2 * i + 2) > elements.size) return arr

        if (elements[2 * i + 1] != null)
            queue.add(2 * i + 1)
        if (elements[2 * i + 2] != null)
            queue.add(2 * i + 2)

        var current = i

        while (!queue.isEmpty()) {
            current = queue.remove()
            elements[current]?.let { arr.add(it) }

            if (current > elements.size || (2 * current + 1) > elements.size || (2 * current + 2) > elements.size)
               continue
            if (elements[2 * current + 1] != null)
                queue.add(2 * current + 1)
            if (elements[2 * current + 2] != null)
                queue.add(2 * current + 2)
        }

        return arr
    }
}