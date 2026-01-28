package tomato.test;

public class Test103 {

    int a ;
    int b;
   public void test(){
       a = 100;
       b = 200;
       int c = a + b;
   }

    public static void main(String[] args) {
       Test103 test103 = new Test103();
        test103.a = 100;
        test103.b = 200;
        int c = test103.a + test103.b;
    }

}
