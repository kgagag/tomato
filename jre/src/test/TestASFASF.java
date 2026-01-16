class A {
    public static int value = 10;
}

class B extends A {
    // 没有成员变量
}

class C extends B {
    // 没有成员变量
}

public class TestASFASF {
    public static void main(String[] args) {
        C c = new C();
        System.out.println(c.value); // 输出: 10

        // 更推荐通过类名访问静态成员
        System.out.println(A.value); // 输出: 10
        System.out.println(C.value); // 输出: 10

        // 修改会影响所有访问
        c.value = 20;
        System.out.println(A.value); // 输出: 20
        System.out.println(B.value); // 输出: 20
    }
}