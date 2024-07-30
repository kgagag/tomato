package test;

public class IntegerHashCodeTest2 {
    public int test(){
        Integer i = Integer.valueOf(10000);
        Integer hasCode = test2(i);
        StringHelper.print20240503("hascode:"+hasCode);
        if (hasCode == 10000){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }

    public int test2(Object object){
        return object.hashCode();
    }
}
