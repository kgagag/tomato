package test;

public class Test44 {
    public int test(){
        String s = "hello";
        char[] ch = s.toCharArray();
        return ch.length == 5 ? Result.SUCCESS : Result.FAILED;

    }
}
