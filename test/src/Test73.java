public class Test73 {
    public int test(){
        Throwable e = new Throwable("exception");
        StringHelper.print20240503(e.getMessage());
        //e.printStackTrace();
        return 20240325;
    }

    public static void main(String[] args) {
        Test73 test73 = new Test73();
        test73.test();
    }
}
