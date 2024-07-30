package test;

public class CheckCust01 {
    public int test(){
        int a = -100;
        short b = (short) a;
        if(b == -100){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
