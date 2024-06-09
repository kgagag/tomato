package test;

public class Test18 {
    public int test(){
        int[] a = new int[100];
        return a.length == 100 ? Result.SUCCESS : Result.FAILED ;
    }
}
