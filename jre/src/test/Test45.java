package test;

public class Test45 {
    public int test(){
        String s = "hello";
        char[] ch = s.toCharArray();
        return ch[0] == 'h' && ch[1] == 'e' ? Result.SUCCESS : Result.FAILED;
    }
}
