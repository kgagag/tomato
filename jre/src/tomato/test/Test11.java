package tomato.test;

public class Test11 {

    static int a = 0;
    public static void add(int i){
        a += i;
    }

    public static int test(){
        //add(300);
        //add(199);
        //System.out.println(a);
        a = 300;
        return a == 300 ? Result.SUCCESS : Result.FAILED ;
    }

    public static void main(String[] args) {
        test();
    }
}
