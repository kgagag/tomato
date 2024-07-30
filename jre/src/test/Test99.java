package test;

public class Test99 {
    public int test (){
        String a = "哈哈哈";
        byte[] bytes = a.getBytes();
        return bytes.length;
    }

    public static void main(String[] args) {
        Test99 test99 = new Test99();
        System.out.println(test99.test());
    }
}
