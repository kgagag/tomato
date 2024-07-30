package test;

public class CheckCust04 {
    public int test(){
        long a = -100;
        double b = (double) a;
        if(b == -100){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
