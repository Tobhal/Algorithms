package algorithms.unbalanced.tree

import algorithms.util.Node
import java.util.*
import kotlin.reflect.KFunction0

class BTree<T>(
    private val data: T,
    private var left: BTree<T>? = null,
    private var right: BTree<T>? = null
) where T: Comparable<T> {
    private var root: Node<T>? = null

    /*
    Status stuff
     */
    fun isFull(): Boolean = (left != null) && (right != null)

    fun numNodes(): Int = 1 + numNodes(left) + numNodes(right)

    fun numLeaves(): Int = numLeaves(left) + numLeaves(right)

    fun numTwoChildren(): Int {
        var add = 0

        if (left != null && right != null)
            add = 1

        return add + numTwoChildren(left) + numTwoChildren(right)
    }

    fun numLevels(): Int {
        val nLeft = numLevels(left)
        val nRight = numLevels(right)

        return if (nLeft > nRight) nLeft else nRight
    }

    private fun numNodes(root: BTree<T>?): Int {
        if (root == null)
            return 0

        return 1 + numNodes(root.left) + numNodes(root.right)
    }

    private fun numLeaves(root: BTree<T>?): Int {
        if (root == null)
            return 0

        if (root.left == null && root.right == null)
            return 1

        return numLeaves(root.left) + numLeaves(root.right)
    }

    private fun numTwoChildren(root: BTree<T>?): Int {
        if (root == null)
            return 0

        var add = 0

        if (root.left != null && root.right != null)
            add = 1

        return add + numTwoChildren(root.left) + numTwoChildren(root.right)
    }

    private fun numLevels(root: BTree<T>?): Int {
        if (root == null)
            return 0

        val nLeft = numLevels(root.left)
        val nRight = numLevels(root.right)

        return 1 + if (nLeft > nRight) nLeft else nRight
    }

    /*
    Add stuff
     */
    // TODO: Fix this
    fun add(data: T) {
        if (this.data == data)
            return

        if (left == null && this.data > data) {
            left = BTree(data)
            return
        }

        if (right == null && this.data < data) {
            right = BTree(data)
            return
        }
    }

    fun addChild(data: T) {
        if (contains(data))
            return

        if (left == null)
            left = BTree(data)
        else if (right == null)
            right = BTree(data)
        else if (!left!!.isFull() && !right!!.isFull() || left!!.numNodes() > right!!.numNodes())
            left!!.addChild(data)
        else if (left!!.isFull() && !right!!.isFull() || left!!.numNodes() < right!!.numNodes())
            right!!.addChild(data)
        else
            left!!.addChild(data)
    }

    /*
    Util
     */
    fun contains(data: T): Boolean = preorder().contains(data)

    /*
    Print stuff
     */
    fun print(order: KFunction0<ArrayList<T>> = this::preorder) {
        for (e in order())
            print("$e ")
        println()
    }

     fun preorder(): ArrayList<T> {
        val arr = ArrayList<T>()
        arr.add(this.data)
        left?.preorder()?.let { arr.addAll(it) }
        right?.preorder()?.let { arr.addAll(it) }
        return arr
    }

     fun inorder(): ArrayList<T> {
        val arr = ArrayList<T>()
        left?.inorder()?.let { arr.addAll(it) }
        arr.add(this.data)
        right?.inorder()?.let { arr.addAll(it) }
        return arr
    }

     fun postorder(): ArrayList<T> {
        val arr = ArrayList<T>()
        left?.postorder()?.let { arr.addAll(it) }
        right?.postorder()?.let { arr.addAll(it) }
        arr.add(this.data)
        return arr
    }

     fun dfs(): ArrayList<T> {
        val arr = ArrayList<T>()
        val queue = LinkedList<BTree<T>>()

        arr.add(this.data)

        if (left != null)
            queue.add(left!!)
        if (right != null)
            queue.add(right!!)

        var node: BTree<T>

        while (!queue.isEmpty()) {
            node = queue.remove()
            arr.add(node.data)
            if (node.left != null)
                queue.add(node.left!!)
            if (node.right != null)
                queue.add(node.right!!)
        }
        return arr
    }
}