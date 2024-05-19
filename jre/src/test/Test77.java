package test;

public class Test77 {
    public int test(){
      String s =  System.getProperty("os.name");
        System.out.println(s);
        return 0;
    }

    public static void main(String[] args) {
        Test77 test77 = new Test77();
        test77.test();
    }
}
