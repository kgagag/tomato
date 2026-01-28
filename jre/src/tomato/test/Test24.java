package tomato.test;

public class Test24 {
    public int test(int[] irr, long[] lrr){
        return irr.length + lrr.length;
    }

    public static void main(String[] args) {
        new Test24().test(new int[20240320], new long[5]);
    }
}
