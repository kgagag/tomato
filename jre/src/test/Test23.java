package test;

public class Test23 {
    public int test(){
        Test23[] test23s = new Test23[10];
        return test23s.length == 10 ? Result.SUCCESS : Result.FAILED;
    }
}
