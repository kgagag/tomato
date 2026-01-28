package tomato.test;

public class Test85 {
    public Integer test(){
        int[][] ints = new int[1][1];
        //StringHelper.print20240503();
//        if("[[I".equals(ints.getClass().getName())){
//            return Result.SUCCESS;
//        }
        StringHelper.print20240503(ints.getClass().getName());
        return Result.SUCCESS;
    }

    public static void main(String[] args) {
        new Test85().test();
    }
}
