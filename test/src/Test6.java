public class Test6 {

    int a = 1;
    public void add(int i){
        if(i >= 100){
            return;
        }
        a += i;
        add( i + 1);
    }

    public int test(){
         add(1);
         return a == 4951 ? 20240325 : 20240324 ;
    }

    public static void main(String[] args) {
        System.out.println(new Test6().test());
    }
}