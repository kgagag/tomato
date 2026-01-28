package tomato.test;

public class CheckCust08 {
    public int test(){
        float a = -100f;
        double b = (double) a;
        if(b == -100d){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
