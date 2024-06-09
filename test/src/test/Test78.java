package test;

import java.util.Map;
import java.util.Objects;

public class Test78 {

    public int test(){
//        Node<String,String> node = new Node(1,"name","zhangsan",null);
////        test.StringHelper.print20240503(node.key);
////        test.StringHelper.print20240503(node.value);
//        test.StringHelper.print20240503(node.toString());
//        return 20240325 ;

        String str = "hello ";
        String str1 = "world";
        String str3 = str + str1;
        StringHelper.print20240503(str3);
        return 20240325 ;
    }
    static class Node<K,V> implements Map.Entry<K,V> {
        final int hash;
        final K key;
        V value;
        Node<K,V> next;

        Node(int hash, K key, V value, Node<K,V> next) {
            this.hash = hash;
            this.key = key;
            this.value = value;
            this.next = next;
        }

        public final K getKey()        { return key; }
        public final V getValue()      { return value; }
        @Override
        public final String toString() { return key + "=" + value; }

        public final int hashCode() {
            return Objects.hashCode(key) ^ Objects.hashCode(value);
        }

        public final V setValue(V newValue) {
            V oldValue = value;
            value = newValue;
            return oldValue;
        }

        public final boolean equals(Object o) {
            if (o == this)
                return true;
            if (o instanceof Map.Entry) {
                Map.Entry<?,?> e = (Map.Entry<?,?>)o;
                if (Objects.equals(key, e.getKey()) &&
                        Objects.equals(value, e.getValue()))
                    return true;
            }
            return false;
        }
    }
}
