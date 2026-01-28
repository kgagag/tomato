package tomato.test;

public class Test42 {
    public int test(){
        int[] ints = new int[]{1,2,3,4};
        int[] ints1 = new int[4];
        System.arraycopy(ints, 2, ints1,0,2);
        return ints1[1] == 4 ? Result.SUCCESS : Result.FAILED;
    }

    public static void main(String[] args) {
        Test42 test42 = new Test42();
        System.out.println(test42.test());
    }
}
