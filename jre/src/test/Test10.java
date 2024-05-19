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
        return a == 4951 ? 20240325 : 20240324 ;
    }

    public static void main(String[] args) {
        System.out.println(Test10.test());
    }
}
