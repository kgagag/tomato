package test;

public class Test29 {
    public int test(int[][] arr,int[][] brr){
      return   arr[0][0] + brr[0][0] == 3 ? Result.SUCCESS : Result.FAILED;
    }

    public static void main(String[] args) {
        Test29 test27 = new Test29();
        System.out.println(test27.test(new int[][]{{1}},new int[][]{{2}}));
    }
}
