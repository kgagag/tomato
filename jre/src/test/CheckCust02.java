package test;

public class CheckCust02 {
    public int test(){
        int a = 100;
        byte b = (byte) a;
        if(b == 100){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
