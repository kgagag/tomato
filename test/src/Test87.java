import java.lang.reflect.Array;

public class Test87 {
    public void test(){
        String[][][] s = new String[10][10][10];
        Object o = Array.newInstance(s.getClass().getComponentType(), 10);
        String[][][] t = (String[][][]) o;
        System.out.println(t);
    }

    public static void main(String[] args) {
        Test87 test87 = new Test87();
        test87.test();
    }
}
