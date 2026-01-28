package tomato.test;

public class Test28 {
    public int test(int[][] arr,int[][] brr){
      return   arr.length + arr[0].length + brr[0].length + brr.length == 300 ? Result.SUCCESS : Result.FAILED;
    }

    public static void main(String[] args) {
        Test28 test27 = new Test28();
        System.out.println(test27.test(new int[100][100],new int[50][50]));
    }
}
