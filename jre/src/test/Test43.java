package test;

public class Test43 {
    public void test(){
        Value[] ints = new Value[]{new Value(1),new Value(2)};
        Value[] ints1 = new Value[4];
        System.arraycopy(ints, 0, ints1,0,2);
        System.out.println(ints1[0].id);
        ints1[0].id = 2;
        System.out.println(ints1[0].id);
    }

    public static void main(String[] args) {
        Test43 test43 = new Test43();
        test43.test();
    }
}
