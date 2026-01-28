package tomato.test;

public class CheckCust07 {
    public int test(){
        double a = -100d;
        long b = (long) a;
        if(b == -100){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
