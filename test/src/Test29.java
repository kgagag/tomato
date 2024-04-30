public class Test29 {
    public int test(int[][] arr,int[][] brr){
      return   arr[0][0] + brr[0][0] == 3 ? 20240325 : 20240324;
    }

    public static void main(String[] args) {
        Test29 test27 = new Test29();
        System.out.println(test27.test(new int[][]{{1}},new int[][]{{2}}));
    }
}
