package tomato.test;

public class IntegerHashCodeTest2 {
    public int test(){
        Integer i = Integer.valueOf(10000);
        int hasCode = test2(i);
        StringHelper.print20240503("hascode:"+hasCode);
        if (hasCode == 10000){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }

    public int test2(Object object){
        int a  = object.hashCode();
        StringHelper.print20240503("hascode2:"+a);
        return a;
    }

    public static void main(String[] args) {
        StringHelper.print20240503(new IntegerHashCodeTest2().test()+"");
    }
}
