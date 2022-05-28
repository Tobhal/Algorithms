package algorithms.unbalanced.array

import algorithms.interfaces.BinaryTree
import java.util.Stack
import kotlin.math.pow

class BinaryTree<T: Comparable<T>> : BinaryTree<T> {
    var elements = ArrayList<T?>(1)
    private var nodes = 1
    private var height = 0

    override fun numNodes(): Int = numNodes(-1)

    private fun numNodes(idx: Int = 0): Int {
        if (indexOut(idx)) return 1

        var sum = 1
        val queue = ArrayDeque<Int>()   // Next index to use

        if (nextIndexOut(idx)) return sum
        addChildrenToQueue(idx, queue)

        var current = idx

        while (!queue.isEmpty()) {
            current = queue.removeFirst()
            sum++

            if (indexOut(current)) continue
            addChildrenToQueue(current, queue)
        }

        return sum
    }

    override fun numLeaves(): Int = numLeaves(0)

    private fun numLeaves(idx: Int = 0): Int {
        if (indexOut(idx)) return 1

        var sum = 0
        val queue = ArrayDeque<Int>()

        addChildrenToQueue(idx, queue)

        var current = idx

        while (!queue.isEmpty()) {
            current = queue.removeFirst()

            if (nextIndexOut(current)) {
                sum++
                continue
            }

            if (elements[leftChild(current)] == null && elements[rightChild(current)] == null) {
                sum++
                continue
            }
            addChildrenToQueue(current, queue)
        }

        return sum
    }

    override fun numTwoChildren(): Int = numTwoChildren(0)

    private fun numTwoChildren(idx: Int = 0): Int {
        if (indexOut(idx)) return 0

        var sum = if (elements[leftChild(idx)] != null && elements[rightChild(idx)] != null) 1 else 0

        val queue = ArrayDeque<Int>()

        addChildrenToQueue(idx, queue)

        var current = idx

        while (!queue.isEmpty()) {
            current = queue.removeFirst()

            if (indexOut(current)) continue

            if (elements[leftChild(current)] != null && elements[rightChild(current)] != null) {
                sum++
            }
            addChildrenToQueue(current, queue)
        }

        return sum
    }

    override fun numLevels(): Int = numLevels(0)

    private fun numLevels(idx: Int = 0): Int {
        if (indexOut(idx)) return 1

        var level = 2   // TODO: <--- possible that it is here a error is
        val queue = ArrayDeque<Int>()
        val nextQueue = ArrayDeque<Int>()
        addChildrenToQueue(idx, queue)

        var current = idx

        while (!queue.isEmpty()) {
            current = queue.removeFirst()

            if (indexOut(current)) continue

            if (queue.isEmpty() && !nextQueue.isEmpty()) {
                addChildrenToQueue(current, nextQueue)

                level++

                print("$level ")

                while (!nextQueue.isEmpty())
                    queue.add(nextQueue.removeFirst())

                continue
            }

            addChildrenToQueue(current, nextQueue)
        }

        return level
    }

