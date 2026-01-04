package test;

public class Test15 {
    public int test(){
        int[][][] arr = new int[10][8][6];
        arr[1][2][3] = 11;
        arr[0][2][3] = 22;
        return arr[1][2][3] == 11 && arr[0][2][3] == 22 ? Result.SUCCESS : Result.FAILED ;
    }

    public static void main(String[] args) {
        Test15 test15 = new Test15();
        System.out.println(test15.test());
    }
}
