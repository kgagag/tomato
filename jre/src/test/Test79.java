package test;

public class Test79 {
    private String name ;
    private String sex;

    @Override
    public String toString() {
        return name + "是" +sex;
    }

    public void test(){
        Test79 test79 = new Test79();
        test79.sex = "男";
        test79.name = "张三";
        StringHelper.print20240503(test79.toString());
    }
}
