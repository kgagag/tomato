public class Test15 {
    public int test(){
        int[][][] arr = new int[10][10][10];
        arr[1][2][3] = 11;
        return arr[1][2][3] == 11 ? 20240325 : 20240324 ;
    }

    public static void main(String[] args) {
        Test15 test15 = new Test15();
        System.out.println(test15.test());
    }
}