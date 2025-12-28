package test;

public class Test102 {
    private void test(Object o){
        int[] a = (int[]) o;
        System.out.println(a.length);
    }

    public static void main(String[] args) {
        int[][] a = new int[5][6];
        new Test102().test(a);
    }
}
