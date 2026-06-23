# Skip List

A **skip list** is a data structure used to store items in sorted order while still allowing fast search, insert, and delete operations.

It is similar to a linked list, but with extra "shortcut" layers that let you skip over many items instead of checking every single one.

Think of it like a highway system:

- The bottom level is a normal road that visits every house.
- The higher levels are highways that skip over many houses.
- When searching, you move quickly on the highways, then drop down to smaller roads when you get close.

---

## Why Learn Skip Lists?

Skip lists are useful because they are easier to understand and implement than many balanced trees, while still giving similar average performance.

They are a good project if you want to practice:

- Linked lists
- Pointers or references
- Sorted data structures
- Searching efficiently
- Randomized algorithms
- Big-O complexity

Skip lists are commonly compared to structures like:

- Binary Search Trees
- AVL Trees
- Red-Black Trees
- B-Trees

The main difference is that skip lists use **randomness** instead of complex tree rotations to stay efficient.

---

## Basic Idea

A normal linked list looks like this:

```text
1 -> 3 -> 5 -> 7 -> 9 -> 12 -> 20
```

If you want to find `20`, you may need to check almost every item.

A skip list adds extra levels above the normal list:

```text
Level 3: 1 ---------------------> 20
Level 2: 1 --------> 7 ---------> 20
Level 1: 1 -> 3 -> 5 -> 7 -> 9 -> 12 -> 20
```

The bottom level contains every value.

The higher levels contain only some values. These higher levels act like shortcuts.

---

## Search Example

Suppose we want to find `12`.

```text
Level 3: 1 ---------------------> 20
Level 2: 1 --------> 7 ---------> 20
Level 1: 1 -> 3 -> 5 -> 7 -> 9 -> 12 -> 20
```

Search steps:

1. Start at the top-left node.
2. Move right while the next value is not too large.
3. If the next value is too large, move down one level.
4. Repeat until you find the value or reach the bottom level.

For `12`, the search might look like:

```text
Start at 1
Try to jump to 20, but 20 is too large
Move down
Jump to 7
Try to jump to 20, but 20 is too large
Move down
Move to 9
Move to 12
Found it
```

Instead of checking every item, the skip list skips over large sections.

---

## Operations

A basic skip list usually supports these operations:

```text
search(value)
insert(value)
delete(value)
```

### Search

Find whether a value exists in the skip list.

Average time complexity:

```text
O(log n)
```

### Insert

Add a new value while keeping the list sorted.

Average time complexity:

```text
O(log n)
```

### Delete

Remove a value from the skip list.

Average time complexity:

```text
O(log n)
```

---

## How Insert Works

When inserting a new value:

1. Search for where the value should go.
2. Insert it into the bottom level.
3. Randomly decide whether it should also appear on higher levels.
4. Link the new node into each selected level.

Most values only appear on the bottom level.

Some values appear on higher levels and become shortcuts.

Example:

```text
Insert 10

Before:

Level 2: 1 --------> 7 ---------> 20
Level 1: 1 -> 3 -> 5 -> 7 -> 9 -> 12 -> 20

After:

Level 2: 1 --------> 7 ---------> 20
Level 1: 1 -> 3 -> 5 -> 7 -> 9 -> 10 -> 12 -> 20
```

If `10` gets promoted to a higher level randomly, it could become:

```text
Level 2: 1 --------> 7 ----> 10 ----> 20
Level 1: 1 -> 3 -> 5 -> 7 -> 9 -> 10 -> 12 -> 20
```

---

## Why Randomness?

Skip lists use randomness to decide how tall each node should be.

A common approach is like flipping a coin:

```text
Heads: promote the node to the next level
Tails: stop
```

So if you insert a value:

- It always appears on level 1.
- It has about a 50% chance of appearing on level 2.
- It has about a 25% chance of appearing on level 3.
- It has about a 12.5% chance of appearing on level 4.

This creates a structure where higher levels have fewer nodes.

That is what makes searching fast.

