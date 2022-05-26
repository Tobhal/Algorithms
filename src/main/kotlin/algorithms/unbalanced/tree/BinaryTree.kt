package algorithms.unbalanced.tree

import algorithms.util.Node
import java.util.*
import algorithms.interfaces.BinaryTree
import kotlin.collections.ArrayList

class BinaryTree<T: Comparable<T>> : BinaryTree<T> {
    private var root: Node<T>? = null

    /*
    Status stuff
     */
    override fun numNodes(): Int = numNodes(this.root)

    private fun numNodes(root: Node<T>? = this.root): Int {
        if (root == null) return 0

        return 1 + numNodes(root.left) + numNodes(root.right)
    }

    override fun numLeaves(): Int = numLeaves(this.root)

    private fun numLeaves(root: Node<T>? = this.root): Int {
        if (root == null) return 0

        if (root.left == null && root.right == null)
            return 1

        return numLeaves(root.left) + numLeaves(root.right)
    }

    override fun numTwoChildren(): Int = numTwoChildren(this.root)

    private fun numTwoChildren(root: Node<T>? = this.root): Int {
        if (root == null) return 0

        var add = 0

        if (root.left != null && root.right != null)
            add = 1

        return add + numTwoChildren(root.left) + numTwoChildren(root.right)
    }

    override fun numLevels(): Int = numLevels(this.root)

    private fun numLevels(root: Node<T>? = this.root): Int {
        if (root == null)
            return 0

        val nLeft = numLevels(root.left)
        val nRight = numLevels(root.right)

        return 1 + if (nLeft > nRight) nLeft else nRight
    }

    /*
    Add stuff
     */
    override fun insert(data: T) {
        this.root = insert(root, data)
    }

    override fun insert(data: List<T>) = data.forEach(){insert(it)}

    private fun insert(root: Node<T>?, data: T): Node<T> {
        if (root == null)
            return Node(data)
        else if (data < root.data)
            root.left = insert(root.left, data)
        else if (data > root.data)
            root.right = insert(root.right, data)
        else
            root.count++
        return root
    }

    override fun remove(data: T) {
        if (root == null)
            return

        if (data == root?.data)
            // Case 1 - Delete leaf Node
            root = replacement(root!!)
        else {
            var parent = root
            var finished = false

            var current = if (data < root!!.data)
                root!!.left
            else
                root!!.right

            while (current != null && !finished) {
                // Case 2 - Delete node with one child
                if (current.data == data) {
                    if (current == parent?.left)
                        parent.left = replacement(current)
                    else
                        parent?.right = replacement(current)
                    finished = true
                } else {
                    // Case 3 - Delete Node with 2 children
                    parent = current
                    if (data < current.data)
                        current = current.left
                    else
                        current = current.right
                }
            }

        }
    }

    private fun replacement(node: Node<T>): Node<T>? {
        if (node.left == null && node.right == null)
            return null
        else if (node.left != null && node.right == null)
            return node.left
        else if (node.left == null && node.right != null)
            return node.right
        else {
            if (node.left?.right == null) {
                node.left?.right = node.right
                return node.left
            }

            var current = node.left
            var parent = node
            while (current?.right != null) {
                parent = current
                current = current.right
            }

            parent.right = current?.left
            current?.left = node.left
            current?.right = node.right
            return current
        }
    }

    /*
    Util
     */
    override fun contains(data: T): Boolean = contains(data, root)

    private fun contains(data: T, root: Node<T>? = this.root): Boolean {
        if (root == null)
            return false

        if (root.data == data)
            return true

        return if (root.data < data)
            contains(data, root.right)
        else
            contains(data, root.left)
    }

    /*
    Print stuff
     */
    override fun print() = print(this::preOrder)
    fun print(order: () -> ArrayList<T> = this::preOrder) {
        for (e in order())
            print("$e ")
        println()
    }

    /*
    Traversal
     */
    override fun preOrder(): ArrayList<T> = preOrder(this.root!!)
    private fun preOrder(root: Node<T> = this.root!!): ArrayList<T> {
        val arr = ArrayList<T>()
        root.let { arr.add(it.data) }
        root.left?.let { preOrder(it) }?.let { arr.addAll(it) }
        root.right?.let { preOrder(it) }?.let { arr.addAll(it) }
        return arr
    }

    override fun inOrder(): ArrayList<T> = inOrder(this.root!!)
    private fun inOrder(root: Node<T> = this.root!!): ArrayList<T> {
        val arr = ArrayList<T>()
        root.left?.let { inOrder(it) }?.let { arr.addAll(it) }
        root.let { arr.add(it.data) }
        root.right?.let { inOrder(it) }?.let { arr.addAll(it) }
        return arr
    }

    override fun postOrder(): ArrayList<T> = postOrder(this.root!!)
    private fun postOrder(root: Node<T> = this.root!!): ArrayList<T> {
        val arr = ArrayList<T>()
        root.left?.let { postOrder(it) }?.let { arr.addAll(it) }
        root.right?.let { postOrder(it) }?.let { arr.addAll(it) }
        root.let { arr.add(it.data) }
        return arr
    }

    override fun bfs(): ArrayList<T> = bfs(this.root!!)
    fun bfs(root: Node<T> = this.root!!): ArrayList<T> {
        val arr = ArrayList<T>()
        val queue = LinkedList<Node<T>>()

        root.let { arr.add(it.data) }

        if (root.left != null)
            queue.add(root.left!!)
        if (root.right != null)
            queue.add(root.right!!)

        var node: Node<T>

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

    override fun dfs(): ArrayList<T> = dfs(this.root!!)

    private fun dfs(root: Node<T> = this.root!!): ArrayList<T> {
        TODO("Not yes implemented")
    }

    override fun toString(): String = this.root?.let { this.preOrder(it).toString() }!!
}