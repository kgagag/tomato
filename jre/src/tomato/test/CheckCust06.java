package tomato.test;

public class CheckCust06 {
    public int test(){
        long a = -100;
        float b = (float) a;
        if(b == -100){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
