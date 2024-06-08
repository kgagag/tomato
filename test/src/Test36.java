public class Test36 {
    public int test(int b , int c){
        try {
            int a = b / c;
            return 20240324;
        }catch (Exception e){
            return 20240325;
        }
    }

    public static void main(String[] args) {
        Test36 test36 = new Test36();
        System.out.println(test36.test(1,0));
    }
}
