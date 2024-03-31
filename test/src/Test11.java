public class Test11 {

    static int a = 1;
    public static void add(int i){
        a += i;
    }

    public static int test(){
        add(100);
        add(199);
        //System.out.println(a);
        return a == 300 ? 20240325 : 20240324 ;
    }
}
