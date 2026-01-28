package tomato.test;

public class OddEvenPrinter {
    private static final int MAX = 100;
    private int number = 1;
    private final Object lock = new Object();
    
    public static void main(String[] args) {
        OddEvenPrinter printer = new OddEvenPrinter();
        
        Thread threadA = new Thread(() -> printer.printOdd(), "A");
        Thread threadB = new Thread(() -> printer.printEven(), "B");
        
        threadA.start();
        threadB.start();
    }
    
    public void printOdd() {
        synchronized (lock) {
            while (number <= MAX) {
                // 如果是偶数，等待
                while (number % 2 == 0) {
                    try {
                        lock.wait();
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                }
                
                if (number > MAX) break;
                
                System.out.println(Thread.currentThread().getName() + ": " + number);
                number++;
                lock.notifyAll();
            }
        }
    }
    
    public void printEven() {
        synchronized (lock) {
            while (number <= MAX) {
                // 如果是奇数，等待
                while (number % 2 != 0) {
                    try {
                        lock.wait();
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                }
                
                if (number > MAX) break;
                
                System.out.println(Thread.currentThread().getName() + ": " + number);
                number++;
                lock.notifyAll();
            }
        }
    }
}