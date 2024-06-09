package test;

public class Test14 {
    public int test(){
        int[] arr = new int[100];
        arr[90] = 10086;
        return arr[90] == 10086 ? Result.SUCCESS : Result.FAILED ;
    }
}