---

## Time Complexity

| Operation | Average Case | Worst Case |
|---|---:|---:|
| Search | O(log n) | O(n) |
| Insert | O(log n) | O(n) |
| Delete | O(log n) | O(n) |

The worst case happens if the random levels are very unlucky.

In practice, skip lists are usually fast.

---

## Space Complexity

A skip list uses extra memory because some values appear on multiple levels.

Average space complexity:

```text
O(n)
```

This means memory usage grows roughly linearly with the number of items.

---

## Skip List vs Binary Search Tree

| Feature | Skip List | Binary Search Tree |
|---|---|---|
| Keeps values sorted | Yes | Yes |
| Fast search | Yes, average O(log n) | Yes, if balanced |
| Fast insert/delete | Yes, average O(log n) | Yes, if balanced |
| Uses randomness | Yes | Usually no |
| Needs rotations | No | Balanced trees often do |
| Beginner-friendly | Medium | Medium to hard |

Skip lists are often easier to implement than AVL trees or Red-Black trees because they do not need rotations.

---

## Suggested Challenge Requirements

For a coding challenge, a good skip list project should include:

### Required

- Store integer values
- Insert values
- Search for values
- Delete values
- Print the skip list by level
- Keep values sorted
- Use random promotion for levels

### Nice Extras

- Support strings or generic values
- Prevent duplicate values
- Add unit tests
- Add a command-line interface
- Visualize the levels
- Benchmark against a normal linked list
- Benchmark against a binary search tree

---

## Example API

Your skip list could expose functions like this:

```text
skiplist.insert(10)
skiplist.insert(5)
skiplist.insert(20)

skiplist.search(10)  // true
skiplist.search(99)  // false

skiplist.delete(5)
```

Example output:

```text
Level 3: 10
Level 2: 10 -> 20
Level 1: 5 -> 10 -> 20
```

---

## Beginner Implementation Tips

Start simple.

Do not try to make the skip list generic at first. Use integers.

A node can store:

```text
value
forward pointers
```

The forward pointers are links to the next node at each level.

For example, a node that appears on 3 levels needs 3 forward pointers.

Pseudo-structure:

```text
Node:
    value
    forward[]
```

The skip list itself usually stores:

```text
SkipList:
    head
    max_level
    current_level
    probability
```

Common values:

```text
max_level = 16
probability = 0.5
```

---

## Common Bugs

Watch out for these:

- Forgetting to update links on every level during insert
- Deleting from the bottom level but not higher levels
- Off-by-one errors with level indexes
- Letting the current level stay too high after deleting tall nodes
- Handling empty lists incorrectly
- Handling duplicate values inconsistently

---

## Difficulty

For a one-week challenge, skip list is a good **medium-hard** project.

It is harder than:

- Linked List
- Stack
- Queue
- Ring Buffer
- Hash Map

It is probably easier than:

- B-Tree
- Red-Black Tree
- Garbage Collector
- Chess Engine
- HTTP Server from Scratch

Recommended challenge rating:

```text
Difficulty: 7/10
Time: 3-7 days
Best for: data structure practice
```

---

## Learning Links

Here are a few helpful explanations:

- Wikipedia overview: https://en.wikipedia.org/wiki/Skip_list
- GeeksforGeeks tutorial: https://www.geeksforgeeks.org/dsa/skip-list/
- Brilliant explanation: https://brilliant.org/wiki/skip-lists/
- Visualgo data structure visualizations: https://visualgo.net/en/list
- Original paper by William Pugh: https://15721.courses.cs.cmu.edu/spring2018/papers/08-oltpindexes1/pugh-skiplists-cacm1990.pdf

---

## Summary

A skip list is a sorted linked list with shortcut layers.

It gives fast average search, insert, and delete performance without needing complicated tree balancing.

The key idea is:

```text
Use random levels to create shortcuts.
Move right when possible.
Move down when the next value is too large.
```

That is what makes the structure both simple and powerful.