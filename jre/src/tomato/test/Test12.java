package tomato.test;

public class Test12 {
    public static int add(int i, int j){
       return i + j;
    }

    public static int test(){
        return add(100,200) == 300 ? Result.SUCCESS : Result.FAILED ;
    }
}
