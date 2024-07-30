package test;

public class Test55 {
    public int test(){
        Float f = 100f;
        int a = 100;
        Float c = a + f;
        return c == 200 ? Result.SUCCESS : Result.FAILED;
    }
}
