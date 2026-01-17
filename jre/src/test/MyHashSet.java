package test;

import java.util.Arrays;
import java.util.ConcurrentModificationException;
import java.util.Iterator;
import java.util.NoSuchElementException;

public class MyHashSet<E> implements Iterable<E> {
    // 默认初始容量
    private static final int DEFAULT_CAPACITY = 16;
    
    // 默认负载因子
    private static final float DEFAULT_LOAD_FACTOR = 0.75f;
    
    // 存储元素的数组（使用链表解决哈希冲突）
    private Node<E>[] table;
    
    // 当前元素数量
    private int size;
    
    // 负载因子
    private final float loadFactor;
    
    // 扩容阈值
    private int threshold;
    
    // 节点类，用于解决哈希冲突（链表法）
    private static class Node<E> {
        final int hash;
        final E key;
        Node<E> next;
        
        Node(int hash, E key, Node<E> next) {
            this.hash = hash;
            this.key = key;
            this.next = next;
        }
    }
    
    // 构造函数
    public MyHashSet() {
        this(DEFAULT_CAPACITY, DEFAULT_LOAD_FACTOR);
    }
    
    public MyHashSet(int initialCapacity) {
        this(initialCapacity, DEFAULT_LOAD_FACTOR);
    }
    
    @SuppressWarnings("unchecked")
    public MyHashSet(int initialCapacity, float loadFactor) {
        if (initialCapacity < 0)
            throw new IllegalArgumentException("Illegal initial capacity: " + initialCapacity);
        if (loadFactor <= 0 || Float.isNaN(loadFactor))
            throw new IllegalArgumentException("Illegal load factor: " + loadFactor);
        
        this.loadFactor = loadFactor;
        this.table = new Node[initialCapacity];
        this.threshold = (int) (initialCapacity * loadFactor);
        this.size = 0;
    }
    
    // 添加元素
    public boolean add(E element) {
        if (element == null) {
            throw new NullPointerException("Null elements are not allowed");
        }
        
        int hash = hash(element);
        int index = indexFor(hash, table.length);
        // 检查是否已存在相同元素
        Node<E> current = table[index];
        while (current != null) {
            if (current.hash == hash && 
                (current.key == element || current.key.equals(element))) {
                return false; // 元素已存在
            }
            current = current.next;
        }
        
        // 添加到链表头部
        Node<E> newNode = new Node<>(hash, element, table[index]);
        table[index] = newNode;
        StringHelper.print20240503("添加元素："+index);
        if(table[ index] == null){
            StringHelper.print20240503("失效");
        }
        size++;
        
        // 检查是否需要扩容
        if (size >= threshold) {
            resize(table.length * 2);
        }
        
        return true;
    }
    
    // 移除元素
    public boolean remove(Object element) {
        if (element == null) return false;
        
        int hash = hash(element);
        int index = indexFor(hash, table.length);
        
        Node<E> prev = null;
        Node<E> current = table[index];
        
        while (current != null) {
            if (current.hash == hash && 
                (current.key == element || current.key.equals(element))) {
                
                if (prev == null) {
                    table[index] = current.next;
                } else {
                    prev.next = current.next;
                }
                size--;
                return true;
            }
            prev = current;
            current = current.next;
        }
        
        return false;
    }
    
    // 检查是否包含元素
    public boolean contains(Object element) {
        if (element == null) return false;
        
        int hash = hash(element);
        int index = indexFor(hash, table.length);
        Node<E> current = table[index];



        while (current != null) {
            if (current.hash == hash && 
                (current.key == element || current.key.equals(element))) {
                return true;
            }
            current = current.next;
        }
        
        return false;
    }
    
    // 获取元素数量
    public int size() {
        return size;
    }
    
    // 检查是否为空
    public boolean isEmpty() {
        return size == 0;
    }
    
    // 清空集合
    @SuppressWarnings("unchecked")
    public void clear() {
        table = new Node[table.length];
        size = 0;
    }
    
    // 转换为数组
    @SuppressWarnings("unchecked")
    public E[] toArray() {
        E[] result = (E[]) new Object[size];
        int index = 0;
        
        for (Node<E> node : table) {
            while (node != null) {
                result[index++] = node.key;
                node = node.next;
            }
        }
        
        return result;
    }
    
