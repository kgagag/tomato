package test;

public class Test93 {
    public int test(){
        String str = "hello world";
        str = str.replaceAll("hello","ni hao");
        if(str.equals("ni hao world")){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }

    public static void main(String[] args) {
        System.out.println(new Test93().test());
    }
}
