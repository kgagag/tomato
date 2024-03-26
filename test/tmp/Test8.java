public class Test8 {

    private static int age;

    static String name = "张三";

    public static void main(String[] args) {
        Test8 t = new Test8();
        for(int i = 0; i <= 100;i++){
              t.age  += i;;
        }
        int a = t.age + 100;
    }
}