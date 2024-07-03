package test;

public class Test94 {
    public int test(){
        int[] array = new int[10];
        if(array.getClass().isArray()){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
