package tomato.test;

public class Test17 {
    /**
     * 实现计算多维数组索引对应的一维数组索引值
     * @param arrDesc 多维数组描述，比如三维数组 a * b * c 的数组就传 [a, b, c]
     * @param indexArr 要计算的多维数组索引，比如[1][2][3] 就传 [1, 2, 3]
     * @return 返回一维数组索引值
     */
    private int getIndex(int[] arrDesc, int[] indexArr) {
        int index = 0;
        int product = 1;
        for (int i = arrDesc.length - 1; i >= 0; i--) {
            index += indexArr[i] * product;
            product *= arrDesc[i];
        }
        return index;
    }

    public static void main(String[] args) {
        Test17 test = new Test17();
        int[] arrDesc = {3, 3};  // 例如 4x5x6 的三维数组
        int[] indexArr = {1,2}; // 访问元素 [1][2][3]
        int flatIndex = test.getIndex(arrDesc, indexArr);
        System.out.println("一维数组索引值: " + flatIndex);
    }
}
