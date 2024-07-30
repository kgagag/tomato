package test;

public class CheckCust03 {
    public int test(){
        int a = 100;
        char b = (char) a;
        if(b == 100){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
