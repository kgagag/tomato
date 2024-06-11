package test;

public class HashCodeTest3 {
    public int test(){
        String[][] srr = new String[10][10];
        int hascode = srr.hashCode();
        StringHelper.print20240503( "hascode3:"+hascode);
        return Result.SUCCESS;
    }

    public int test2(Object object){
        return object.hashCode();
    }
}
