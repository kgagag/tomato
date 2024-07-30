package test;

public class Test40 {
    public int test(){
//        StringBuilder stringBuilder = new StringBuilder("hello");
//        test.StringHelper.print20240503(stringBuilder.toString());
        String s = "abcdefg";
        return s.length() == 7 ? Result.SUCCESS : Result.FAILED;
    }
}
