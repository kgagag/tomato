package test;

public class Test19 {
    public int test(){
        int[] arr = new int[100];
        for(int i = 0; i < arr.length;i++){
            arr[i] = i;
        }
        return arr[99] == 99 ? Result.SUCCESS : Result.FAILED ;
    }
}
