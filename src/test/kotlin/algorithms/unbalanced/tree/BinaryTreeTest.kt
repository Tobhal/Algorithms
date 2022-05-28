package algorithms.unbalanced.tree

import algorithms.unbalanced.array.BinaryTree
import org.junit.jupiter.api.Test

import org.junit.jupiter.api.Assertions.*

internal class BinaryTreeTest {

    @Test
    fun numNodes() {
        val root = BinaryTree(arrayListOf(8, 3, 1, 6, 4, 7, 10, 14, 13, 13))
        assertEquals(root.numNodes(), 9)
    }

    @Test
    fun numLeaves() {
        val root = BinaryTree(arrayListOf(8, 3, 1, 6, 4, 7, 10, 14, 13, 13))
        assertEquals(root.numLeaves(), 4)
    }

    @Test
    fun numTwoChildren() {
        val root = BinaryTree(arrayListOf(8, 3, 1, 6, 4, 7, 10, 14, 13, 13))
        assertEquals(root.numTwoChildren(), 3)
    }

    @Test
    fun numLevels() {
        val root = BinaryTree(arrayListOf(8, 3, 1, 6, 4, 7, 10, 14, 13, 13))
        assertEquals(root.numLevels(), 4)
    }

    @Test
    fun insert() {
        val root = BinaryTree(arrayListOf(8, 3, 1, 6, 4, 7, 10, 14, 13, 13))
        assertArrayEquals(root.inOrder().toArray(), arrayOf(1, 3, 4, 6, 7, 8, 10, 13, 14))
    }

    @Test
    fun remove() {
        val root = BinaryTree<Int>()
        val correct = ArrayList<Int>()

        // Test removing element not in root
        root.remove(1)
        assertEquals(root.numNodes(), 0)

        // Basic delete functionality
        for (i in 10 until 20) {
            root.insert(i)
            correct.add(i)
        }

        for (i in 9 downTo 0) {
            root.insert(i)
            correct.add(0, i)
        }

        for (i in 0 until 10) {
            root.remove(i)
            correct.remove(i)
            assertArrayEquals(root.inOrder().toArray(), correct.toArray())
        }

        for (i in 19 downTo 10) {
            root.remove(i)
            correct.remove(i)
            assertArrayEquals(root.inOrder().toArray(), correct.toArray())
        }

        // Edge cases
        // Deleting leaf
        root.insert(arrayListOf(10, 8, 9, 6, 7))
        root.remove(6)
        assertArrayEquals(root.inOrder().toArray(), arrayOf(7, 8, 9, 10))

        // Deleting internal node
        root.insert(6)
        root.remove(7)
        assertArrayEquals(root.inOrder().toArray(), arrayOf(6, 8, 9, 10))

        // Deleting an internal node and pushing new node up
        for (i in root.preOrder()) root.remove(i)

        root.insert(arrayListOf(10, 8, 9, 7))
        root.remove(8)
        assertArrayEquals(root.inOrder().toArray(), arrayOf(7, 9, 10))

        // Deleting an internal node and progressing down left subtree to rightmost
        for (i in root.preOrder()) root.remove(i)

        root.insert(arrayListOf(10, 8, 9, 6, 7))
        root.remove(8)
        assertArrayEquals(root.inOrder().toArray(), arrayOf(6, 7, 9, 10))
    }

    @Test
    fun contains() {
        val root = BinaryTree(arrayListOf(8, 3, 1, 6, 4, 7, 10, 14, 13))
        assertEquals(root.contains(6), true)
        assertEquals(root.contains(2), false)
        assertEquals(root.contains(8), true)
        assertEquals(root.contains(13), true)
        assertEquals(root.contains(15), false)
    }

    @Test
    fun preOrder() {
        val root = BinaryTree(arrayListOf(8, 3, 1, 6, 4, 7, 10, 14, 13))
        assertArrayEquals(root.preOrder().toArray(), arrayOf(8, 3, 1, 6, 4, 7, 10, 14, 13))
    }

    @Test
    fun inOrder() {
        val root = BinaryTree(arrayListOf(8, 3, 1, 6, 4, 7, 10, 14, 13))
        assertArrayEquals(root.inOrder().toArray(), arrayOf(1, 3, 4, 6, 7, 8, 10, 13, 14))
    }

    @Test
    fun postOrder() {
        val root = BinaryTree(arrayListOf(8, 3, 1, 6, 4, 7, 10, 14, 13))
        assertArrayEquals(root.postOrder().toArray(), arrayOf(1, 4, 7, 6, 3, 13, 14, 10 ,8))
    }

    @Test
    fun bfs() {
        val root = BinaryTree(arrayListOf(8, 3, 1, 6, 4, 7, 10, 14, 13))
        assertArrayEquals(root.bfs().toArray(), arrayOf(8, 3, 10, 1, 6, 14, 4, 7, 13))
    }
}