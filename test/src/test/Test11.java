package test;

public class Test11 {

    static int a = 0;
    public static void add(int i){
        a += i;
    }

    public static int test(){
        add(300);
        //add(199);
        //System.out.println(a);
        return a == 300 ? Result.SUCCESS : Result.FAILED ;
    }
}
