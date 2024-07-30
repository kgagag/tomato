package test;

public class Test90 {
    public int test(){
        //StringHelper.print20240503("==="+add(Long.MAX_VALUE,Integer.MAX_VALUE));
        long a =  sub(Long.MAX_VALUE,Long.MAX_VALUE - 1000);
       // StringHelper.print20240503(a +"===");
        return Result.SUCCESS;
    }

    public long sub(long l1 ,long l2){
        return l1 - l2;
    }
}
