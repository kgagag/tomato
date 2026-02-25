package tomato.test;

public class Test112 {
    public static void main(String[] args) {
        Test112 test112 = new Test112();
        try {
            test112.a();
        }catch (Exception e){
            StackTraceElement[]  err =   e.getStackTrace();
            System.out.println(err.length);
            e.fillInStackTrace();
            e.printStackTrace();
        }
    }

    private void a (){
        b();
    }

    private void b (){
        c();
    }

    private void c (){
        d();
    }

    private void d (){
        throw new RuntimeException();
    }
}
