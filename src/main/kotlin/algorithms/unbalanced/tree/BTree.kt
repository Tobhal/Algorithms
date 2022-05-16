package algorithms.unbalanced.tree

import algorithms.util.Node
import java.util.*
import kotlin.reflect.KFunction1

class BTree<T> where T: Comparable<T> {
    private var root: Node<T>? = null

    /*
    Status stuff
     */
    fun numNodes(root: Node<T>? = this.root): Int {
        if (root == null)
            return 0

        return 1 + numNodes(root.left) + numNodes(root.right)
    }

    fun numLeaves(root: Node<T>? = this.root): Int {
        if (root == null)
            return 0

        if (root.left == null && root.right == null)
            return 1

        return numLeaves(root.left) + numLeaves(root.right)
    }

    fun numTwoChildren(root: Node<T>? = this.root): Int {
        if (root == null)
            return 0

        var add = 0

        if (root.left != null && root.right != null)
            add = 1

        return add + numTwoChildren(root.left) + numTwoChildren(root.right)
    }

    fun numLevels(root: Node<T>? = this.root): Int {
        if (root == null)
            return 0

        val nLeft = numLevels(root.left)
        val nRight = numLevels(root.right)

        return 1 + if (nLeft > nRight) nLeft else nRight
    }

    /*
    Add stuff
     */
    fun insert(data: T) {
        this.root = insert(root, data)
    }

    fun insert(data: List<T>) {
        for (e in data) {
            root = insert(root, e)
        }
    }

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

    fun remove(data: T) {
        if (root == null)
            return

        if (data == root?.data)
            root = replacement(root!!)
        else {
            var parent = root
            var finished = false

            var current = if (data < root!!.data)
                root!!.left
            else
                root!!.right

            while (current != null && !finished) {
                if (current.data == data) {
                    if (current == parent?.left)
                        parent.left = replacement(current)
                    else
                        parent?.right = replacement(current)
                    finished = true
                } else {
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
    // fun contains(data: T): Boolean = root?.let { preorder(it).contains(data) }!!

    fun contains(data: T, root: Node<T>? = this.root): Boolean {
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
    fun print(order: KFunction1<Node<T>, ArrayList<T>> = this::preorder) {
        for (e in root?.let { order(it) }!!)
            print("$e ")
        println()
    }

    fun preorder(root: Node<T>): ArrayList<T> {
        val arr = ArrayList<T>()
        root.let { arr.add(it.data) }
        root.left?.let { preorder(it) }?.let { arr.addAll(it) }
        root.right?.let { preorder(it) }?.let { arr.addAll(it) }
        return arr
    }

    fun inorder(root: Node<T>): ArrayList<T> {
        val arr = ArrayList<T>()
        root.left?.let { inorder(it) }?.let { arr.addAll(it) }
        root.let { arr.add(it.data) }
        root.right?.let { inorder(it) }?.let { arr.addAll(it) }
        return arr
    }

    fun postorder(root: Node<T>): ArrayList<T> {
        val arr = ArrayList<T>()
        root.left?.let { postorder(it) }?.let { arr.addAll(it) }
        root.right?.let { postorder(it) }?.let { arr.addAll(it) }
        root.let { arr.add(it.data) }
        return arr
    }

     fun bfs(root: Node<T>): ArrayList<T> {
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
}