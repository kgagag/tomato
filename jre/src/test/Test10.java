package test;

public class Test10 {

    static int a = 1;
    public static void add(int i){
        if(i >= 100){
            return;
        }
        a += i;
        add( i + 1);
    }

    public static int test(){
        add(1);
        return a == 4951 ? Result.SUCCESS : Result.FAILED ;
    }

    public static void main(String[] args) {
        Test10.test();
    }
}
