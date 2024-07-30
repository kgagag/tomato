package test;

public class Test37 {
    public int test(){
        String str = "hello world";
        return str.charAt(2) == 'l' ? Result.SUCCESS : Result.FAILED;
    }
}