    // 扩容方法
    @SuppressWarnings("unchecked")
    private void resize(int newCapacity) {
        Node<E>[] oldTable = table;
        table = new Node[newCapacity];
        threshold = (int) (newCapacity * loadFactor);
        
        // 重新哈希所有元素
        for (Node<E> node : oldTable) {
            while (node != null) {
                Node<E> next = node.next;
                int newIndex = indexFor(node.hash, newCapacity);
                node.next = table[newIndex];
                table[newIndex] = node;
                node = next;
            }
        }
    }
    
    // 计算哈希值
    private int hash(Object key) {
        int h = key.hashCode();
        return h ^ (h >>> 16); // 扰动函数，减少碰撞
    }
    
    // 计算索引位置
    private int indexFor(int hash, int length) {
        return (length - 1) & hash; // 相当于 hash % length，但性能更好
    }
    
    // 迭代器实现
    @Override
    public Iterator<E> iterator() {
        return new HashSetIterator();
    }
    
    private class HashSetIterator implements Iterator<E> {
        private Node<E> next;        // 下一个要返回的节点
        private Node<E> current;     // 当前节点
        private int index;           // 当前桶索引
        private int expectedSize;    // 用于快速失败
        
        HashSetIterator() {
            expectedSize = size;
            // 找到第一个非空桶
            for (index = 0; index < table.length; index++) {
                if (table[index] != null) {
                    next = table[index];
                    break;
                }
            }
        }
        
        @Override
        public boolean hasNext() {
            return next != null;
        }
        
        @Override
        public E next() {
            if (size != expectedSize) {
                throw new ConcurrentModificationException();
            }
            
            if (next == null) {
                throw new NoSuchElementException();
            }
            
            current = next;
            next = current.next;
            
            // 如果当前链表遍历完，找下一个非空桶
            if (next == null) {
                for (index = index + 1; index < table.length; index++) {
                    if (table[index] != null) {
                        next = table[index];
                        break;
                    }
                }
            }
            
            return current.key;
        }
        
        @Override
        public void remove() {
            if (current == null) {
                throw new IllegalStateException();
            }
            
            MyHashSet.this.remove(current.key);
            expectedSize = size;
            current = null;
        }
    }
    
    // 测试方法
//    public static void main(String[] args) {
//        MyHashSet<String> set = new MyHashSet<>();
//
//        // 测试添加
//        System.out.println("添加元素:");
//        System.out.println("添加 'apple': " + set.add("apple"));
//        System.out.println("添加 'banana': " + set.add("banana"));
//        System.out.println("添加 'apple' 第二次: " + set.add("apple")); // 应该返回false
//
//        // 测试包含
//        System.out.println("\n检查包含:");
//        System.out.println("包含 'apple': " + set.contains("apple"));
//        System.out.println("包含 'orange': " + set.contains("orange"));
//
//        // 测试大小
//        System.out.println("\n集合大小: " + set.size());
//
//        // 测试迭代器
//        System.out.println("\n遍历集合:");
//        for (String fruit : set) {
//            System.out.println(fruit);
//        }
//
//        // 测试移除
//        System.out.println("\n移除 'banana': " + set.remove("banana"));
//        System.out.println("移除后大小: " + set.size());
//
//        // 测试转数组
//        System.out.println("\n转换为数组:");
//        String[] array = set.toArray();
//        System.out.println(Arrays.toString(array));
//
//        // 测试清空
//        set.clear();
//        System.out.println("\n清空后大小: " + set.size());
//        System.out.println("是否为空: " + set.isEmpty());
//    }

    public static void main(String[] args) {
        MyHashSet<String> set = new MyHashSet<>();
        String a = "apple";
        set.add("apple");
        //set.add("banana");
        StringHelper.print20240503("set 的容量是：" + set.size());
        if(set.contains("apple")){
            StringHelper.print20240503("set 包含 apple");
        }else {
            StringHelper.print20240503("set 不包含 apple");
        }
    }
}