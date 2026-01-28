package tomato.test;

public class Test97 {
    public int test()  {
        try {
            Class<?> clazz = Class.forName("java.lang.String");
            StringHelper.print20240503(clazz.getName());
            return Result.SUCCESS;
        }catch (Exception e){
            return Result.FAILED;
        }
    }

}
