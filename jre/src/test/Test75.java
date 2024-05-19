package test;

public class Test75 {
    public int test(){
        Throwable e = new RuntimeException("exception");
        StringHelper.print20240503(e.getMessage());
        //e.printStackTrace();
        return 20240325;
    }

    public static void main(String[] args) {
        Test75 test73 = new Test75();
        test73.test();
    }
}