    /*
    Add stuff
     */
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
            else if (elements[i] == data)
                return
        }
    }

    override fun remove(data: T) {
        this.remove(data, 0)
    }

    private fun remove(data: T, idx: Int = 0) {
        if (elements.size == 0) return

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

    private fun increaseLevels(amount: Int = 1) {
        this.nodes = 2.0.pow(++this.height + amount).toInt() - 1

        for (i in elements.size until nodes)
            elements.add(i, null)
    }

    fun fitSize() {
        val newRoot = BinaryTree<T>()
        newRoot.insert(this.bfs())
        elements = newRoot.elements
    }

    /*
    Traversal
     */
    override fun preOrder(): ArrayList<T> = preOrder(0)
    private fun preOrder(idx: Int = 0): ArrayList<T> {
        if (elements.isEmpty()) return ArrayList()

        val arr = ArrayList<T>()
        if (nextIndexOut(idx) && elements[idx] != null)
            return arrayListOf(elements[idx]!!)

        val stack = Stack<Int>()
        stack.add(idx)
        var i: Int

        while (!stack.isEmpty()) {
            i = stack.pop()

            if (elements[i] != null)
                arr.add(elements[i]!!)

            if (nextIndexOut(i)) continue

            if (elements[rightChild(i)] != null)
                stack.add(rightChild(i))
            if (elements[leftChild(i)] != null)
                stack.add(leftChild(i))
        }

        return arr
    }

    override fun inOrder(): ArrayList<T> = inOrder(0)
    private fun inOrder(idx: Int = 0): ArrayList<T> {
        if (elements.isEmpty() || elements[0] == null) return ArrayList()

        val arr = ArrayList<T>()
        if (nextIndexOut(idx) && elements[idx] != null)
            return arrayListOf(elements[idx]!!)

        val stack = Stack<Int>()
        stack.add(idx)
        var i: Int
        val done = BooleanArray(nodes) { false }
        val numNodes = this.numNodes(idx)

        while (!stack.isEmpty()) {
            i = stack.pop()

            if (nextIndexOut(i)) {
                done[i] = true
                arr.add(elements[i]!!)
                i = parent(i)
            }

            if (elements[leftChild(i)] != null && !done[leftChild(i)]) {
                stack.add(leftChild(i))
                continue
            }

            if (!done[i]) {
                arr.add(elements[i]!!)
                done[i] = true
            }

            if (elements[rightChild(i)] != null && !done[rightChild(i)]) {
                stack.add(rightChild(i))
                continue
            }

            stack.add(parent(i))

            if (numNodes <= arr.size)
                return arr
        }
        return arr
    }

    override fun postOrder(): ArrayList<T> = postOrder(0)
    private fun postOrder(idx: Int = 0): ArrayList<T> {
        if (elements.isEmpty() || elements[0] == null) return ArrayList()

        val arr = ArrayList<T>()
        if (nextIndexOut(idx) && elements[idx] != null)
            return arrayListOf(elements[idx]!!)

        val stack = Stack<Int>()
        stack.add(idx)
        var i: Int
        val done = BooleanArray(nodes) { false }
        val numNodes = this.numNodes(idx)

        while (!stack.isEmpty()) {
            i = stack.pop()

            if (nextIndexOut(i)) {
                done[i] = true
                arr.add(elements[i]!!)
                i = parent(i)
            }

            if (elements[leftChild(i)] != null && !done[leftChild(i)]) {
                stack.add(leftChild(i))
                continue
            }

            if (elements[rightChild(i)] != null && !done[rightChild(i)]) {
                stack.add(rightChild(i))
                continue
            }

            if (!done[i]) {
                arr.add(elements[i]!!)
                done[i] = true
            }

            stack.add(parent(i))

            if (numNodes <= arr.size)
                return arr
        }
        return arr
    }

    override fun bfs(): ArrayList<T> = bfs(0)
   private fun bfs(idx: Int = 0): ArrayList<T> {
       if (idx > elements.size || elements.isEmpty() || elements[0] == null) return arrayListOf()

       val arr = ArrayList<T>()
       val queue = ArrayDeque<Int>()   // Next index to use

       elements[idx]?.let { arr.add(it) }

       if (nextIndexOut(idx)) return arr
       addChildrenToQueue(idx, queue)

       var current = idx

       while (!queue.isEmpty()) {
           current = queue.removeFirst()
           elements[current]?.let { arr.add(it) }

           if (indexOut(current)) continue
           addChildrenToQueue(current, queue)
       }

        return arr
    }

    override fun dfs(): ArrayList<T> = dfs(0)
    private fun dfs(idx: Int = 0): ArrayList<T> {
        TODO("Not yet implemented")
    }

    /*
    Util
     */
    private fun clear(idx: Int = 0) {
        if (idx > elements.size) return

        val queue = ArrayDeque<Int>()
        var current = idx

        elements[idx] = null

        if (nextIndexOut(idx)) return
        addChildrenToQueue(idx, queue)

        while (!queue.isEmpty()) {
            current = queue.removeFirst()
            elements[current] = null

            if (indexOut(current)) continue
            addChildrenToQueue(current, queue)
        }
    }

    private fun indexOut(idx: Int, size: Int = elements.size, op: (Boolean, Boolean) -> Boolean = Boolean::or): Boolean =
        op(
            op(idx > size, (leftChild(idx) > size)),
            (rightChild(idx) > size)
        )

    private fun nextIndexOut(idx: Int, size: Int = elements.size, op: (Boolean, Boolean) -> Boolean = Boolean::or): Boolean =
        op (leftChild(idx) > size, rightChild(idx) > size)

    private fun leftChild(idx: Int): Int = 2 * idx + 1

    private fun rightChild(idx: Int): Int = 2 * idx + 2

    private fun parent(idx: Int): Int = (idx - 1) / 2

    private fun addChildrenToQueue(idx: Int, queue: ArrayDeque<Int>) {
        if (elements[leftChild(idx)] != null)
            queue.add(leftChild(idx))
        if (elements[rightChild(idx)] != null)
            queue.add(rightChild(idx))
    }

    override fun print() = print(this::inOrder)
    fun print(order: () -> ArrayList<T> = this::inOrder) {
        for (e in order())
            print("$e ")
    }

    override fun println() = println(this::inOrder)
    fun println(order: () -> ArrayList<T> = this::inOrder) {
        for (e in order())
            print("$e ")
        println("")
    }

    override fun toString(): String = inOrder().toString()

}