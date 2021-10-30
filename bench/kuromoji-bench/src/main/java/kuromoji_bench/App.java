package kuromoji_bench;

import com.atilika.kuromoji.ipadic.Token;
import com.atilika.kuromoji.ipadic.Tokenizer;
import java.util.List;
import java.util.ArrayList;
import java.util.Scanner;
import java.time.Instant;
import java.time.Duration;

public class App {
    public static void main(String[] args) {
        Tokenizer tokenizer = new Tokenizer();
        Scanner input = new Scanner(System.in, "utf-8");
        List<String> lines = new ArrayList<String>();
        while (input.hasNext()) {
            lines.add(input.nextLine());
        }
        Instant start = Instant.now();
        int n_words = 0;
        for (String line : lines) {
            List<Token> tokens = tokenizer.tokenize(line);
            n_words += tokens.size();
        }
        Instant finish = Instant.now();
        double timeElapsed = (double) Duration.between(start, finish).toNanos() / 1000000000;
        System.out.println("Elapsed-kuromoji: " + timeElapsed + " [sec]");
        System.out.println(n_words);
    }
}
