package test;

public class CheckCust05 {
    public int test(){
        long a = -100;
        int b = (int) a;
        if(b == -100){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
