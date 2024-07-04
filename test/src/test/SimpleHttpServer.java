import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.io.PrintWriter;
import java.net.ServerSocket;
import java.net.Socket;
import java.nio.charset.StandardCharsets;

public class SimpleHttpServer {
    public static void main(String[] args) {
        int port = 8080;
        try (ServerSocket serverSocket = new ServerSocket(port)) {
            System.out.println("HTTP 服务器正在运行，端口：" + port);
            while (true) {
                try (Socket clientSocket = serverSocket.accept()) {
                    handleClientRequest(clientSocket);
                } catch (Exception e) {
                    System.err.println("客户端请求处理失败: " + e.getMessage());
                }
            }
        } catch (Exception e) {
            System.err.println("服务器启动失败: " + e.getMessage());
        }
    }

    private static void handleClientRequest(Socket clientSocket) {
        try (
                BufferedReader in = new BufferedReader(new InputStreamReader(clientSocket.getInputStream(), StandardCharsets.UTF_8));
                PrintWriter out = new PrintWriter(new OutputStreamWriter(clientSocket.getOutputStream(), StandardCharsets.UTF_8), true);
        ) {
            String inputLine;
            StringBuilder request = new StringBuilder();
            while ((inputLine = in.readLine()) != null && !inputLine.isEmpty()) {
                request.append(inputLine).append("\n");
            }
            System.out.println("接收到请求:\n" + request.toString());

            // 构建 HTTP 响应
            String httpResponse = "HTTP/1.1 200 OK\r\n" +
                    "Content-Type: text/html; charset=UTF-8\r\n" +
                    "Content-Length: " + getHomePage().getBytes(StandardCharsets.UTF_8).length + "\r\n" +
                    "\r\n" +
                    getHomePage();

            out.write(httpResponse);
            out.flush();
        } catch (Exception e) {
            System.err.println("请求处理异常: " + e.getMessage());
        }
    }

    private static String getHomePage() {
        return "<!DOCTYPE html>" +
                "<html>" +
                "<head>" +
                "<title>我的主页</title>" +
                "<style>" +
                "body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background-color: #f7f7f7; margin: 0; padding: 0; color: #333; }" +
                "header { background-color: #4CAF50; color: white; padding: 20px 0; text-align: center; font-size: 24px; }" +
                "nav ul { list-style: none; padding: 0; text-align: center; background-color: #333; margin: 0; }" +
                "nav ul li { display: inline; margin: 0; }" +
                "nav ul li a { color: white; text-decoration: none; padding: 14px 20px; display: inline-block; }" +
                "nav ul li a:hover { background-color: #4CAF50; }" +
                "main { padding: 20px; text-align: center; }" +
                "footer { background-color: #333; color: white; text-align: center; padding: 10px 0; position: relative; bottom: 0; width: 100%; margin-top: 20px; }" +
                "</style>" +
                "</head>" +
                "<body>" +
                "<header><h1>欢迎来到我的主页!</h1></header>" +
                "<nav>" +
                "<ul>" +
                "<li><a href='/about'>关于我们</a></li>" +
                "<li><a href='/contact'>联系我们</a></li>" +
                "</ul>" +
                "</nav>" +
                "<main>" +
                "<p>这是一个简单的 HTTP 服务器示例。</p>" +
                "</main>" +
                "<footer>" +
                "<p>&copy; 2024 我的 HTTP 服务器</p>" +
                "</footer>" +
                "</body>" +
                "</html>";
    }
}
