public class Test16 {
    public int test(){
        int[][][] arr = new int[10][10][10];
        int a = 0;
        for(int i = 0 ;i <arr.length;i++){
            for(int j = 0 ;j < arr[i].length;j++){
                for(int k = 0 ;k < arr[i].length;k++){
                    arr[i][j][k] = a ++;
                }
            }
        }
        return arr[9][9][9] == 999 ? 20240325 : 20240324 ;
    }

    public static void main(String[] args) {
        Test16 test15 = new Test16();
        System.out.println(test15.test());
    }
}
