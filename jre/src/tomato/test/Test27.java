package tomato.test;

public class Test27 {
    public int test(int[][] arr,int[][] brr){
        for(int i = 0; i < arr.length; i++){
            for(int j= 0; j < arr[i].length;j++){
                arr[i][j] = j * i;
            }
        }

        for(int i = 0; i < brr.length; i++){
            for(int j= 0; j < brr[i].length;j++){
                brr[i][j] = j * i;
            }
        }

        return arr[arr.length - 1][arr[0].length - 1] + brr[brr.length - 1][brr[0].length - 1]  == 12202 ? Result.SUCCESS : Result.FAILED;
    }

    public static void main(String[] args) {
        Test27 test27 = new Test27();
        System.out.println(test27.test(new int[100][100],new int[50][50]));
    }
}
