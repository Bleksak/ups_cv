import java.net.Socket;
import java.io.OutputStream;
import java.io.InputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.lang.Thread;
import java.util.Scanner;

class Main {
public static void main(String[] args) throws IOException {
	Socket s;
	try {
		s = new Socket("127.0.0.1", 2000);
	} catch(IOException ex) {
		System.out.println("Cannot connect to the server");
		return;
	}
	OutputStream os = s.getOutputStream();
	InputStream is = s.getInputStream();

	Receiver recv = new Receiver(is);
	recv.start();
	Scanner sc = new Scanner(System.in);

	while(recv.running) {
		String str = sc.nextLine();
		os.write(str.getBytes(StandardCharsets.UTF_8));
	}
}
}

class Receiver extends Thread {
	boolean running = true;
	InputStream is;

	Receiver(InputStream is) {
		this.is = is;
	}

	public void run() {
		while(true) {
			try {
				int av = is.available();
				byte[] bytes = new byte[av + 1024];
				int read = is.read(bytes, 0, av+1024);
				if(read <= 0) {
					System.out.println("Server shutdown");
					running = false;
					break;
				}
				String s = new String(bytes, StandardCharsets.UTF_8);
				System.out.println(s);
			} catch(IOException ex) {
				break;
			}
		}
	}
}

