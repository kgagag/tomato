package test;

public class Test93 {
    public int test(){
        String str = "hello world";
        str.replaceAll("hello","ni hao");
        if(str.equals("nihao world")){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
