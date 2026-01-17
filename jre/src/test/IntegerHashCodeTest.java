package test;

public class IntegerHashCodeTest {
    public int test(){
        Integer a = Integer.valueOf(1000);
        Integer b = 1000;
//        StringHelper.print20240503("hascode1:"+a.hashCode());
//        StringHelper.print20240503("hascode2:" +b.hashCode());
//        StringHelper.print20240503(a.toString());
//        StringHelper.print20240503(a.intValue() +"========"+b.intValue());
        if(a.intValue() == b.intValue()){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }

    public static void main(String[] args) {
        IntegerHashCodeTest hashCodeTest = new IntegerHashCodeTest();
        hashCodeTest.test();
    }
}
