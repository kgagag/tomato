public class Test60 {
    public int test(){
      long l =   Double.doubleToLongBits(Double.longBitsToDouble(10));
      return l == 10 ? 20240325 : 20240324;
    }

    public static void main(String[] args) {
        Test60 test60 = new Test60();
        System.out.println(test60.test());
    }
}
